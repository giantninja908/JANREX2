use super::Gamestate;
use raylib::prelude::*;
use rand::seq::SliceRandom; // for random spawn position

impl Gamestate {
    pub fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        if rl.is_window_resized() {
            self.window_size =
                Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32)
        }

        // retarded speed up for performance which probably doesnt even matter
        if self.code_last.is_none() || &self.code != self.code_last.as_ref().unwrap() {
            rl.set_window_title(thread, &format!("JANREX 2: {}", self.code));
            self.code_last = Some(self.code.clone())
        }

        let mpos = rl.get_mouse_position();

        let fps = rl.get_fps();

        // getting new active menu
        if let Some(new_menu) = match self.menus.active {
            super::ActiveMenu::MainMenu => {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                    rl.set_camera_mode(&self.camera_state.camera, CameraMode::CAMERA_FIRST_PERSON);
                    Some(super::ActiveMenu::InGame)
                } else {
                    None
                }
            },
            super::ActiveMenu::InGame => {
                if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) { // when having ESCAPE key as raylib's exit button, this will not work
                    rl.set_camera_mode(&self.camera_state.camera, CameraMode::CAMERA_FREE);
                    self.last_spawn = None; // DEBUG, so we get a new position
                    Some(super::ActiveMenu::MainMenu)
                } else {
                    None
                }
            },
        } {
            self.menus.active = new_menu;
        }

        let active_menu = match self.menus.active {
            super::ActiveMenu::MainMenu => {
                let time = rl.get_time() as f32;
                self.camera_state.camera = super::new_camera_main_menu(&time);
                &mut self.menus.main_menu
            },
            super::ActiveMenu::InGame => {
                if self.last_spawn.is_none() {
                    println!("Updating spawn POS");
                    self.last_spawn = Some(self.map.spawns.choose(&mut rand::thread_rng()).unwrap().clone());
                    let last_spawn = self.last_spawn.unwrap();
                    self.camera_state.camera = Camera3D::perspective(
                        last_spawn.pos,
                        last_spawn.pos, // target rotation needs to be set from the map spawn rotation
                        Vector3::new(0.0, 1.0, 0.0),
                        90.0,
                    );
                    self.camera_state.camera.target.x += 1.0; // needs a little (diff to position) otherwise nothing gets seen
                }
                let movement_speed = 100.0/fps as f32;

                // fov
                if rl.is_key_down(KeyboardKey::KEY_Q) {
                    self.camera_state.camera.fovy += movement_speed;
                }
                if rl.is_key_down(KeyboardKey::KEY_E) {
                    self.camera_state.camera.fovy -= movement_speed;
                }
                // movement
                let mut forward_speed = 0.0;
                let mut side_speed = 0.0;
                let mut up_speed = 0.0;
                if rl.is_key_down(KeyboardKey::KEY_W) {
                    forward_speed += movement_speed;
                }
                if rl.is_key_down(KeyboardKey::KEY_S) {
                    forward_speed -= movement_speed;
                }
                if rl.is_key_down(KeyboardKey::KEY_A) {
                    side_speed -= movement_speed;
                }
                if rl.is_key_down(KeyboardKey::KEY_D) {
                    side_speed += movement_speed;
                }
                // up/down
                if rl.is_key_down(KeyboardKey::KEY_UP) {
                    up_speed += movement_speed;
                }
                if rl.is_key_down(KeyboardKey::KEY_DOWN) {
                    up_speed -= movement_speed;
                }

                // Thanks to: https://github.com/JeffM2501/raylibExtras/blob/index/rlExtrasCPP/FreeCamera.h
                // should be rewritten to use quaternions

                let deg_2_rad = 0.017453292519943295; // https://numpy.org/doc/stable/reference/generated/numpy.deg2rad.html
                let deg_per_pixel = self.camera_state.camera.fovy / rl.get_screen_height() as f32;
                let delta = (self.last_mpos-mpos) * deg_per_pixel * deg_2_rad;

                let mut forward = self.camera_state.camera.target - self.camera_state.camera.position;
                forward.normalize();

                let right = forward.cross(self.camera_state.camera.up);

                let tilt_mat = raylib::core::math::Matrix::rotate(right, delta.y);
                forward.transform(tilt_mat);

                let spin_mat = raylib::core::math::Matrix::rotate(self.camera_state.camera.up, delta.x);
                forward.transform(spin_mat);

                let right = forward.cross(self.camera_state.camera.up);

                
                // REMOVE COMMENTS WHEN WANTING TO TRY IT OUT, NOT WORKING PROPERLY AT ALL
                // collisions
                // let player_box = super::maps::PlayerBox {
                //     position: self.camera_state.camera.position,
                //     scale: Vector3::new(1.0, 2.0, 1.0),
                //     pos_max: self.camera_state.camera.position + Vector3::new(1.0, 2.0, 1.0),
                //     pos_min: self.camera_state.camera.position - Vector3::new(1.0, 2.0, 1.0),
                // };

                // // figure out how to interact with collisions
                // let mut collidable = 0;
                // let mut collided_objects: Vec<&super::maps::Object> = vec![];
                // for obj in self.map.objects.iter() {
                //     if obj.collision {
                //         collidable += 1;
                //         if super::maps::intersect(&player_box, obj) {
                //             // handle collisions here
                //             self.camera_state.camera.position = obj.position + obj.scale * 1.05; // this is just retarded | closest point of collision needs to be found and set so they dont colide anymore
                //             collided_objects.push(obj);
                //         }
                //     }
                // }
                // println!("Collidable: {collidable}, Collisions: {}", collided_objects.len());

                // updates
                self.camera_state.camera.position += forward * forward_speed;
                self.camera_state.camera.position += right * side_speed;
                self.camera_state.camera.position.y += up_speed;

                self.camera_state.camera.target = self.camera_state.camera.position + forward;

                // prints
                // println!("Mouse Pos - Now: {:?}, Delta: {:?}, target: {:?}", mpos, delta, self.camera_state.camera.target);
                // println!("Mouse Pos - Now: {:?}, Forward: {:?}, Delta: {:?}", mpos, forward, delta);

                // rl.update_camera(&mut self.camera_state.camera); // raylib's built-in camera update function thats garbage
                &mut self.menus.in_game
            },
        };

        self.last_mpos = mpos;
        // println!("Mouse position: {:?}", mpos);

        active_menu.update(
            mpos,
            Vector2::zero(),
            self.window_size,
            rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON),
        );

    }
}
