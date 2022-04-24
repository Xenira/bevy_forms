use bevy_render::color::Color;

#[derive(Default)]
pub struct Theme {
    pub colors: ThemeColors,
}

pub struct ThemeColors {
    pub primary: Color,
    pub button: Color,
    pub button_hovered: Color,
    pub button_clicked: Color,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            primary: Default::default(),
            button: Default::default(),
            button_hovered: Default::default(),
            button_clicked: Default::default(),
        }
    }
}
