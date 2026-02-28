# aaa
Tui tool for 3a files rendering  
[Here is 3a format specification.](https://github.com/DomesticMoth/3a)  
[Here is a collection of 3a animations.](https://github.com/DomesticMoth/3a_storage)  

## Installation
### Download binaries
You can download binaries from the [github releases page](https://github.com/DomesticMoth/aaa/releases)
### With "cargo install"
```
$ cargo install aaa
```
### Manual compilation from sources
```
$ git clone https://github.com/DomesticMoth/aaa.git
$ cd aaa
$ cargo build --release
```

## Usage
```help
Usage: aaa <command> [<args>]

Animated ASCII art tool by asciimoth

Options:
  --help, help      display usage information

Commands:
  play              Play art (or two side by side) in terminal
  fetch             Show system info side by side with animated logo. (by
                    default requires one of fetch tools to be installed:
                    neofetch | fastfetch | screenfetch | nitch | profetch | leaf
                    | fetch-scm)
  preview           Show art preview
  list              List all builtin art
  strip             Strip comments from art
  edit              Editing subcommands
  generate          Generate new art
  from-text         Constructs art from plain text with ANSI color escape codes
  to-frames         Print art as a blank line separated sequence of frames with
                    ANSI colors codes.
  to-cast           Convert art to asciicast v2 format
  to-3a             Print art back in 3a format
  to-svg            Convert art to svg
  to-json           Convert art to json document
  to-dur            Convert art to durformat (durdraw's ascii art format)
  to-ttyrec         Convert art to ttyrec format
  to-png            Convert art to png
  to-gif            Convert art to gif animation
  to-webp           Convert art to webp animation (ffmpeg cli required)
  to-mp4            Convert art to mp4 video (ffmpeg cli required)
```

```help
Usage: aaa edit [<file>] <command> [<args>]

Editing subcommands

Positional Arguments:
  file              art file path (alternatively pipe art to stdin)

Options:
  --help, help      display usage information

Commands:
  set               Set cell (text, color)
  color-map         Search or add new color mapping. Mapped color prints to
                    stderr.
  color-unmap       Remove color mapping.
  color-force       Force enable/disable colors
  palette-reset     Reset palette.
  frame-rm          Remove frame
  frame-dup         Duplicate frame
  frame-ensure      Ensures a frame exists at the given index, creating new
                    frames if necessary
  frame-slice       Remove all frames out of inclusive subrange
  frame-swap        Swap two frames
  frame-rev         Reverse frames
  frame-dedup       Deduplicate frames
  frame-r-f         Rotate frames forth
  frame-r-b         Rotate frames back
  pin-text          Pin text channel
  pin-color         Pin color channel
  crop              Crop art
  fill              Fill all frames or specific one with text and color
  fill-area         Fill area in all frames or specific one with text and color
  clean             Fill all frames or specific one with default text, color
  print             Print text to art
  print-ansi        Print text with ansi color codes to art
  filter            Filter art with arbitrary program
  tag-add           Add tag to art
  tag-rm            Remove tag from art
  tags-drop         Drop all tags
  delay-set         Set delay for whole art or for specific frame
  delay-reset       Reset all art delays to default (50 milis)
  title             Set art title
  authors           Set authors
  origs             Set orig authors
  src               Set src
  editor            Set editor
  license           Set license
  loop              Set loop
  preview           Set preview frame
```
