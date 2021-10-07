use super::Gamestate;
use raylib::prelude::*;

impl Gamestate {
    pub fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        if rl.is_window_resized() {
            self.window_size =
                Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32)
        }

        rl.set_window_title(thread, &format!("JANREX 2: {}", self.code));

        let mpos = rl.get_mouse_position();
        let active_menu = match self.menus.active {
            super::ActiveMenu::MainMenu => &mut self.menus.main_menu,
            super::ActiveMenu::InGame => &mut self.menus.in_game,
        };

        active_menu.update(
            mpos,
            Vector2::zero(),
            self.window_size,
            rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON),
        );
    }
}
