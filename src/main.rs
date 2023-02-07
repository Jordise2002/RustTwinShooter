use game_manager::GameManager;
use macroquad::{prelude::*};
use macroquad::rand as rand;
use instant as time;
mod player;
mod common_traits;
mod common_functions;
mod game_manager;
mod projectile;
mod enemy;

fn window_conf() -> Conf
{
    Conf{
        window_title: "Juego".to_owned(),
        window_height:750,
        window_width:750,
        window_resizable:false,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut game_manager = GameManager::new();
    rand::srand(time::now() as u64);
    loop {
        clear_background(GREEN);
        game_manager.handle_spawning();
        game_manager.handle_input();
        game_manager.move_entities();
        game_manager.check_colisions();
        game_manager.remove_bullets();
        game_manager.draw_gm(); 
        next_frame().await;
    }
}
