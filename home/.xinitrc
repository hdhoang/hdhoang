source /etc/X11/xinit/xinitrc.d/40-libcanberra-gtk-module
source ~/.profile

eval `ssh-agent`
#emacs --daemon&

setxkbmap -layout us -variant dvorak -option ctrl:nocaps,grp_led:caps,keypad:oss,misc:typo,lv3:switch,compose:menu&
xmodmap ~/.Xmodmap&

exec ck-launch-session startkde
