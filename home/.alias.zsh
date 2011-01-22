# -*- mode: sh -*-
h=~/Public/hdh-misc.git/
t=~/t
dl=~/Public/mirror/downloads/
pkg=/var/cache/pacman/pkg/

f=~/Public/fav
pool=/mnt/marbi
alias favd='ls ~f/Music |cut -d \[ -f 1 -s | uniq -d'
alias favm='rename "_[]" "" ~/Music/*(.); mv -iv ~/Music/*(.) ~f/Music'
alias favn="perl-rename -iv 's/ /_/g' ~f/Music/*' '*(.)"
alias favs='rs --delete ~f ~pool'

alias -s cbr=okular
alias -s cbz=okular
alias -s pdf=okular
alias -s html=firefox
alias -s htm=firefox
alias -s torrent=ktorrent
alias -s avi=vlc
alias -s mp4=vlc
