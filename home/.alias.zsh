# -*- mode: sh -*-
function favm() {
    perl-rename "s/- /__/g;s/ /_/g;s/''/_/g;s/_\[Unknown_Album\]//;s/\[unknown\]/(unknown)/" ~/Music/*(.)
    mv -iv ~/Music/^desktop.ini(.) $f/Music
}

alias -s cbr=okular
alias -s cbz=okular
alias -s pdf=okular
alias -s html=firefox
alias -s htm=firefox
alias -s torrent=ktorrent
alias -s avi=m
alias -s mp4=m
alias -s mpeg=m
alias -s mpg=m
alias -s wmv=m
alias -s mov=m
alias -s m4v=m
alias -s flv=m
alias -s webm=m

