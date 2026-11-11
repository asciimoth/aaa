# Render with `just vhs`. Keep recording output separate from `just demo-gif`.
Output target/vhs.gif

Require aaa
Require zpg
Require lolcat
Require nitch

Set Shell "bash"
Set FontFamily "Fira Code"
# At 13 px, Fira Code progress bar segments join in the VHS canvas renderer.
Set FontSize 13
Set LetterSpacing 0
Set Width 1000
Set Height 600
Set Padding 20
Set Framerate 30
Set TypingSpeed 25ms
Set CursorBlink false
# Start the GIF with the rainbow logo beside the system information.
Set LoopOffset 95%

Env PS1 "$ "
Env UNICODE_PROGRESS_BAR "true"
Env AAA_DEMO_DELAY "0.15"

# Keep all generated art in Cargo's ignored output directory.
Hide
Type "mkdir -p target/vhs"
Enter
Sleep 300ms
Type@1ms "clear"
Enter
Sleep 300ms
Show

# Browse the built-in art, then play one animation and two side by side.
Type "aaa list"
Sleep 500ms
Enter
Sleep 2s
Hide
Type@1ms "clear"
Enter
Sleep 300ms
Show
Sleep 300ms
Type "aaa play distro_nixos_big --loop false"
Sleep 500ms
Enter
Sleep 500ms
Wait+Line@30s /^\$$/
Sleep 1s
Hide
Type@1ms "clear"
Enter
Sleep 300ms
Show
Sleep 300ms
Type "aaa play distro_nixos_big --secondary dna --loop false"
Sleep 500ms
Enter
Sleep 500ms
Wait+Line@30s /^\$$/
Sleep 1s

# Export the same animation as an asciinema recording.
Hide
Type@1ms "clear"
Enter
Sleep 300ms
Show
Type "zpg aaa convert distro_nixos_big to-cast > target/vhs/nixos.cast"
Sleep 500ms
Enter
Sleep 500ms
Wait+Line@30s /^\$$/
Sleep 1s

# Like `just demo-edit`: add each frame number, then play the result.
Hide
Type@1ms "clear"
Enter
Sleep 300ms
Show
Type "zpg aaa edit distro_nixos_big filter --jobs 4 ansi ./examples/overlay_filter.sh |"
Enter
Type "  aaa play --loop false"
Sleep 500ms
Enter
Sleep 500ms
Wait+Line@2m /^\$$/
Sleep 1s

# Like `just demo-filter`: show four workers and their child progress reports.
Hide
Type@1ms "clear"
Enter
Sleep 300ms
Show
Type "zpg aaa edit distro_nixos_big filter --jobs 4 ansi ./examples/progress_filter.sh > target/vhs/overlay.3a"
Sleep 500ms
Enter
Sleep 500ms
Wait+Line@2m /^\$$/
Sleep 1s

# Like `just demo-gif`: show parallel rendering and sequential GIF encoding.
Hide
Type@1ms "clear"
Enter
Sleep 300ms
Show
Type "zpg aaa convert distro_nixos_big to-gif --jobs 4 --font-size 12 > target/vhs/nixos.gif"
Sleep 500ms
Enter
Sleep 500ms
Wait+Line@5m /^\$$/
Sleep 1s

# Apply the rainbow filter from the old asciinema demo.
Hide
Type@1ms "clear"
Enter
Sleep 300ms
Show
Type "zpg aaa edit distro_nixos_big filter --jobs 4 ansi ./examples/lolcat_filter.sh > target/vhs/rainbow.3a"
Sleep 500ms
Enter
Sleep 500ms
Wait+Line@2m /^\$$/
Sleep 1s
Hide
Type@1ms "clear"
Enter
Sleep 300ms
Show
Type "aaa play target/vhs/rainbow.3a --loop false"
Sleep 500ms
Enter
Sleep 500ms
Wait+Line@30s /^\$$/
Sleep 1s

# Finish with system information beside the animated rainbow logo.
Hide
Type@1ms "clear"
Enter
Sleep 300ms
Show
Type "aaa fetch --art target/vhs/rainbow.3a --loop false -- nitch"
Sleep 500ms
Enter
Sleep 500ms
Wait+Line@30s /^\$$/
Sleep 2s
