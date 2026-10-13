pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    pub fn to_rgb(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    // test modules which includes set of tests
    // this is nested module, because color.rs is declare as color module in src/main.rs,
    // which makes the color.rs is the parent module and `tests` as its child
    use super::Rgb;

    #[test]
    fn hex_fomats_channels_with_zero_padding() {
        let orange = Rgb {
            r: 255,
            g: 128,
            b: 0,
        };
        let black = Rgb { r: 0, g: 0, b: 0 };

        assert_eq!(orange.to_hex(), "#FF8000");
        assert_eq!(black.to_hex(), "#000000");
    }

    #[test]
    fn rgb_formats_channels_as_decimal() {
        let orange = Rgb {
            r: 255,
            g: 128,
            b: 0,
        };

        assert_eq!(orange.to_rgb(), "rgb(255, 128, 0)");
    }
}
