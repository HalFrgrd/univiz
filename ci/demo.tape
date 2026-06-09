# Settings
Output univiz-demo.gif
Output univiz-demo.svg
Set FontSize 22
Set Width 1400
Set Height 800
Set Theme "Dracula"
Set Padding 40
Set TypingSpeed 50ms
Set Shell bash --norc
Env PS1 "\[\e[32m\]\u@\h\[\e[0m\]:\[\e[35m\]\w\[\e[0m\]$ "

Sleep 100ms
Type 'univiz "hi 👋"'
Sleep 1s
Enter
Sleep 3s

Type 'univiz "café 😀🎉"'
Sleep 1s
Enter
Sleep 3s

Type 'univiz "café 😀🎉" --json | less'
Enter
Sleep 1200ms
Type d
Sleep 1200ms
Type d
Sleep 1200ms
Type d
Sleep 1200ms
Type d
Sleep 1200ms
