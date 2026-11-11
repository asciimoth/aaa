use std::{
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use rs3a::{Art, Cell, TiledTextOptions};

fn strip_trailing_whitespace(text: &str) -> String {
    text.split('\n')
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
}

fn assert_same_text(actual: &Art, expected: &Art, path: &Path) {
    assert_eq!(actual.frames(), expected.frames(), "{}", path.display());
    assert_eq!(actual.width(), expected.width(), "{}", path.display());
    assert_eq!(actual.height(), expected.height(), "{}", path.display());
    assert!(!actual.color(), "{}", path.display());

    for frame in 0..expected.frames() {
        for row in 0..expected.height() {
            for column in 0..expected.width() {
                assert_eq!(
                    actual.get(frame, column, row, Cell::default()).text,
                    expected.get(frame, column, row, Cell::default()).text,
                    "different text in {}, frame {frame}, column {column}, row {row}",
                    path.display()
                );
            }
        }
    }
}

#[test]
fn infers_omitted_tile_size_and_gap_hints() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_aaa"))
        .args(["from-tiled-text", "2", "1"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"A B\nC D")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());

    let art: Art = String::from_utf8(output.stdout).unwrap().parse().unwrap();
    assert_eq!((art.frames(), art.width(), art.height()), (2, 1, 2));
    assert_eq!(art.get(0, 0, 0, Cell::default()).text.to_string(), "A");
    assert_eq!(art.get(0, 0, 1, Cell::default()).text.to_string(), "C");
    assert_eq!(art.get(1, 0, 0, Cell::default()).text.to_string(), "B");
    assert_eq!(art.get(1, 0, 1, Cell::default()).text.to_string(), "D");
}

#[test]
fn converts_trimmed_colorless_tiled_versions_of_builtin_art() {
    let art_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("art");
    let mut paths = fs::read_dir(&art_dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "3a"))
        .collect::<Vec<_>>();
    paths.sort();
    assert!(!paths.is_empty());

    for path in paths {
        let source_art: Art = fs::read_to_string(&path).unwrap().parse().unwrap();
        let columns = source_art.frames().min(3).max(1);
        let rows = source_art.frames().div_ceil(columns).max(1);
        let options = TiledTextOptions::new(columns, rows)
            .with_cell_size(source_art.width(), source_art.height())
            .with_gaps(2, 1);
        let tiled_text = source_art.to_tiled_text(options).unwrap();
        let input = strip_trailing_whitespace(&tiled_text);
        assert!(!input.contains('\u{1b}'), "{}", path.display());
        assert!(
            input
                .split('\n')
                .all(|line| line.chars().last().is_none_or(|ch| !ch.is_whitespace())),
            "{}",
            path.display()
        );
        let expected = Art::from_tiled_text(&input, options).unwrap();

        let mut child = Command::new(env!("CARGO_BIN_EXE_aaa"))
            .args([
                "from-tiled-text",
                &columns.to_string(),
                &rows.to_string(),
                "--tile-width",
                &source_art.width().to_string(),
                "--tile-height",
                &source_art.height().to_string(),
                "--horizontal-gap",
                "2",
                "--vertical-gap",
                "1",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(
            output.status.success(),
            "conversion failed for {}",
            path.display()
        );

        let actual: Art = String::from_utf8(output.stdout).unwrap().parse().unwrap();
        assert_same_text(&actual, &expected, &path);
    }
}
