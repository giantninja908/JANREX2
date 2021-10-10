use super::*;

impl GuiElement {
    pub fn draw(
        &self,
        d: &mut RaylibVRMode<RaylibDrawHandle>,
        thread: &RaylibThread,
        offset: Vector2,
        scaler: Vector2,
    ) {
        let pos = match self {
            Self::Box {
                x,
                y,
                width: w,
                height: h,
                ..
            }
            | Self::Button {
                width: w,
                height: h,
                x,
                y,
                ..
            }
            | Self::Text {
                width: w,
                height: h,
                x,
                y,
                ..
            }
            | Self::Image {
                width: w,
                height: h,
                x,
                y,
                ..
            } => Vector2::new(
                match x {
                    GuiSize::Fixed(pos) => (pos * scaler.x) + offset.x,
                    GuiSize::Centered(pos) => match w {
                        GuiSize::Fixed(wi) | GuiSize::Centered(wi) => {
                            (pos - wi / 2.0) * scaler.x + offset.x
                        }
                    },
                },
                match y {
                    GuiSize::Fixed(pos) => (pos * scaler.y) + offset.y,
                    GuiSize::Centered(pos) => match h {
                        GuiSize::Fixed(wi) | GuiSize::Centered(wi) => {
                            (pos - wi / 2.0) * scaler.y + offset.y
                        }
                    },
                },
            ),
        };
        let scale = match self {
            Self::Box { width, height, .. }
            | Self::Button { width, height, .. }
            | Self::Image { width, height, .. }
            | Self::Text { width, height, .. } => match width {
                GuiSize::Fixed(w) | GuiSize::Centered(w) => match height {
                    GuiSize::Fixed(h) | GuiSize::Centered(h) => {
                        Vector2::new(*w * scaler.x, *h * scaler.y)
                    }
                },
            },
        };
        match self {
            Self::Box {
                color,
                children,
                border,
                ..
            } => {
                d.draw_rectangle_v(pos, scale, color);
                if let Some(childs) = children {
                    for i in childs.iter() {
                        i.draw(d, thread, pos, scale);
                    }
                }
                if let Some(bord) = border {
                    d.draw_rectangle_lines_ex(
                        Rectangle::new(pos.x, pos.y, scale.x, scale.y),
                        bord.size as i32,
                        bord.color,
                    );
                }
            }
            Self::Image { tint, texture, .. } => d.draw_texture_pro(
                texture,
                Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32),
                Rectangle::new(pos.x, pos.y, scale.x, scale.y),
                Vector2::zero(),
                0.0,
                match tint {
                    &Some(col) => col,
                    None => Color::WHITE,
                },
            ),
            Self::Text {
                width,
                height,
                x,
                y,
                color,
                text,
                font_size,
                ..
            } => {
                let centpos = Vector2::new(
                    match width {
                        GuiSize::Centered(w) => {
                            let text_width = text::measure_text(text, *font_size);
                            match x {
                                GuiSize::Fixed(_) => {
                                    pos.x + ((*w * scaler.x - text_width as f32) / 2.0)
                                }
                                GuiSize::Centered(_) => {
                                    pos.x - (text_width as f32 / 2.0) + (w / 2.0) * scaler.x
                                }
                            }
                        }
                        GuiSize::Fixed(_) => match x {
                            &GuiSize::Fixed(_) | &GuiSize::Centered(_) => pos.x,
                        },
                    },
                    match height {
                        GuiSize::Centered(h) => match y {
                            GuiSize::Fixed(_) => {
                                pos.y + ((*h * scaler.y - *font_size as f32) / 2.0)
                            }
                            GuiSize::Centered(_) => {
                                pos.y - (*font_size as f32 / 2.0) + (h / 2.0) * scaler.y
                            }
                        },
                        GuiSize::Fixed(_) => match y {
                            &GuiSize::Fixed(_) | &GuiSize::Centered(_) => pos.y,
                        },
                    },
                );
                d.draw_text(text, centpos.x as i32, centpos.y as i32, *font_size, color);
            }
            Self::Button {
                border,
                text,
                font_size,
                color,
                x,
                y,
                width,
                height,
                text_color,
                hover_color,
                click_color,
                status,
                ..
            } => {
                d.draw_rectangle_v(
                    pos,
                    scale,
                    match status {
                        &GuiButtonStatus::Idle => color,
                        &GuiButtonStatus::Click => click_color,
                        &GuiButtonStatus::Hover => hover_color,
                    },
                );
                let centpos = Vector2::new(
                    match width {
                        GuiSize::Fixed(w) | GuiSize::Centered(w) => {
                            let text_width = text::measure_text(text, *font_size);
                            match x {
                                GuiSize::Fixed(_) => pos.x + (*w - text_width as f32) / 2.0,
                                GuiSize::Centered(_) => {
                                    pos.x - (text_width as f32 / 2.0) + (w / 2.0) * scaler.x
                                }
                            }
                        }
                    },
                    match height {
                        GuiSize::Fixed(h) | GuiSize::Centered(h) => match y {
                            GuiSize::Fixed(_) => pos.y + (*h - *font_size as f32) / 2.0,
                            GuiSize::Centered(_) => {
                                pos.y - (*font_size as f32 / 2.0) + (h / 2.0) * scaler.y
                            }
                        },
                    },
                );
                d.draw_text(
                    text,
                    centpos.x as i32,
                    centpos.y as i32,
                    *font_size,
                    text_color,
                );
                if let Some(bord) = border {
                    d.draw_rectangle_lines_ex(
                        Rectangle::new(pos.x, pos.y, scale.x, scale.y),
                        bord.size as i32,
                        bord.color,
                    );
                }
            }
        }
    }
}
