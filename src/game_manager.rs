use macroquad::time;
use macroquad::rand as rand;
use macroquad::prelude::{is_mouse_button_pressed, WHITE};
use macroquad::text::draw_text;
use macroquad::window::{screen_width, screen_height};
use crate::enemy::{Enemy};
use crate::player::Player;
use crate::common_traits::{EntityTrait};
use crate::projectile::{Projectile};
use macroquad::input::MouseButton;

const HIT_INTERVAL:f64 = 0.5;
const COOLDOWN:f64 =  0.25;
const PLAYER_HEALTH:u32 = 3;
pub struct GameManager
{
    player: Player,
    enemies: Vec<Enemy>,
    bullets: Vec<Projectile>,
    current_enemy_id:u32,
    current_bullet_id:u32,
    player_health:u32,
    last_player_hit:f64,
    last_shot:f64,
    score:u32
}

impl GameManager
{
    pub fn new() -> Self
    {
        GameManager { player: Player::new(), enemies: vec![], bullets: vec![], current_enemy_id: 0, current_bullet_id:0,player_health:PLAYER_HEALTH,last_player_hit: time::get_time(), last_shot: time::get_time() - 0.5,score:0 }
    }

    pub fn handle_input(&mut self)
    {
        self.player.handle_movement();
        if is_mouse_button_pressed(MouseButton::Left) && time::get_time() - self.last_shot > COOLDOWN
        {
            self.last_shot = time::get_time();
            self.shoot();
        }
    }

    fn shoot(& mut self)
    {
        let pos_accel = self.player.get_projectile_start_position();
        self.bullets.push(Projectile::new((pos_accel.0, pos_accel.1), (pos_accel.2, pos_accel.3),self.current_bullet_id));
        self.current_bullet_id += 1;
    }

    pub fn draw_gm(&self) {
        self.player.draw();
        for enemy in &self.enemies
        {
            enemy.draw();
        }

        for bullet in &self.bullets
        {
            bullet.draw();
        }
        draw_text(&self.player_health.to_string(), 20., 20., 30., WHITE);
        draw_text(&self.score.to_string(), screen_width() - 40.,20.,30., WHITE)
    }
 
    fn spawn(&mut self)
    {
        self.current_enemy_id += 1;
        self.enemies.push(Enemy::new((rand::gen_range(0., screen_width()), rand::gen_range(0., screen_height())),self.current_enemy_id));
    }
    pub fn handle_spawning(&mut self)
    {
        if rand::gen_range(0, 500) > 495
        {
            self.spawn();
        }
    }
    pub fn move_entities(& mut self) {
        self.player.move_entity();
        for enemy in &mut self.enemies
        {
            enemy.move_entity();
        }
        for bullet in & mut self.bullets
        {
            bullet.move_entity();
        }
    }
    
    pub fn check_colisions(& mut self)
    {
        let mut deaths = vec![];
        for bullet in self.bullets.iter()
        {
            for enemy in & self.enemies
            {
                if bullet.check_colides(Box::new(enemy))
                {
                    deaths.push(enemy.id);
                    self.score = self.score +1 ;
                }
            }
        }

        for death in deaths.iter()
        {
            let mut index = 0;
            for enemy in &self.enemies
            {
                if *death == enemy.id
                {
                    break;
                }
                index += 1;
            }
            self.enemies.remove(index);
        }

        for enemy in &self.enemies
        {
            if enemy.check_colides(Box::new(&self.player))
            {
                if HIT_INTERVAL <= time::get_time() - self.last_player_hit
                {
                    self.player_health -= 1;
                    self.last_player_hit = time::get_time();
                }
                
            }
        }
    }

    pub fn remove_bullets(&mut self)
    {
        let mut to_remove = vec![];
        for bullet in &self.bullets
        {
            let x_range = bullet.get_x_range();
            let y_range = bullet.get_y_range();

            if x_range.0 < 0. || x_range.1 > screen_width() ||
            y_range.0 < 0. || y_range.1 > screen_width()
            {
                to_remove.push(bullet.id)
            } 
        }

        for remove in to_remove
        {
            let mut index:usize = 0;
            for bullet in &self.bullets
            {
                if remove == bullet.id
                {
                    break;
                }
                index += 1;
            }
            self.bullets.remove(index);
        }
    }
}
