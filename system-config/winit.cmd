start /b control desk.cpl
rundll32 shell32.dll,Control_RunDLL input.dll
rundll32 sysdm.cpl,EditEnvironmentVariables
start /b caps2ctrl
start /b regedit \unikey.reg
start /b unikeynt
start /b emacs\bin\runemacs
start /b firefox\firefox -profile \fxprofile -no-remote
start /b foobar2000\foobar2000
