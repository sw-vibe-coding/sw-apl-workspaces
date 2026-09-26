# vhs tape: aplterm, the 2741 terminal, dialled into a local sw-apl
# service that has this repository as library 2, drawing with PLOT.
# Render with `just tape`, which starts the service, runs vhs, and
# converts the gif to an animated webp.
#
# vhs turns a typed left arrow into the cursor key, so the tape
# assigns nothing; aplterm runs --literal so the glyphs vhs types
# arrive as themselves rather than through the 2741 keymap.
Output images/plot.gif

Set Shell "bash"
Set FontSize 22
Set FontFamily "Menlo"
Set Width 1000
Set Height 760
Set Padding 20
Set Theme "Catppuccin Mocha"
Set TypingSpeed 100ms

Hide
Type "clear; ${APLTERM:-../sw-apl/target/release/aplterm} --literal --connect ${APLTERM_ADDR:-127.0.0.1:2741}" Enter
Sleep 800ms
Show

Sleep 1s
Type ")LOAD 2 PLOT" Sleep 800ms Enter Sleep 1.5s
Type "DESCRIBE" Sleep 800ms Enter Sleep 4s
Type "PLOT 1○(⍳60)÷10" Sleep 1s Enter Sleep 5s
Type "PLOT (⍳15)*2" Sleep 1s Enter Sleep 4s
Type "BAR 3 1 4 1 5 9 2 6" Sleep 1s Enter Sleep 4s
Type ")OFF" Sleep 1.5s Enter
Hide
Sleep 500ms
