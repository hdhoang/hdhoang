# -*- mode: sh -*-
function favm() {
    prv "s/- /__/g;s/ /_/g;s/''/_/g;s/_\[Unknown_Album\]//" ~/Music/*(.)
    mv -iv ~/Music/^desktop.ini(.) ~f/Music
}

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
