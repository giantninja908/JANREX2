use super::Gamestate;
use raylib::prelude::*;

impl Gamestate {
    /// render function
    /// renders the GameState, takes raylib requirements
    pub fn render(&mut self, mut rl: &mut raylib::RaylibHandle, thread: &RaylibThread) {
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::BLACK);
        d.draw_text(&format!("Game Code: {}", self.code), 0, 0, 20, Color::WHITE);

        d.draw_text(&format!("{}", self.time), 500, 0, 20, Color::WHITE);

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
                match &i.sender {
                    Some(_) => Color::WHITE,
                    None => Color::PURPLE,
                },
            );
            c -= 20;
        }


        let active_menu = match self.menus.active {
            super::ActiveMenu::MainMenu => &mut self.menus.main_menu,
            super::ActiveMenu::InGame => &mut self.menus.in_game,
        };

        active_menu.draw(&mut d, thread, Vector2::zero(), self.window_size);
    }
}
