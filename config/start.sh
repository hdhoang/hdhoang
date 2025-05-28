#!/bin/env fish
echo #`# <#`
set fish_trace 1

alias do=true
alias done=true
set BASH_SOURCE $(status current-filename 2>/dev/null)
cd $(dirname $BASH_SOURCE)

# https://wezfurlong.org/wezterm/config/lua/config/term.html
tic -x $PWD/wezterm.terminfo

flatpak -u override --filesystem=xdg-config/fontconfig:ro
mkdir -vp ~/.local/share/fonts/
for file in ../assets/*tf
do
    # use hardlinks to avoid app crashing when files disappear,
    # and also easier for flatpak apps to get access
    ln -vf $file ~/.local/share/fonts/
done
end
gsettings set org.gnome.desktop.interface font-name "$GTK_FONT_NAME"
gsettings set org.gnome.desktop.interface document-font-name "$GTK_FONT_NAME"
gsettings set org.gnome.desktop.interface monospace-font-name "Kelmscott Mono Medium"

ln -rsvf $PWD/tool.toml ~/.tool.toml
ln -rsvf $PWD/Justfile ~/Justfile
ln -rsvf $PWD/topgrade.toml ~/.config/topgrade.toml
ln -rsvf $PWD/../dprint.json ~/dprint.json

mkdir -vp ~/.config/{containers,emacs,jj/conf.d,wezterm,fish,rink,environment.d,systemd/user,sway/config.d,nushell,rclone,rsgain/presets,tridactyl,fontconfig/conf.d,tmux,zellij/layouts}/ ~/run/ ~/.crawl/

ln -rsvf $PWD/crawl-init.txt ~/.crawl/init.txt

ln -rsvf $PWD/,add-sysext ~/run/
ln -rsvf $PWD/,scrobble-filter.py ~/run/,scrobble-filter
ln -rsvf $PWD/rescrobbled.service ~/.config/systemd/user/
systemctl --user enable --now rescrobbled.service
ln -rsvf $PWD/,ctia ~/run/
ln -rsvf $PWD/ctia.timer ~/.config/systemd/user/ctia.timer
ln -rsvf $PWD/ctia.timer ~/.config/systemd/user/ctia.service
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
ln -rsvf $PWD/env.sh ~/.config/plasma-workspace/env/

ln -rsvf $PWD/early-init.el ~/.config/emacs/
ln -rsvf $PWD/init.el ~/.config/emacs/
ln -rsvf ~/run ~/.config/emacs/tree-sitter

ln -rsvf $PWD/tmux.conf ~/.config/tmux/
ln -rsvf $PWD/zellij.kdl ~/.config/zellij/config.kdl
ln -rsvf $PWD/play.kdl ~/.config/zellij/layouts/
ln -rsvf $PWD/_tridactylrc ~/.config/tridactyl/tridactylrc

ln -rsvf $PWD/gitconfig ~/.gitconfig
ln -rsvf $PWD/jj_config.toml ~/.jjconfig.toml
ln -rsvf $PWD/jj_specific.toml ~/.config/jj/conf.d/non-git.toml
ln -rsvf $PWD/rink.toml ~/.config/rink/config.toml
ln -rsvf $PWD/containers_storage.conf ~/.config/containers/storage.conf

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
