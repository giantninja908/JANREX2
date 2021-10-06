use raylib::prelude::*;
mod draw;
mod update;
pub use draw::*;
pub use update::*;

#[derive(Debug, Clone, Copy)]
pub struct GuiBorder {
    color: Color,
    size: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum GuiSize {
    Fixed(f32),
    Centered(f32),
}

#[derive(Debug, Clone)]
pub enum GuiButtonStatus {
    Hover,
    Idle,
    Click,
}

#[derive(Debug)]
pub enum GuiElement {
    Button {
        width: GuiSize,
        height: GuiSize,
        x: GuiSize,
        y: GuiSize,
        text: String,
        font_size: i32,
        text_color: Color,
        color: Color,
        border: Option<GuiBorder>,
        hover_color: Color,
        click_color: Color,
        status: GuiButtonStatus,
        index: u32,
    },
    Text {
        width: GuiSize,
        height: GuiSize,
        x: GuiSize,
        y: GuiSize,
        color: Color,
        text: String,
        font_size: i32,
        index: u32, //text index, NOT a button index
    },
    Box {
        width: GuiSize,
        height: GuiSize,
        x: GuiSize,
        y: GuiSize,
        border: Option<GuiBorder>,
        children: Option<Vec<GuiElement>>,
        color: Color,
    },
    Image {
        texture: Texture2D,
        width: GuiSize,
        height: GuiSize,
        x: GuiSize,
        y: GuiSize,
        tint: Option<Color>,
    },
}
