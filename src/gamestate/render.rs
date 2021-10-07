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
        let txt = self
            .messages
            .iter()
            .rev()
            .map(|i| {
                (
                    format!(
                        "{} : {}",
                        match &i.sender {
                            Some(sender) => sender,
                            None => "SERVER",
                        },
                        &i.content
                    ),
                    match &i.sender {
                        Some(_) => Color::WHITE,
                        None => Color::PURPLE,
                    },
                )
            })
            .fold(String::new(), |a, b| a + &format!("\n{}", b.0));

        let active_menu = match self.menus.active {
            super::ActiveMenu::MainMenu => {
                self.menus.main_menu.mod_text(0, txt);
                self.menus.main_menu.mod_text(1, format!("{}", self.time));
                &mut self.menus.main_menu
            }
            super::ActiveMenu::InGame => &mut self.menus.in_game,
        };

        active_menu.draw(&mut d, thread, Vector2::zero(), self.window_size);
    }
}
