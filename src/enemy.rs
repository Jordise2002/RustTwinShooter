use macroquad::{shapes::draw_rectangle, prelude::ORANGE};

use crate::{common_functions::Position, common_traits::EntityTrait};
const ENEMY_SIZE:f32 = 15.;
pub struct Enemy
{
    pub id:u32,
    size:Position,
    pos:Position
}

impl Enemy
{
   pub fn new(pos: Position, id: u32) -> Self
   {
        Enemy { size: (ENEMY_SIZE, ENEMY_SIZE), pos: pos, id:id }
   } 
}

impl EntityTrait for Enemy
{
    fn draw(&self) {
        draw_rectangle(self.pos.0, self.pos.1, self.size.0, self.size.1, ORANGE);
    }

    fn move_entity(& mut self) {
        
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

