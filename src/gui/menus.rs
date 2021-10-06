pub use super::{GuiBorder, GuiElement::*, GuiSize};
pub use raylib::prelude::Color;

impl super::GuiElement {
    pub fn main_menu() -> Self {
        Self::Box {
            width: GuiSize::Fixed(1.0),
            height: GuiSize::Fixed(1.0),
            border: None,
            x: GuiSize::Fixed(0.0),
            y: GuiSize::Fixed(0.0),
            color: Color::DARKGRAY,
            children: None,
        }
    }
    pub fn ingame_menu() -> Self {
        Self::Box {
            width: GuiSize::Fixed(1.0),
            height: GuiSize::Fixed(1.0),
            border: None,
            x: GuiSize::Fixed(0.0),
            y: GuiSize::Fixed(0.0),
            color: Color::DARKGRAY,
            children: None,
        }
    }
}
