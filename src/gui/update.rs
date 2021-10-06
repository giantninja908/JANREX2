use super::*;

impl GuiElement {
    pub fn update(
        &mut self,
        mpos: Vector2,
        offset: Vector2,
        scaler: Vector2,
        clicked: bool,
    ) -> bool {
        let (pos, scale) = match self {
            &mut Self::Box {
                x,
                y,
                width,
                height,
                ..
            }
            | &mut Self::Button {
                x,
                y,
                width,
                height,
                ..
            }
            | &mut Self::Text {
                x,
                y,
                width,
                height,
                ..
            }
            | &mut Self::Image {
                x,
                y,
                width,
                height,
                ..
            } => (
                Vector2::new(
                    match x {
                        GuiSize::Fixed(pos) => (pos * scaler.x) + offset.x,
                        GuiSize::Centered(pos) => match width {
                            GuiSize::Fixed(wi) | GuiSize::Centered(wi) => {
                                (pos - wi / 2.0) * scaler.x + offset.x
                            }
                        },
                    },
                    match y {
                        GuiSize::Fixed(pos) => (pos * scaler.y) + offset.y,
                        GuiSize::Centered(pos) => match height {
                            GuiSize::Fixed(wi) | GuiSize::Centered(wi) => {
                                (pos - wi / 2.0) * scaler.y + offset.y
                            }
                        },
                    },
                ),
                match width {
                    GuiSize::Fixed(w) | GuiSize::Centered(w) => match height {
                        GuiSize::Fixed(h) | GuiSize::Centered(h) => {
                            Vector2::new(w * scaler.x, h * scaler.y)
                        }
                    },
                },
            ),
        };
        match self {
            Self::Button {
                status,
                x,
                y,
                width,
                height,
                ..
            } => {
                //collision
                let pos = Vector2::new(
                    match x {
                        GuiSize::Fixed(pos) => (*pos * scaler.x) + offset.x,
                        GuiSize::Centered(pos) => match width {
                            GuiSize::Fixed(wi) | GuiSize::Centered(wi) => {
                                (*pos - *wi / 2.0) * scaler.x + offset.x
                            }
                        },
                    },
                    match y {
                        GuiSize::Fixed(pos) => (*pos * scaler.y) + offset.y,
                        GuiSize::Centered(pos) => match height {
                            GuiSize::Fixed(wi) | GuiSize::Centered(wi) => {
                                (*pos - *wi / 2.0) * scaler.y + offset.y
                            }
                        },
                    },
                );

                let scale = match width {
                    GuiSize::Fixed(w) | GuiSize::Centered(w) => match height {
                        GuiSize::Fixed(h) | GuiSize::Centered(h) => {
                            Vector2::new(*w * scaler.x, *h * scaler.y)
                        }
                    },
                };

                if mpos.x < pos.x + scale.x
                    && mpos.x > pos.x
                    && mpos.y < pos.y + scale.y
                    && mpos.y > pos.y
                {
                    //collision found!
                    if clicked {
                        *status = GuiButtonStatus::Click;
                    } else {
                        *status = GuiButtonStatus::Hover;
                    }
                    return true;
                } else {
                    *status = GuiButtonStatus::Idle;
                    return false;
                }
            }
            Self::Box { children, .. } => {
                if let Some(childs) = children {
                    return childs
                        .iter_mut()
                        .map(|child| child.update(mpos, pos, scale, clicked))
                        .fold(false, |a, b| a || b); // cool functional stuffz
                } else {
                    return false;
                }
            }
            Self::Text { .. } | Self::Image { .. } => false,
        }
    }
}
