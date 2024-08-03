use ratatui::style::Color;

pub struct Palette {
    pub name: String,
    pub colors: ColorPalette,
}

impl Palette {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            colors: ColorPalette::new(name),
        }
    }
}

pub struct ColorPalette {
    pub background: Color,
    pub foreground: Color,
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
    pub gray: Color,
    pub light_gray: Color,
    pub light_red: Color,
    pub light_green: Color,
    pub light_yellow: Color,
    pub light_blue: Color,
    pub light_magenta: Color,
    pub light_cyan: Color,
    pub white: Color,
}

impl ColorPalette {
    pub fn new(name: &str) -> Self {
        match name {
            "default" => Self {
                background: Color::Rgb(21, 21, 21),
                foreground: Color::Rgb(215, 208, 199),
                black: Color::Rgb(16, 16, 16),
                red: Color::Rgb(210, 61, 61),
                green: Color::Rgb(160, 207, 93),
                yellow: Color::Rgb(243, 157, 33),
                blue: Color::Rgb(78, 159, 177),
                magenta: Color::Rgb(133, 66, 255),
                cyan: Color::Rgb(66, 113, 123),
                gray: Color::Rgb(44, 44, 44),
                light_gray: Color::Rgb(221, 221, 221),
                light_red: Color::Rgb(232, 79, 79),
                light_green: Color::Rgb(184, 214, 140),
                light_yellow: Color::Rgb(225, 170, 93),
                light_blue: Color::Rgb(125, 193, 207),
                light_magenta: Color::Rgb(155, 100, 251),
                light_cyan: Color::Rgb(109, 135, 141),
                white: Color::Rgb(221, 221, 221),
            },
            _ => Self {
                background: Color::Rgb(255, 255, 255),
                foreground: Color::Rgb(16, 16, 16),
                black: Color::Rgb(16, 16, 16),
                red: Color::Rgb(210, 61, 61),
                green: Color::Rgb(160, 207, 93),
                yellow: Color::Rgb(243, 157, 33),
                blue: Color::Rgb(78, 159, 177),
                magenta: Color::Rgb(133, 66, 255),
                cyan: Color::Rgb(66, 113, 123),
                gray: Color::Rgb(64, 64, 64),
                light_gray: Color::Rgb(221, 221, 221),
                light_red: Color::Rgb(232, 79, 79),
                light_green: Color::Rgb(184, 214, 140),
                light_yellow: Color::Rgb(225, 170, 93),
                light_blue: Color::Rgb(125, 193, 207),
                light_magenta: Color::Rgb(155, 100, 251),
                light_cyan: Color::Rgb(109, 135, 141),
                white: Color::Rgb(221, 221, 221),
            },
        }
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self::new("dark")
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
