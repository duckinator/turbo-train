use bevy::prelude::*;

#[derive(Clone, Debug)]
pub enum PieceColor {
    Red,
    Yellow,
    Orange,
    Green,
    Purple,
}

impl PieceColor {
    pub fn to_bevy_color(&self) -> Color {
        match self {
            PieceColor::Red => Color::hsl(360. * 1 as f32, 0.95, 0.7),
            PieceColor::Yellow => Color::hsl(360. * 2 as f32, 0.95, 0.7),
            PieceColor::Orange => Color::hsl(360. * 3 as f32, 0.95, 0.7),
            PieceColor::Green => Color::hsl(360. * 4 as f32, 0.95, 0.7),
            PieceColor::Purple => Color::hsl(360. * 5 as f32, 0.95, 0.7),
        }
    }
}
