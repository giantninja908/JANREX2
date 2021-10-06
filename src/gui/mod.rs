use raylib::prelude::*;
mod draw;
mod menus;
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

// gui helper functions
impl GuiElement {
    /// given an index and new string, modify a TEXT object recursively
    pub fn mod_text(&mut self, index: u32, new_text: String) -> bool {
        match self {
            GuiElement::Box { children, .. } => {
                //recursion
                if let Some(childs) = children {
                    childs
                        .iter_mut()
                        .map(|child| child.mod_text(index, new_text.to_owned()))
                        .fold(false, |a, b| a || b) // see if one returns true, and return that as a success
                } else {
                    false
                }
            }
            GuiElement::Text {
                index: indx, text, ..
            } => {
                if &index == indx {
                    *text = new_text;
                    return true;
                } else {
                    return false;
                }
            }
            _ => false,
        }
    }

    /// returns indecies of all children buttons that are clicked recursively
    pub fn pressed_buttons(&self) -> Vec<u32> {
        match self {
            GuiElement::Box { children, .. } => {
                //recursion
                if let Some(childs) = children {
                    childs
                        .iter()
                        .map(|child| child.pressed_buttons())
                        .flatten()
                        .collect::<Vec<_>>()
                } else {
                    Vec::new()
                }
            }
            GuiElement::Button { status, index, .. } => {
                return match status {
                    GuiButtonStatus::Click => vec![*index],
                    _ => Vec::new(),
                }
            }
            _ => Vec::new(),
        }
    }
}
