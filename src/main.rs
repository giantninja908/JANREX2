// #[allow(dead_code)] // to reduce warning spam

mod consts;
mod gamestate;
mod gui;
mod key_rotate;
mod packet_sender;
mod token_fetch;
pub(crate) use raylib::prelude::*;

const WIDTH: i32 = 1920/2;
const HEIGHT: i32 = 1080/2;

#[tokio::main]
async fn main() {
    let (mut rl, thread) = raylib::init()
        .msaa_4x()
        .size(WIDTH, HEIGHT)
        .title("JANREX 2")
        .resizable()
        .build();

    rl.set_exit_key(None); // so switching between ingame window and main menu is possible
    rl.set_target_fps(120); // comment out for unlimited fps

    {
        let img_dat = include_bytes!("../assets/icon/icon.png");
        let img = raylib::core::texture::Image::load_image_from_mem(
            "png",
            &(img_dat.iter().map(|x| *x).collect()),
            img_dat.len() as i32,
        )
        .unwrap();
        rl.set_window_icon(img);
    }

    {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        let font_size = 100;
        d.draw_text("LOADING", (WIDTH-font_size*5)/2, (HEIGHT-font_size)/2, font_size, Color::WHITE);
    }

    let mut gamestate = gamestate::Gamestate::new(&mut rl, &thread).await;

    while !rl.window_should_close() {
        gamestate.parse_network(&mut rl, &thread).await;
        gamestate.update(&mut rl, &thread);
        gamestate.render(&mut rl, &thread);
    }
}
