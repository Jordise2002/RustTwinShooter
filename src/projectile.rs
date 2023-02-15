use macroquad::prelude::{PURPLE};
use macroquad::shapes::draw_rectangle;

use crate::common_functions::Position;
use crate::common_traits::EntityTrait;
const PROJECTILE_SPEED:f32 = 6.;
const PROJECTILE_SIZE:f32 = 10.;
pub struct Projectile 
{
    size:Position,
    pos: Position,
    acceleration_vector: Position,
    pub id:u32
}

impl Projectile
{
    pub fn new(pos: Position, accel: Position, id:u32) -> Self
    {
        Projectile
        {
            size:(PROJECTILE_SIZE, PROJECTILE_SIZE),
            pos:pos,
            acceleration_vector: accel,
            id:id
        }
    }
}
impl EntityTrait for Projectile
{
    fn draw(&self) {
        draw_rectangle(self.pos.0, self.pos.1, self.size.0, self.size.1, PURPLE);
    }

    fn move_entity(& mut self) {
        
        self.pos = (self.pos.0 + self.acceleration_vector.0 * PROJECTILE_SPEED, self.pos.1 + self.acceleration_vector.1 * PROJECTILE_SPEED);
    }

    fn get_type(&self) -> crate::common_traits::EntityTipes {
        crate::common_traits::EntityTipes::Projectile
    }

    fn get_x_range(&self) -> (f32, f32) {
        (self.pos.0, self.pos.0 + self.size.0)
    }

    fn get_y_range(&self) -> (f32,f32) {
        (self.pos.1, self.pos.1 + self.size.1)
    }
}