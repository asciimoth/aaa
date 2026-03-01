# aaa
aaa is a cli tool to read, write, edit, generate, play, and convert
[animated ASCII art](https://github.com/DomesticMoth/3a)
to other formats (SVG, PNG, GIF, WebP, MP4, asciinema, dur, etc).

[![asciicast](https://asciinema.org/a/ZtzhhTOmVWCAmrfz.svg)](https://asciinema.org/a/ZtzhhTOmVWCAmrfz)

You can find more art supported by aaa [here](https://github.com/DomesticMoth/3a_storage).

## Usage
```help
Usage: aaa <COMMAND>

Commands:
  list         List builtin art
  gen          Generate new art
  play         Play art (or two side by side) in terminal
  fetch        Show system info side by side with animated logo. (by default requires one of fetch tools to be installed: neofetch | fastfetch | screenfetch | nitch | profetch | leaf | fetch-scm)
  preview      Show art preview
  edit         Editing subcommands
  convert      Format conversion subcommands
  from-text    Constructs art from plain text with ANSI color escape codes
  completions  Generate shell completions to stdout (shell: bash|zsh|fish|powershell|elvish)
  help         Print this message or the help of the given subcommand(s)

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

