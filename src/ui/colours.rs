use iced::{Color, theme::Palette};

// 0–255 RGB — tweak these to taste
pub const SIDEBAR_BG: Color = Color::from_rgb8(30, 30, 38);
pub const SIDEBAR_ITEM_BG: Color = Color::from_rgb8(45, 45, 55);
pub const SIDEBAR_ITEM_HOVER: Color = Color::from_rgb8(60, 60, 72);
pub const SIDEBAR_SELECTED_BG: Color = Color::from_rgb8(70, 90, 140);
pub const SIDEBAR_TEXT: Color = Color::from_rgb8(220, 220, 225);

pub const MIDNIGHT_VIOLET: Color = Color::from_rgb8(36, 22, 35);
pub const PEARL_AQUA: Color = Color::from_rgb8(154, 210, 203);
pub const DUSTY_MAUVE: Color = Color::from_rgb8(156, 122, 151);
pub const DEEP_CRIMSON: Color = Color::from_rgb8(163, 0, 21);

pub const THIEVES_TOOLKIT_THEME: Palette = Palette {
    background: MIDNIGHT_VIOLET,
    text: Color::WHITE,
    primary: DUSTY_MAUVE,
    success: PEARL_AQUA,
    warning: Color::WHITE,
    danger: DEEP_CRIMSON,
};

