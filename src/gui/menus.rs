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
            children: Some(vec![
                Box {
                    // chat box
                    border: Some(GuiBorder {
                        color: Color::new(128, 128, 128, 128),
                        size: 2.0,
                    }),
                    x: GuiSize::Fixed(0.0),
                    y: GuiSize::Fixed(0.5),
                    color: Color::new(13, 13, 13, 180),
                    height: GuiSize::Fixed(0.5),
                    width: GuiSize::Fixed(0.2),
                    children: Some(vec![Text {
                        width: GuiSize::Fixed(0.9),
                        height: GuiSize::Fixed(0.9),
                        x: GuiSize::Fixed(0.05),
                        y: GuiSize::Fixed(0.05),
                        color: Color::WHITE,
                        font_size: 12,
                        index: 0,
                        text: String::new(),
                    }]),
                },
                Text {
                    width: GuiSize::Fixed(0.2),
                    height: GuiSize::Fixed(0.2),
                    x: GuiSize::Centered(0.5),
                    y: GuiSize::Fixed(0.0),
                    color: Color::WHITE,
                    index: 1,
                    font_size: 30,
                    text: String::from("00:00"),
                },
            ]),
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
