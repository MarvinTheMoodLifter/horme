use ratatui::style::Color;

pub struct Palette {
    pub name: String,
    pub colors: ColorPalette,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            colors: ColorPalette::default(),
        }
    }
}

impl Palette {
    pub fn new(palette_name: &str, colors: ColorPalette) -> Self {
        Self {
            name: palette_name.to_string(),
            colors,
        }
    }
}

pub struct ColorPalette {
    pub background: Color,
    pub foreground: Color,
    pub altbackground: Color,
    pub todo: Color,
    pub doing: Color,
    pub done: Color,
    pub info: Color,
    pub subtask: Color,
    pub cancelled: Color,
}

impl ColorPalette {
    pub fn new(colors: Vec<String>) -> Self {
        Self {
            background: convert_hex_to_rgb(&colors[0]),
            foreground: convert_hex_to_rgb(&colors[1]),
            altbackground: convert_hex_to_rgb(&colors[2]),
            todo: convert_hex_to_rgb(&colors[3]),
            doing: convert_hex_to_rgb(&colors[4]),
            done: convert_hex_to_rgb(&colors[5]),
            info: convert_hex_to_rgb(&colors[6]),
            subtask: convert_hex_to_rgb(&colors[7]),
            cancelled: convert_hex_to_rgb(&colors[8]),
        }
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            background: Color::Rgb(25, 23, 36),
            foreground: Color::Rgb(224, 222, 244),
            altbackground: Color::Rgb(31, 29, 46),
            todo: Color::Rgb(246, 193, 119),
            doing: Color::Rgb(196, 167, 231),
            done: Color::Rgb(49, 116, 143),
            info: Color::Rgb(235, 188, 186),
            subtask: Color::Rgb(156, 207, 216),
            cancelled: Color::Rgb(235, 111, 146),
        }
    }
}

pub fn convert_hex_to_rgb(hex: &str) -> Color {
    // Don't consider the first '#' character
    // Check if it's a 3 or 6 character hex code
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    // Remove the '#' character
    let hex = &hex[1..];

    match hex.len() {
        3 => {
            // Convert 3 character hex code to 6 by repeating each character
            // and save the new value in the r, g, and b variables
            r = u8::from_str_radix(&hex[0..1].repeat(2), 16).unwrap();
            g = u8::from_str_radix(&hex[1..2].repeat(2), 16).unwrap();
            b = u8::from_str_radix(&hex[2..3].repeat(2), 16).unwrap();
        }
        6 => {
            r = u8::from_str_radix(&hex[0..2], 16).unwrap();
            g = u8::from_str_radix(&hex[2..4], 16).unwrap();
            b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        }
        _ => {
            return Color::Rgb(r, g, b);
        }
    }
    Color::Rgb(r, g, b)
}
