-- -*- mode: prog; tab-width: 2 -*-

local wezterm = require("wezterm")
local config = wezterm.config_builder()

config.default_prog = {"nu"}
config.default_cwd = "~"

config.term = "wezterm"
config.color_scheme = "CLRS"

if wezterm.target_triple == "x86_64-pc-windows-msvc" then
  config.default_cwd = "B:"
  config.color_scheme = "Tomorrow Night Burns"
end

config.font = wezterm.font_with_fallback {
 "Fira Code",
 "monospace",
 "Consolas",
}
config.font_size = 9.0

config.enable_scroll_bar = true
config.window_decorations = 'RESIZE'

config.hyperlink_rules = wezterm.default_hyperlink_rules()
-- regex": "\\b\\w+@
config.hyperlink_rules[5] = nil

local launch_menu = {}
if wezterm.home_dir == "/home/hieuhg" then
  table.insert(launch_menu, {
    label = "j",
    args = { "sh", "-c", "ssh -At j 'tmux -u a -d || exec tmux -u'" },
  })
  table.insert(launch_menu, {
    label = "v",
    cwd = "/home/hieuhg/cc/staff-vpn-2025",
    args = { "sh", "-c", "sudo-rs openvpn --config staff_ca_2033.ovpn --config dns.ovpn" },
  })
  table.insert(launch_menu, {
    label = "i",
    cwd = "/home/hieuhg/cc/",
    args = { "bash", "./ipmiview.bash" },
  })
  config.default_prog = { "rfish" }
else
  table.insert(launch_menu, {
    label = "map",
    args = { "ssh", "hdhoang@map", "tmux", "-u", "a", "-d" },
  })
end
config.launch_menu = launch_menu

config.use_ime = false
config.key_map_preference = "Physical"
config.keys = {
  { key = "phys:K", mods = "CTRL|SHIFT", action = wezterm.action{SpawnTab="CurrentPaneDomain"} },
  { key = ",", mods = "CTRL|SHIFT", action = wezterm.action{CloseCurrentTab={confirm=true}} },

  { key = "phys:I", mods = "CTRL|SHIFT", action = wezterm.action{CopyTo="Clipboard"} },
  { key = ".", mods = "CTRL|SHIFT", action = wezterm.action{PasteFrom="Clipboard"} },

  { key = "phys:H", mods = "CTRL|SHIFT", action = "ShowLauncher" },
}

return config
