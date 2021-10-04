use super::Gamestate;
use raylib::prelude::*;

impl Gamestate {
    pub fn render(&mut self, mut rl: &mut raylib::RaylibHandle, thread: &RaylibThread) {
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::BLACK);
        d.draw_text(&format!("Game Code: {}", self.code), 0, 0, 20, Color::WHITE);

        d.draw_text(self.time, 500, 0, 20, Color::WHITE);

        let mut c = 700;
        for i in self.messages.iter().rev() {
            d.draw_text(
                &format!(
                    "{} : {}",
                    match &i.sender {
                        Some(sender) => sender,
                        None => "SERVER",
                    },
                    &i.content
                ),
                10,
                c,
                17,
                Color::WHITE,
            );
            c -= 20;
        }
    }
}
