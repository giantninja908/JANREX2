mod gamestate;
mod gui;
mod key_rotate;
mod packet_sender;
mod token_fetch;
pub(crate) use raylib::prelude::*;

#[tokio::main]
async fn main() {
    let (mut rl, thread) = raylib::init()
        .msaa_4x()
        .size(1920, 1080)
        .title("JANREX 2")
        .resizable()
        .build();

    // rl.set_target_fps(120);
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
        d.draw_text("LOADING", 10, 10, 50, Color::WHITE);
    }

    let mut gamestate = gamestate::Gamestate::new(&mut rl, &thread).await;

    while !rl.window_should_close() {
        gamestate.parse_network(&mut rl, &thread).await;
        gamestate.update(&mut rl, &thread);
        gamestate.render(&mut rl, &thread);
    }
}
