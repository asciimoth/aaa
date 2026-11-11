use image::AnimationDecoder;
use rs3a::{Art, Cell};
use std::{
    fs,
    io::Cursor,
    path::Path,
    process::{Command, Output},
};

fn fixture(directory: &Path) -> std::path::PathBuf {
    let mut art = Art::new(6, 8, 2, Cell::default());
    for frame in 0..art.frames() {
        art.print_ansi(frame, 0, 0, &format!("\x1b[3{}m{}", frame + 1, frame));
        art.set_frame_delay(frame, 100 + frame * 10);
    }
    let path = directory.join("input.3a");
    fs::write(&path, art.to_string()).unwrap();
    path
}

fn run(path: &Path, args: &[&str], command: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_aaa"))
        .arg(command)
        .arg(path)
        .args(args)
        .output()
        .unwrap()
}

fn success(output: &Output) {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn parallel_filters_preserve_frames_palette_delays_and_environment() {
    let dir = tempfile::tempdir().unwrap();
    let path = fixture(dir.path());
    let script = dir.path().join("filter.sh");
    fs::write(
        &script,
        r#"
set -eu
cat >/dev/null
test "$AAA_FRAMES" = 6
test "$AAA_WIDTH" = 8
test "$AAA_HEIGHT" = 2
test "$AAA_COLOR" = true
test "$AAA_INPUT" = "$1"
test "$AAA_DELAY" = "$((100 + AAA_FRAME * 10))"
printf '\033[38;2;%s;10;20m%s\nrow2\n' "$((AAA_FRAME + 100))" "$AAA_FRAME"
"#,
    )
    .unwrap();
    for mode in ["text", "ansi", "frame"] {
        let invoke = |jobs| {
            run(
                &path,
                &[
                    "filter",
                    "--jobs",
                    jobs,
                    mode,
                    "--",
                    "sh",
                    script.to_str().unwrap(),
                    mode,
                ],
                "edit",
            )
        };
        let sequential = invoke("1");
        success(&sequential);
        for jobs in ["2", "3", "0", "20"] {
            let parallel = invoke(jobs);
            success(&parallel);
            assert_eq!(parallel.stdout, sequential.stdout);
            assert_eq!(
                parallel.stderr,
                b"filtering:\n\r  1/6\r  2/6\r  3/6\r  4/6\r  5/6\r  6/6\n"
            );
            let art: Art = String::from_utf8(parallel.stdout).unwrap().parse().unwrap();
            for frame in 0..6 {
                assert_eq!(
                    art.get(frame, 0, 0, Cell::default()).text.to_string(),
                    frame.to_string()
                );
                assert_eq!(art.get_frame_delay(frame), 100 + frame * 10);
                let color = art.get(frame, 0, 0, Cell::default()).color.unwrap();
                assert_eq!(
                    art.get_color_map(color).fg,
                    rs3a::Color::RGB(frame as u8 + 100, 10, 20)
                );
            }
        }
    }
}

#[test]
fn filter_errors_have_frame_context_and_no_art_output() {
    let dir = tempfile::tempdir().unwrap();
    let path = fixture(dir.path());
    for script in [
        "cat >/dev/null; exit 7",
        "cat >/dev/null; printf 'short\\n'",
    ] {
        let output = run(
            &path,
            &["filter", "--jobs", "2", "text", "--", "sh", "-c", script],
            "edit",
        );
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains("\nError: frame"), "{stderr}");
        assert!(!stderr.contains("panicked"), "{stderr}");
    }
}

#[test]
fn gif_parallel_rendering_preserves_pixels_order_and_delays() {
    let dir = tempfile::tempdir().unwrap();
    let path = fixture(dir.path());
    let invoke = |jobs| {
        run(
            &path,
            &["to-gif", "--jobs", jobs, "--font-size", "8"],
            "convert",
        )
    };
    let sequential = invoke("1");
    let parallel = invoke("3");
    success(&sequential);
    success(&parallel);
    assert_eq!(parallel.stdout, sequential.stdout);
    let frames = image::codecs::gif::GifDecoder::new(Cursor::new(parallel.stdout))
        .unwrap()
        .into_frames()
        .collect_frames()
        .unwrap();
    assert_eq!(frames.len(), 6);
    for (index, frame) in frames.iter().enumerate() {
        assert_eq!(
            std::time::Duration::from(frame.delay()).as_millis(),
            100 + index as u128 * 10
        );
    }
    assert_ne!(frames[0].buffer(), frames[1].buffer());
}

#[cfg(unix)]
#[test]
fn ffmpeg_receives_ordered_frames_and_delays_after_parallel_rendering() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    let path = fixture(dir.path());
    let ffmpeg = dir.path().join("ffmpeg");
    // Capture the encoder inputs without requiring an installed FFmpeg version.
    fs::write(
        &ffmpeg,
        r#"#!/bin/sh
set -eu
cp frame_*.png "$CHECK_DIR/"
for arg do last="$arg"; done
if [ "$last" = pipe:1 ]; then
    cat list.txt
else
    cat list.txt > "$last"
fi
"#,
    )
    .unwrap();
    fs::set_permissions(&ffmpeg, fs::Permissions::from_mode(0o755)).unwrap();
    let mut paths = vec![dir.path().to_path_buf()];
    paths.extend(std::env::split_paths(&std::env::var_os("PATH").unwrap()));
    let search_path = std::env::join_paths(paths).unwrap();
    let expected = (0..6)
        .map(|f| {
            format!(
                "file 'frame_{f:010}.png'\nduration {:.3}\n",
                (100 + f * 10) as f64 / 1000.0
            )
        })
        .collect::<String>()
        + "file 'frame_0000000005.png'\n";
    for format in ["to-mp4", "to-webp"] {
        let mut images = Vec::new();
        for jobs in ["1", "3"] {
            let capture = dir.path().join(format!("{format}-{jobs}"));
            fs::create_dir(&capture).unwrap();
            let output = Command::new(env!("CARGO_BIN_EXE_aaa"))
                .arg("convert")
                .arg(&path)
                .args([format, "--jobs", jobs, "--font-size", "8"])
                .env("PATH", &search_path)
                .env("CHECK_DIR", &capture)
                .output()
                .unwrap();
            success(&output);
            assert_eq!(String::from_utf8(output.stdout).unwrap(), expected);
            images.push(
                (0..6)
                    .map(|f| {
                        image::open(capture.join(format!("frame_{f:010}.png")))
                            .unwrap()
                            .into_rgba8()
                    })
                    .collect::<Vec<_>>(),
            );
        }
        assert_eq!(images[0], images[1]);
        assert_ne!(images[0][0], images[0][1]);
    }
}
