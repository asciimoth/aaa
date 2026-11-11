//! Run with `just test-progress` inside `nix develop` (requires zpg).
#![cfg(unix)]

use std::{
    fs,
    io::Write,
    os::unix::fs::PermissionsExt,
    path::Path,
    process::{Command, Output, Stdio},
    thread,
    time::Duration,
};

use rs3a::{Art, Cell};

const AAA: &str = env!("CARGO_BIN_EXE_aaa");

fn fixture(dir: &Path, frames: usize) -> std::path::PathBuf {
    let mut art = Art::new(frames, 3, 1, Cell::default());
    art.set_loop_key(false);
    art.set_global_delay(1);
    for frame in 0..frames {
        art.print_ansi(frame, 0, 0, "ABC");
    }
    let path = dir.join(format!("input-{frames}.3a"));
    fs::write(&path, art.to_string()).unwrap();
    path
}

fn supervised(log: &Path) -> Command {
    let mut command = Command::new("zpg");
    command
        .env_remove("ZIG_PROGRESS")
        .args(["--test", "--test-output"])
        .arg(log)
        .arg(AAA);
    command
}

fn success(output: &Output) {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn valid_log(path: &Path) -> String {
    let log = fs::read_to_string(path).unwrap();
    assert!(log.contains("snapshot="), "{log}");
    assert!(log.contains("progress pipe closed"), "{log}");
    for invalid in ["no progress received", "invalid", "truncated", "incomplete"] {
        assert!(!log.contains(invalid), "{log}");
    }
    log
}

fn worker(path: &Path) {
    fs::write(
        path,
        r#"
set -eu
zpg --report 0 2 --name "worker $AAA_FRAME"
sleep 0.25
zpg --report 1 2 --name "worker $AAA_FRAME"
sleep 0.25
cat
zpg --report 2 2 --name "worker $AAA_FRAME"
"#,
    )
    .unwrap();
}

#[test]
#[ignore = "requires zpg; run just test-progress"]
fn filter_reports_parallel_and_sequential_work_without_changing_output() {
    let dir = tempfile::tempdir().unwrap();
    let art = fixture(dir.path(), 4);
    let script = dir.path().join("worker.sh");
    worker(&script);
    let baseline = Command::new(AAA)
        .arg("edit")
        .arg(&art)
        .args(["filter", "text", "cat"])
        .output()
        .unwrap();
    success(&baseline);
    for jobs in ["1", "2"] {
        let log = dir.path().join(format!("jobs-{jobs}.log"));
        let output = supervised(&log)
            .arg("edit")
            .arg(&art)
            .args(["filter", "--jobs", jobs, "text", "--", "sh"])
            .arg(&script)
            .output()
            .unwrap();
        success(&output);
        assert_eq!(output.stdout, baseline.stdout);
        assert!(
            output.stderr.is_empty(),
            "fallback progress must be suppressed"
        );
        let log = valid_log(&log);
        assert!(log.contains("name=\"worker 0\""), "{log}");
        let counts: Vec<usize> = log
            .lines()
            .filter(|line| line.ends_with("name=\"filtering\""))
            .map(|line| {
                line.split("completed=")
                    .nth(1)
                    .unwrap()
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap()
            })
            .collect();
        assert!(counts.iter().any(|&count| count > 0), "{log}");
        assert!(
            counts.iter().all(|&count| count <= 4),
            "double-counted tasks: {log}"
        );
        assert!(counts.windows(2).all(|pair| pair[0] <= pair[1]), "{log}");
        if jobs == "2" {
            assert!(
                log.split("zpg-test: snapshot=")
                    .any(|snapshot| snapshot.contains("name=\"worker 0\"")
                        && snapshot.contains("name=\"worker 1\"")),
                "{log}"
            );
        }
    }
}

#[test]
#[ignore = "requires zpg; run just test-progress"]
fn nested_aaa_children_have_separate_progress_trees() {
    let dir = tempfile::tempdir().unwrap();
    let outer = fixture(dir.path(), 1);
    let inner = fixture(dir.path(), 4);
    let script = dir.path().join("worker.sh");
    worker(&script);
    let log = dir.path().join("nested.log");
    let output = supervised(&log)
        .arg("edit")
        .arg(outer)
        .args([
            "filter",
            "text",
            "--",
            "sh",
            "-c",
            "cat >/dev/null; exec \"$@\"",
            "nested",
            AAA,
            "edit",
        ])
        .arg(inner)
        .args(["filter", "--jobs", "2", "text", "--", "sh"])
        .arg(script)
        .output()
        .unwrap();
    success(&output);
    let log = valid_log(&log);
    assert!(
        log.split("zpg-test: snapshot=").any(|snapshot| snapshot
            .matches("name=\"aaa edit\"")
            .count()
            == 2
            && snapshot.contains("name=\"worker 0\"")),
        "{log}"
    );
}

#[test]
#[ignore = "requires zpg; run just test-progress"]
fn serial_commands_report_waiting_for_input_and_preserve_output() {
    let dir = tempfile::tempdir().unwrap();
    let log = dir.path().join("text.log");
    let mut child = supervised(&log)
        .arg("from-text")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    // Hold stdin open long enough for the activity report to be sampled.
    thread::sleep(Duration::from_millis(450));
    child.stdin.take().unwrap().write_all(b"ABC\n").unwrap();
    let output = child.wait_with_output().unwrap();
    success(&output);
    let log = valid_log(&log);
    assert!(log.contains("name=\"read text\""), "{log}");
    let art: Art = String::from_utf8(output.stdout).unwrap().parse().unwrap();
    assert_eq!(art.get(0, 0, 0, Cell::default()).text.to_string(), "A");
}

#[test]
#[ignore = "requires zpg; run just test-progress"]
fn filter_failures_close_progress_pipes_and_keep_errors() {
    let dir = tempfile::tempdir().unwrap();
    let art = fixture(dir.path(), 4);
    for (name, args, expected) in [
        (
            "missing",
            vec!["/aaa-missing-progress-test-command"],
            "No such file",
        ),
        (
            "exit",
            vec![
                "sh",
                "-c",
                "cat >/dev/null; zpg --report 1 2; sleep 0.3; exit 7",
            ],
            "exit code 7",
        ),
    ] {
        let log = dir.path().join(format!("{name}.log"));
        let output = supervised(&log)
            .arg("edit")
            .arg(&art)
            .args(["filter", "--jobs", "2", "text", "--"])
            .args(args)
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains(expected));
        valid_log(&log);
    }
}

#[test]
#[ignore = "requires zpg; run just test-progress"]
fn gif_stdout_stays_binary_and_unchanged() {
    let dir = tempfile::tempdir().unwrap();
    let art = fixture(dir.path(), 4);
    let args = ["to-gif", "--jobs", "2", "--font-size", "8"];
    let plain = Command::new(AAA)
        .arg("convert")
        .arg(&art)
        .args(args)
        .output()
        .unwrap();
    let log = dir.path().join("gif.log");
    let reported = supervised(&log)
        .arg("convert")
        .arg(&art)
        .args(args)
        .output()
        .unwrap();
    success(&plain);
    success(&reported);
    assert_eq!(reported.stdout, plain.stdout);
    valid_log(&log);
}

#[test]
#[ignore = "requires zpg; run just test-progress"]
fn ffmpeg_and_fetch_children_report_through_private_pipes() {
    let dir = tempfile::tempdir().unwrap();
    let art = fixture(dir.path(), 4);
    let ffmpeg = dir.path().join("ffmpeg");
    fs::write(
        &ffmpeg,
        r#"#!/bin/sh
set -eu
zpg --report 0 1 --name "encoder worker"
sleep 0.4
printf encoded
zpg --report 1 1 --name "encoder worker"
"#,
    )
    .unwrap();
    fs::set_permissions(&ffmpeg, fs::Permissions::from_mode(0o755)).unwrap();
    let mut paths = vec![dir.path().to_path_buf()];
    paths.extend(std::env::split_paths(&std::env::var_os("PATH").unwrap()));
    let log = dir.path().join("ffmpeg.log");
    let output = supervised(&log)
        .arg("convert")
        .arg(&art)
        .args(["to-mp4", "--jobs", "2", "--font-size", "8"])
        .env("PATH", std::env::join_paths(paths).unwrap())
        .output()
        .unwrap();
    success(&output);
    assert_eq!(output.stdout, b"encoded");
    assert!(valid_log(&log).contains("name=\"encoder worker\""));

    let log = dir.path().join("fetch.log");
    let output = supervised(&log).args(["fetch", "--art"]).arg(&art)
        .args(["--loop", "false", "--effect", "none", "--", "sh", "-c",
            "zpg --report 0 1 --name 'fetch worker'; sleep 0.4; printf 'info\\n'; zpg --report 1 1 --name 'fetch worker'"])
        .output().unwrap();
    success(&output);
    assert!(valid_log(&log).contains("name=\"fetch worker\""));
}
