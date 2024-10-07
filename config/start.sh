#!/bin/bash -eux
echo #`# <#`

cd ${BASH_SOURCE%/*}

# https://wezfurlong.org/wezterm/config/lua/config/term.html
tic -x $PWD/wezterm.terminfo

mkdir -vp ~/.local/share/fonts/
for file in ../assets/*.?tf; do
    ln -rsvf $file ~/.local/share/fonts/
done
gsettings set org.gnome.desktop.interface document-font-name "${GTK_FONT_NAME}"
gsettings set org.gnome.desktop.interface font-name "${GTK_FONT_NAME}"
gsettings set org.gnome.desktop.interface monospace-font-name "FiraCode Nerd Font"

ln -rsvf $PWD/tool.toml ~/.tool.toml
ln -rsvf $PWD/Justfile ~/Justfile

mkdir -vp ~/.config/{emacs,pijul,jj,wezterm,fish,rink,environment.d,sway/config.d,nushell,rclone,rsgain/presets,tridactyl,fontconfig/conf.d,tmux}/ ~/run/

ln -rsvf $PWD/ebur128.ini ~/.config/rsgain/presets/
ln -rsvf $PWD/config.fish ~/.config/fish/
ln -rsvf $PWD/config.nu ~/.config/nushell/
ln -rsvf $PWD/env.nu ~/.config/nushell/

ln -rsvf $PWD/,unikey ~/run/
ln -rsvf $PWD/sway.conf ~/.config/sway/config.d/i3.conf
ln -rsvf $PWD/waybar.config ~/.config/waybar/config

ln -rsvf $PWD/$HOSTNAME.conf ~/.config/sway/environment
ln -rsvf $PWD/$HOSTNAME.conf ~/.config/environment.d/
ln -rsvf $PWD/env.conf ~/.config/environment.d/00-env.conf

mkdir -vp ~/.config/plasma-workspace/env/
ln -rsvf $PWD/env.sh  ~/.config/plasma-workspace/env/

ln -rsvf $PWD/init.el ~/.config/emacs/
ln -rsvf ~/run ~/.config/emacs/tree-sitter

ln -rsvf $PWD/tmux.conf ~/.config/tmux/
ln -rsvf $PWD/_tridactylrc ~/.config/tridactyl/tridactylrc

ln -rsvf $PWD/gitconfig ~/.gitconfig
ln -rsvf $PWD/pijul_config.toml ~/.config/pijul/config.toml
ln -rsvf $PWD/jj_config.toml ~/.config/jj/config.toml
ln -rsvf $PWD/rink.toml ~/.config/rink/config.toml

ln -rsvf $PWD/46-twemoji-color.conf ~/.config/fontconfig/conf.d/
ln -rsvf $PWD/fonts.conf ~/.config/fontconfig/
ln -rsvf $PWD/wezterm.lua ~/.config/wezterm/

mkdir -p ~/.cargo/
ln -rsvf $PWD/cargo.toml ~/.cargo/config.toml
ln -rsvf $PWD/../rustfmt.toml ~/rustfmt.toml

exit #> > $null

iwr https://fly.io/install.ps1 -useb | iex

setx HOME b:

# https://github.com/vosen/ZLUDA#known-issues
setx HIP_VISIBLE_DEVICES=1

# Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
# irm get.scoop.sh | iex
# scoop bucket add extras
