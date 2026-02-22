# aaa
Tui tool for 3a files rendering  
[Here is 3a format specification.](https://github.com/DomesticMoth/3a)  
[Here is a collection of 3a animations.](https://github.com/DomesticMoth/3a_storage)  
Example:  
[![example](https://raw.githubusercontent.com/asciimoth/aaa/main/exaple.svg)](https://asciinema.org/a/599989)
## Usage
```
aaa 1.1.1

DomesticMoth

CLI tool for work with 3a files

USAGE:
    aaa [OPTIONS] [FILE]

ARGS:
    <FILE>    

OPTIONS:
    -c, --colors <COLORS>        Override colors param
        --colortable             Show table of available art colors
    -d, --delay <DELAY>          Override delay param
        --datacols <DATACOLS>    Override datacols param
        --demo                   Show demo animation
        --get-clear-body         Return source code body, stripped of comments and no display
                                 characters
        --get-escape-comments    Return source code without comments
        --get-header             Return source code header
        --get-param-audio        Get parameter audio value
        --get-param-author       Get parameter author value
        --get-param-colors       Get parameter colors value
        --get-param-datacols     Get parameter watacols value
        --get-param-delay        Get parameter delay value
        --get-param-height       Get parameter height value
        --get-param-loop         Get parameter loop value
        --get-param-preview      Get parameter preview value
        --get-param-title        Get parameter title value
        --get-param-utf8         Get parameter utf8 value
        --get-param-width        Get parameter width value
        --get-pretify-body       Return pretifyed source code body
    -h, --help                   Print help information
    -l, --looped <LOOPED>        Override loop param
    -p, --print                  Print source code instead of rendering it
        --preview <PREVIEW>      Override preview param
        --to-plain-text          Render to plain text instead of animation (render only preview
                                 frame)
    -V, --version                Print version information
    -x <X>                       Left up corner x position
    -y <Y>                       Left up corner y position
```
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
## TODO
- [X] playing
    - [ ] setting terminal title to art's title line
    - [ ] show art metadata under animation
- conversion
    - [X] ansi frames
    - [X] svg
    - [X] asciicast
    - [X] json
    - [ ] to png/jpg/bmp/etc
    - [ ] to gif
    - [ ] to video?
    - [ ] to webp??
- editing 
    - [X] set cell
    - colors
        - [X] search or add color mapping
        - [X] remove color mapping
            - replace unmapped color in art with `_`
        - [X] remove whole palette
        - [X] force disable/enable colors
    - frames
        - [X] remove frame
        - [X] dup frame
        - [X] make sure frame exists
        - [X] slice
        - [X] swap two frames
        - [X] reverse frames
        - [X] dedup frames
        - [X] rotate back / forth
    - pinning
        - [X] text
        - [X] colors
    - [X] crop (shift + resize)
    - [X] fill (whole art / single frame)
    - [X] fill area (whole art / single frame)
    - [X] clean (whole art / single frame)
    - [X] print text
    - [X] filters
    - tags
        - [X] add tag
        - [X] remove tag
        - [X] remove all tags
    - delay
        - [X] set global/per-frame delay
        - [X] unset delays to default
    - header
        - [ ] set title
        - [ ] add author
        - [ ] remove author
        - [ ] remove all authors
        - [ ] add orig author
        - [ ] remove orig author
        - [ ] remove all orig authors
        - [ ] set src
        - [ ] set editor
        - [ ] set license
        - [ ] set loop
        - [ ] set preview
- [ ] art generation
- [ ] fetch
- [ ] ability to get 3a files by URL
- [X] comments stripping
- [ ] art optimisation
- [ ] shell autocompletion
- [ ] MIME DB files
- [ ] Man pages
- usage examples
    - [ ] complex jq queries
    - [ ] editing

