-- -*- mode: prog; tab-width: 2; comment-start: "-- " -*-

local wezterm = require("wezterm")
local config = wezterm.config_builder()

config.default_prog = {"nu"}

config.term = "wezterm"
config.color_scheme = "CLRS"

if wezterm.target_triple == "x86_64-pc-windows-msvc" then
  config.default_cwd = "B:"
  config.color_scheme = "Tomorrow Night Burns"
else
  -- blank window on amd
  config.front_end = "WebGpu"
end

config.animation_fps = 1

config.font_dirs = { "../assets", "homes/assets" }
-- config.font_locator = "ConfigDirsOnly"
config.font = wezterm.font_with_fallback {
 "FiraCode Nerd Font",
 "Fira Code",
 "monospace",
 "Consolas",
}
config.font_size = 9.0
-- config.freetype_load_target = "Light"
config.freetype_render_target = "HorizontalLcd"

config.enable_scroll_bar = true
config.window_decorations = "RESIZE"

config.quick_select_patterns = {
 "[a-zA-Z._]{12,}", -- package names
}
config.hyperlink_rules = wezterm.default_hyperlink_rules()
config.hyperlink_rules[5] = nil -- regex": "\\b\\w+@

local launch_menu = {}
if wezterm.home_dir == "/home/hieuhg" then
  table.insert(launch_menu, {
    label = "j",
    args = { "sh", "-c", "ssh -At j 'tmux -u a -d || exec tmux -u'" },
  })
  table.insert(launch_menu, {
    label = "v",
    cwd = "/home/hieuhg/cc/",
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

wezterm.on("update-status", function(window)
  for _, b in ipairs(wezterm.battery_info()) do
    if b.state == "Discharging" then
     if b.time_to_empty < 900 then
       window:toast_notification("low battery", "plug it in")
     end
    end
  end
end)

return config
