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
