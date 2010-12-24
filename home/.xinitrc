source /etc/X11/xinit/xinitrc.d/40-libcanberra-gtk-module
source ~/.profile

eval `ssh-agent`
#emacs --daemon&

setxkbmap -layout us -variant dvorak -option lv3:rwin_switch,ctrl:nocaps,grp_led:caps,keypad:oss,kpdl:kposs,misc:typo,compose:menu&
xmodmap ~/.Xmodmap&

exec ck-launch-session startkde
