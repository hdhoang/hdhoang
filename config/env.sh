#!/bin/bash
set -o allexport
source <(/usr/lib/systemd/user-environment-generators/30-systemd-environment-d-generator)
source <(/usr/lib/systemd/user-environment-generators/60-flatpak)
set +o allexport
