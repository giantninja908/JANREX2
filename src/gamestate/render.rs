use super::Gamestate;
use raylib::prelude::*;
// for random spawn position
// use rand::seq::SliceRandom;

impl Gamestate {
    /// render function
    /// renders the GameState, takes raylib requirements
    pub fn render(&mut self, mut rl: &mut raylib::RaylibHandle, thread: &RaylibThread) {
        let time = rl.get_time() as f32;
        
        let fps = raylib::RaylibHandle::get_fps(rl);

        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::BLACK);


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
            .take(7) //limit to 7 chat messages at once
            .fold(String::new(), |a, b| a + &format!("\n{}", b.0));

        let active_menu = match self.menus.active {
            super::ActiveMenu::MainMenu => {
                self.menus.main_menu.mod_text(0, txt);
                self.menus.main_menu.mod_text(1, format!("{}", self.time));
                &mut self.menus.main_menu
            }
            // super::ActiveMenu::InGame => &mut self.menus.in_game,
            super::ActiveMenu::InGame => &mut self.menus.main_menu, // used this because in_game for some reason has the map not rendered
        };

        {
            //3d rendering!!!
            let mut d2 = d.begin_mode3D(self.camera_state.camera); // _ shit code, unwrap shouldnt be used since there are better ways
            self.map.render(&mut d2, thread);
        }

        // rendering top left game code and fps display
        d.draw_text(&format!("Game Code: {}", self.code), 0, 0, 20, Color::WHITE);
        d.draw_text(&format!("FPS: {}", fps), 0, 20, 20, Color::WHITE);

        active_menu.draw(&mut d, thread, Vector2::zero(), self.window_size);
    }
}
