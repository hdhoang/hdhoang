# -*- mode: sh -*-
h=~/Public/hdh-misc.git/
c=~h/system-config/$(hostname -s)
t=~/t
dl=~/Downloads/
g=/mnt/cmaluz/@g/
pkg=/var/cache/pacman/pkg/
el=/var/log/messages

f=~/Public/fav
pool=/mnt/marbi
function favm() {
    prv "s/- /__/g;s/ /_/g;s/''/_/g" ~f/Music/*' '*
    rename '_[Unknown_Album]' '' ~/Music/*Unknown_Album*
    mv -iv ~/Music/^desktop.ini(.) ~f/Music
}
function com() {
    prv 's/rar$/cbr/;s/zip$/cbz/' {~dl,/mnt/cmaluz/@d}/*(.)
    mv {~dl,/mnt/cmaluz/@d}/*cb? ~/Public/mirror/
}
alias favd='ls ~f/Music |cut -d \[ -f 1 -s | uniq -d'
alias fava='ls ~f/Music |cut -d - -f 1 -s | uniq -c | sort -n'
alias favs='xattr fav fav ~f/**/*; rs --delete ~f ~pool'

alias -s cbr=okular
alias -s cbz=okular
alias -s pdf=okular
alias -s html=firefox
alias -s htm=firefox
alias -s torrent=ktorrent
alias -s avi=vlc
alias -s mp4=vlc
alias -s mpg=vlc
alias -s wmv=vlc
