# aaa
aaa is a cli tool to read, write, edit, generate, play, and convert
[animated ASCII art](https://github.com/DomesticMoth/3a)
to other formats (SVG, PNG, GIF, WebP, MP4, asciinema, dur, etc).

[![asciicast](https://asciinema.org/a/ZtzhhTOmVWCAmrfz.svg)](https://asciinema.org/a/ZtzhhTOmVWCAmrfz)

You can find more art supported by aaa [here](https://openascii.moth.contact).

## Installation
### Binary
If none of the options listed below work for you, you can always just download
an executable for your platform from the [releases page](https://github.com/asciimoth/aaa/releases/latest).
### Nix
Nix users can install aaa with flake:
```sh
# Install to sys profile
nix profile add github:asciimoth/aaa
# Remove from sys profile
nix profile remove aaa

# Add to temporal shell
nix shell github:asciimoth/aaa
```

aaa is also [available in nixpkgs](https://search.nixos.org/packages?channel=unstable&query=aaa&show=aaa) but may be outdated.
### Deb/Rpm
You can download deb/rpm packages from [releases page](https://github.com/asciimoth/aaa/releases/latest)
or use my [deb/rpm repo](https://repo.moth.contact/).

#### Fedora

The RPM requires FFmpeg with the `libx264` encoder for MP4 output and the
`libwebp` encoder for WebP output. Fedora's `ffmpeg-free` package does not
include `libx264`. Enable [RPM Fusion Free](https://rpmfusion.org/Configuration)
before you install `aaa`:

```sh
sudo dnf install \
  "https://mirrors.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm"
sudo curl -fsSLo /etc/yum.repos.d/ghrepo.repo \
  https://repo.moth.contact/rpm.repo
sudo dnf install --allowerasing aaa
```

The `--allowerasing` option lets DNF replace conflicting Fedora FFmpeg
packages. Check the proposed changes before you confirm the transaction.
For a downloaded RPM, replace `aaa` in the last command with its path,
such as `./aaa_2.1.0_linux_amd64.rpm`.

### Arch
Arch users can install [aaa-bin](https://aur.archlinux.org/packages/aaa-bin)
from AUR or download `.pkg` files from [releases page](https://github.com/asciimoth/aaa/releases/latest).
### Cargo
You can also install it with cargo:
```sh
cargo install aaa
```

## Usage
```help
Usage: aaa <COMMAND>

Commands:
  list             List builtin art
  gen              Generate new art
  play             Play art (or two side by side) in terminal
  fetch            Show system info side by side with animated logo. (by default requires one of fetch tools to be installed: neofetch | fastfetch | screenfetch | nitch | profetch | leaf | fetch-scm)
  preview          Show art preview
  edit             Editing subcommands
  convert          Format conversion subcommands
  from-text        Constructs art from plain text with ANSI color escape codes
  from-tiled-text  Construct art from colorless plain-text frames arranged in a grid
  completions      Generate shell completions to stdout (shell: bash|zsh|fish|powershell|elvish)
  help             Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```help
Format conversion subcommands

Usage: aaa convert [OPTIONS] [FILE] <COMMAND>

Commands:
  to-frames  Print art as a blank line separated sequence of frames with ANSI colors codes
  to-cast    Convert art to asciicast v2 format
  to-dur     Convert art to durformat (durdraw's ascii art format)
  to-json    Convert art to json document
  to-ttyrec  Convert art to ttyrec format
  to-png     Convert art to png image
  to-gif     Convert art to gif animation
  to-webp    Convert art to webp animation (ffmpeg cli required)
  to-mp4     Convert art to mp4 video (ffmpeg cli required)
  to-svg     Convert art to svg animation
  to3a       Print art back in 3a format
  help       Print this message or the help of the given subcommand(s)

Arguments:
  [FILE]  art file path (alternatively pipe art to stdin)

Options:
  -n, --no-colors  disable colors
  -h, --help       Print help
```

```help

Editing subcommands

Usage: aaa edit [FILE] <COMMAND>

Commands:
  set             Set cell (text, color)
  strip           Strip comments from art
  color-map       Search or add new color mapping. Mapped color prints to stderr
  color-unmap     Remove color mapping
  palette-reset   Reset palette
  color-force     Force enable/disable colors
  crop            Crop art
  delay-set       Set delay for whole art or for specific frame
  delay-reset     Reset all art delays to default (50 milis)
  fill            Fill all frames or specific one with text and color
  fill-area       Fill area in all frames or specific one with text and color
  clean           Fill all frames or specific one with default text, color
  pin-text        Pin text channel
  pin-color       Pin color channel
  print           Print text to art
  print-ansi      Print text with ansi color codes to art
  filter          Filter art with arbitrary program
  frame-remove    Remove frame
  frame-dup       Duplicate frame
  frame-ensure    Ensures a frame exists at the given index, creating new frames if necessary
  frames-slice    Remove all frames out of inclusive subrange
  frames-swap     Swap two frames
  frames-reverse  Reverse frames
  frame-dedup     Deduplicate frames
  frame-rf        Rotate frames forth
  frame-rb        Rotate frames back
  title           Set art title
  authors         Set authors
  origs           Set orig authors
  src             Set src
  editor          Set editor
  license         Set license
  loop            Set loop
  preview         Set preview frame
  tag-add         Add tag to art
  tag-rm          Remove tag from art
  tags-drop       Drop all tags
  help            Print this message or the help of the given subcommand(s)

Arguments:
  [FILE]  art file path (alternatively pipe art to stdin)

Options:
  -h, --help  Print help
```

## Parallel frame processing

`edit ... filter`, `convert ... to-gif`, `convert ... to-mp4`, and
`convert ... to-webp` process frames in parallel. By default, they use the
available CPU threads. Use `--jobs N` (or `-j N`) to set the number of workers;
`--jobs 0` selects the default and `--jobs 1` processes frames sequentially.

```sh
aaa edit distro_nixos_big filter --jobs 4 ansi ./examples/overlay_filter.sh | aaa play
aaa convert distro_nixos_big to-gif --jobs 4 > animation.gif
```

Frames can finish processing in any order. Output frames, delays, and palette
updates stay in their original order. Progress shows the number of completed
frames. Processing uses batches of at most one frame per worker to limit the
number of results held in memory. Filter input is prepared before processing.

Use `--jobs 1` for filter scripts that depend on execution order or share mutable
state, such as a file that each invocation updates. Each process receives its
own `AAA_FRAME` and `AAA_DELAY` values. Filter stderr is shared, so messages from
different processes can appear together. On failure, active work finishes before
the command returns an error; no new batch starts.

GIF encoding and final MP4/WebP encoding stay in sequence. Other edit commands
continue to use the existing `rs3a` operations.

## Progress reports

`aaa` uses [zprs](https://github.com/asciimoth/zprs) to report work through the
Zig progress protocol. Run commands with [zpg](https://github.com/asciimoth/zpg)
to see the task tree. Progress does not change the art or binary data on stdout:

```sh
zpg aaa edit distro_nixos_big filter --jobs 4 ansi ./examples/overlay_filter.sh | aaa play
zpg aaa convert distro_nixos_big to-gif --jobs 4 > animation.gif
```

The reports cover both parallel and sequential work:

| Work | Report |
| --- | --- |
| Filters | Completed frames, active workers, and each child process's reports |
| GIF, MP4, WebP | Completed rendered frames, rows in each active frame, and encoding |
| PNG | Rows rendered, encoding, and output |
| Roller effects and area fills | Completed steps or rows |
| Loading, parsing, generation, other edits and conversions, output, playback preparation | Named activities with unknown totals |
| Fetch commands and FFmpeg | Child activity, plus nested reports if the child supports the protocol |

Unknown totals show activity rather than an estimated percentage. Short tasks
can finish between reports. Playback itself supplies its own visual feedback.
Without a usable progress channel, reports are disabled and frame processing
keeps the existing single-line stderr counter. Under `zpg`, that counter is
suppressed when reporting is active. On platforms without pipe reporting, the
commands still work.

Each external command is started with `Node::spawn`, gets a private progress
pipe, and is waited for before its task ends. Threads share task handles. A
script must also keep concurrent reporters on separate pipes. The overlay
example lets only the last process in its pipeline report; the other processes
receive an empty `ZIG_PROGRESS`. The progress-filter demo reports its own stages
and disables reporting inside its pipeline.

### Demos

Enter `nix develop` first. On Linux, it includes the pinned `zpg` package and
FFmpeg. The upstream `zpg` flake currently supplies Linux packages only.
Each demo builds the local CLI, then runs it through `zpg`. Filter scripts also
use the local CLI from `target/debug`.

| Command | Result |
| --- | --- |
| `just demo-edit` | Filter with four workers, then play once |
| `just demo-filter` | Show four reporting scripts; save `target/demo-filter.3a` |
| `just demo-filter 1` | Show the same filter work sequentially |
| `just demo-gif` | Render with four workers; save `target/demo.gif` |
| `just demo-mp4` | Render with four workers and run FFmpeg; save `target/demo.mp4` |
| `just demo-webp` | Render with four workers and run FFmpeg; save `target/demo.webp` |
| `just demo-png` | Render one PNG on one thread; save `target/demo.png` |

For filter and animation demos, the first argument selects the worker count.
For demos that save a file, the final argument selects its path. Existing output
files are replaced. The progress-filter script has short delays so its stages
remain visible; set `AAA_DEMO_DELAY` to change the delay in seconds.

Run `just test-progress` to validate reports, nested child processes, failure
cleanup, and output bytes with `zpg --test`. These integration tests require
`zpg` and are separate from `just test`. To inspect reports directly:

```sh
zpg --test --test-output target/progress.log target/debug/aaa \
  edit distro_nixos_big filter --jobs 4 text cat > target/filtered.3a
```

## Related links
- [3a format spec](https://github.com/asciimoth/3a)
- [rs3a](https://github.com/asciimoth/rs3a) - 3a library on which aaa is based
- [py3a](https://github.com/asciimoth/py3a)
- [go3a](https://github.com/asciimoth/go3a)
