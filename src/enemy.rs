use macroquad::window::{screen_width, screen_height};
use macroquad::{shapes::draw_rectangle, prelude::ORANGE};
use macroquad::time;
use macroquad::rand;
use crate::common_functions;
use crate::{common_functions::Position, common_traits::EntityTrait};
const ENEMY_SIZE:f32 = 15.;
const ENEMY_SPEED:f32 = 5.;
pub struct Enemy
{
    pub id:u32,
    size:Position,
    pos:Position,
    accel:Position,
    last_change:f64,
    time_to_change:f64
}

impl Enemy
{
   pub fn new(pos: Position, id: u32) -> Self
   {
        Enemy { size: (ENEMY_SIZE, ENEMY_SIZE), pos: pos, id:id,accel:(0.,0.),last_change: time::get_time(), time_to_change: 0.}
   } 

   fn change_direction(&mut self)
   {
       self.accel.0 = rand::gen_range(-1., 1.);
       self.accel.1 = rand::gen_range(-1., 1.);
       
       self.accel = common_functions::normalize(self.accel);
       
       self.time_to_change = rand::gen_range(0.5, 4.);
       self.last_change = time::get_time();
   }
}

impl EntityTrait for Enemy
{
    fn draw(&self) {
        draw_rectangle(self.pos.0, self.pos.1, self.size.0, self.size.1, ORANGE);
    }

   fn move_entity(& mut self) {
        if time::get_time() > self.last_change + self.time_to_change
        {
            self.change_direction();
        }

        let aux_x = self.pos.0 + self.accel.0 * ENEMY_SPEED;
        let aux_y = self.pos.1 + self.accel.1 * ENEMY_SPEED;

        if aux_x < 0. 
        {
            self.pos.0 = 0.;
            self.accel.0 = -self.accel.0;
        }
        else if aux_x > screen_width() - self.size.0
        {
            self.pos.0 = screen_width() - self.size.0;
            self.accel.0 = - self.accel.0;
        }
        else 
        {
            self.pos.0 = aux_x;
        }
        
        if aux_y < 0. 
        {
            self.pos.1 = 0.;
            self.accel.1 = -self.accel.1;
        }
        else if aux_y > screen_height() - self.size.1
        {
            self.pos.1 = screen_height() - self.size.1;
            self.accel.1 = -self.accel.1;
        }
        else 
        {
            self.pos.1 = aux_y;
        }
    }

    fn get_type(&self) -> crate::common_traits::EntityTipes {
        crate::common_traits::EntityTipes::Enemy
    }

    fn get_x_range(&self) -> (f32, f32) {
        (self.pos.0, self.pos.0 + self.size.0)
    }

    fn get_y_range(&self) -> (f32,f32) {
        (self.pos.1, self.pos.1 + self.size.1)
    }
}

