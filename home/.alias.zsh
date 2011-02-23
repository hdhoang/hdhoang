# -*- mode: sh -*-
h=~/Public/hdh-misc.git/
c=~h/system-config/$(hostname -s)
t=~/t
dl=~/Public/mirror/downloads/
pkg=/var/cache/pacman/pkg/
el=/var/log/everything.log

f=~/Public/fav
pool=/mnt/marbi
alias favd='ls ~f/Music |cut -d \[ -f 1 -s | uniq -d'
function favm() {
    prv 's/ /_/g;s/([^_])-_/\1__/g' ~f/Music/*' '*
    prv "s/''/_/g" ~f/Music/*\'\'*
    rename '_[Unknown_Album]' '' ~/Music/*Unknown_Album*
    mv -iv ~/Music/^desktop.ini(.) ~f/Music
}
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
