set -Ua fish_features remove-percent-self test-require-arg

bind alt-s 'fish_commandline_prepend run0'

#10213
bind ctrl-c __fish_cancel_commandline
