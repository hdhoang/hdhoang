-- -*- mode: prog; tab-width: 2; comment-start: "-- " -*-

local wezterm = require("wezterm")
local config = wezterm.config_builder()

config.color_scheme = "CLRS"

config.default_prog = { "zellij" }
if wezterm.target_triple == "x86_64-pc-windows-msvc" then
  config.default_cwd = "B:"
  config.default_prog = { "nu" }
end

config.front_end = "WebGpu"
config.animation_fps = 1

config.font_dirs = { "../assets", "homes/assets" }
-- config.font_locator = "ConfigDirsOnly"
config.font = wezterm.font_with_fallback {
 { family = "JuliaMono Nerd Font Mono", weight = "Medium", }, -- wezterm doesn't see non-Mono version
 { family = "AtkynsonMono Nerd Font", weight = "Medium", scale = 1.05, },
 { family = "Kelmscott Mono", scale = 1.2 },
 "B612 Mono",
 "FiraCode Nerd Font",
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

config.use_ime = false
config.enable_kitty_keyboard = true
config.key_map_preference = "Physical"
config.keys = {
  { key = "phys:K", mods = "CTRL|SHIFT", action = wezterm.action{SpawnTab="CurrentPaneDomain"} },
  { key = ",", mods = "CTRL|SHIFT", action = wezterm.action{CloseCurrentTab={confirm=true}} },

  { key = "phys:I", mods = "CTRL|SHIFT", action = wezterm.action{CopyTo="Clipboard"} },
  { key = ".", mods = "CTRL|SHIFT", action = wezterm.action{PasteFrom="Clipboard"} },

  { key = "phys:H", mods = "CTRL|SHIFT", action = "ShowLauncher" },

  -- https://github.com/acomagu/fish-osc133
  { key = "UpArrow", mods = "SHIFT", action = wezterm.action.ScrollToPrompt(-1) },
  { key = "DownArrow", mods = "SHIFT", action = wezterm.action.ScrollToPrompt(1) },
}
config.mouse_bindings = {
  {
    event = { Down = { streak = 4, button = "Left" } },
    action = wezterm.action.SelectTextAtMouseCursor "SemanticZone",
    mods = "NONE",
  },
}

return config
