#!/bin/bash
if [[ $(ibus engine) == Unikey ]]; then
    ibus engine xkb:us:dvorak:eng
else
    ibus engine Unikey
fi
