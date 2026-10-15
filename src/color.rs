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

    /// Whole degrees and percentages, rounded to the nearest integer.
    /// Gray has hue 0; a hue that rounds to 360 wraps back to 0.
    pub fn hsl_channels(&self) -> (u16, u8, u8) {
        let (hue, min, max) = self.hue_and_bounds();
        let delta = max - min;
        let lightness = (max + min) / 2.0;
        let saturation = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * lightness - 1.0).abs())
        };

        (
            (hue.round() as u16) % 360,
            (saturation * 100.0).round() as u8,
            (lightness * 100.0).round() as u8,
        )
    }

    /// Uses the same rounding and gray-hue convention as `hsl_channels`.
    pub fn hsv_channels(&self) -> (u16, u8, u8) {
        let (hue, min, max) = self.hue_and_bounds();
        let saturation = if max == 0.0 { 0.0 } else { (max - min) / max };

        (
            (hue.round() as u16) % 360,
            (saturation * 100.0).round() as u8,
            (max * 100.0).round() as u8,
        )
    }

    pub fn to_hsl(&self) -> String {
        let (h, s, l) = self.hsl_channels();
        format!("hsl({h}, {s}%, {l}%)")
    }

    /// HSV notation for this app; `hsv()` is not a CSS color function.
    pub fn to_hsv(&self) -> String {
        let (h, s, v) = self.hsv_channels();
        format!("hsv({h}, {s}%, {v}%)")
    }

    // Return hue in degrees plus the smallest/largest normalized RGB channels.
    fn hue_and_bounds(&self) -> (f64, f64, f64) {
        let r = f64::from(self.r) / 255.0;
        let g = f64::from(self.g) / 255.0;
        let b = f64::from(self.b) / 255.0;
        let min = r.min(g).min(b);
        let max = r.max(g).max(b);
        let delta = max - min;
        let hue = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * ((g - b) / delta).rem_euclid(6.0)
        } else if max == g {
            60.0 * ((b - r) / delta + 2.0)
        } else {
            60.0 * ((r - g) / delta + 4.0)
        };

        (hue, min, max)
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

    #[test]
    fn gray_colors_have_zero_hue_and_saturation() {
        for (channel, percent) in [(0, 0), (128, 50), (255, 100)] {
            let color = Rgb {
                r: channel,
                g: channel,
                b: channel,
            };
            assert_eq!(color.hsl_channels(), (0, 0, percent));
            assert_eq!(color.hsv_channels(), (0, 0, percent));
        }
    }

    #[test]
    fn primary_and_secondary_colors_cover_the_hue_wheel() {
        for (r, g, b, hue) in [
            (255, 0, 0, 0),
            (255, 255, 0, 60),
            (0, 255, 0, 120),
            (0, 255, 255, 180),
            (0, 0, 255, 240),
            (255, 0, 255, 300),
        ] {
            let color = Rgb { r, g, b };
            assert_eq!(color.hsl_channels(), (hue, 100, 50));
            assert_eq!(color.hsv_channels(), (hue, 100, 100));
        }
    }

    #[test]
    fn conversions_round_fractional_hues_and_percentages() {
        let purple = Rgb {
            r: 124,
            g: 58,
            b: 237,
        };
        assert_eq!(purple.hsl_channels(), (262, 83, 58));
        assert_eq!(purple.hsv_channels(), (262, 76, 93));

        let almost_red = Rgb { r: 255, g: 0, b: 1 };
        assert_eq!(almost_red.hsl_channels(), (0, 100, 50));
        assert_eq!(almost_red.hsv_channels(), (0, 100, 100));
    }

    #[test]
    fn hsl_and_hsv_format_the_rounded_channels() {
        let orange = Rgb {
            r: 255,
            g: 128,
            b: 0,
        };
        assert_eq!(orange.to_hsl(), "hsl(30, 100%, 50%)");
        assert_eq!(orange.to_hsv(), "hsv(30, 100%, 100%)");
    }
}
