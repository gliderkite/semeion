use ggez::graphics::Color;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Palette(HashMap<u64, Color>);

impl Default for Palette {
    fn default() -> Self {
        Self::tint(Color::from_rgb(19, 99, 119), 200, 0.03)
    }
}

impl Palette {
    /// Gets the color associated to the given index.
    pub fn get(&self, index: u64) -> Color {
        let index = index % self.0.len() as u64;
        *self.0.get(&index).unwrap_or(&Color::BLACK)
    }

    /// Creates shades from the original color.
    fn tint(mut color: Color, shades: usize, factor: f32) -> Self {
        let mut colors = HashMap::new();

        for i in 0..shades {
            let (r, g, b) = color.to_rgb();
            let r = r as f32 + (255.0 - r as f32) * factor;
            let g = g as f32 + (255.0 - g as f32) * factor;
            let b = b as f32 + (255.0 - b as f32) * factor;

            color = Color::from_rgb(r as u8, g as u8, b as u8);
            colors.insert(i as u64, color);
        }

        Self(colors)
    }
}
