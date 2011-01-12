source ~/.profile

setxkbmap -layout us -variant dvorak -option lv3:rwin_switch,ctrl:nocaps,grp_led:caps,keypad:oss,kpdl:kposs,misc:typo,compose:menu&
xmodmap ~/.Xmodmap&

exec ck-launch-session startkde
