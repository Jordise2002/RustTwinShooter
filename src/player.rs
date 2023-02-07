use macroquad::{window::*, shapes::draw_rectangle, prelude::*};
use crate::common_traits;
use crate::common_functions::{Position, normalize};
const MAX_SPEED:f32 = 5.;
const CROSSHAIR_DISTANCE:f32 = 30.;
const PLAYER_SIZE:f32 = 20.;
const CROSSHAIR_SIZE:f32 = 7.5;
pub struct Player
{
    pos:Position,
    acceleration_vector:Position,
    size:Position,
    crosshair:Crosshair
}

struct Crosshair
{
    pos:Position,
    size:Position,
    pub vector:Position
}

impl Crosshair
{
    pub fn new(pos:Position, size:Position) -> Self
    {
        Crosshair { pos: pos, size: size,vector:(0.,0.) }
    }

    fn draw(&self) {
        draw_rectangle(self.pos.0, self.pos.1, self.size.0, self.size.1, RED);
    }

    fn handle_movement(& mut self, player_position: Position) {
        let mouse_position = mouse_position();
        self.vector = (mouse_position.0 - player_position.0, mouse_position.1 - player_position.1);
        self.vector = normalize(self.vector);
        self.pos = (player_position.0 + self.vector.0 * CROSSHAIR_DISTANCE - self.size.0 / 2., player_position.1 + self.vector.1 * CROSSHAIR_DISTANCE - self.size.1 / 2.);
    }
}

impl Player
{
    pub fn new() -> Self
    {
        Player
        {
            pos:(screen_width()/2.,screen_height()/2.),
            size:(PLAYER_SIZE,PLAYER_SIZE),
            acceleration_vector:(0.,0.),
            crosshair: Crosshair::new((screen_width()/2., screen_height()/2.),(CROSSHAIR_SIZE, CROSSHAIR_SIZE))
        }
    }
    fn get_center(&self) -> Position
    {
        (self.pos.0 + self.size.0 / 2., self.pos.1 + self.size.1 / 2.)
    }

    pub fn get_projectile_start_position(&self) -> (f32, f32, f32 ,f32)
    {
        let v = normalize(self.crosshair.vector);
        let center = self.get_center();
        (center.0 + v.0 * CROSSHAIR_DISTANCE, center.1 + v.1 * CROSSHAIR_DISTANCE, v.0, v.1)
    }
    pub fn handle_movement(&mut self)
    {
        self.acceleration_vector = (0., 0.);
        if is_key_down(KeyCode::A)
        {
            self.acceleration_vector.0 -= 1.;
        }

        if is_key_down(KeyCode::D)
        {
            self.acceleration_vector.0 += 1.;
        }

        if is_key_down(KeyCode::W)
        {
            self.acceleration_vector.1 -= 1.;
        }

        if is_key_down(KeyCode::S)
        {
            self.acceleration_vector.1 += 1.;
        }

        self.acceleration_vector = normalize(self.acceleration_vector);
    }
}

impl common_traits::EntityTrait for Player
{
    fn draw(&self)
    {
        draw_rectangle(self.pos.0, self.pos.1, self.size.0, self.size.1, BLUE);
        self.crosshair.draw();
    }

    fn move_entity(& mut self) {
        let aux_x = self.pos.0 + self.acceleration_vector.0 * MAX_SPEED;
        let aux_y = self.pos.1 + self.acceleration_vector.1 * MAX_SPEED;

        if aux_x < 0. 
        {
            self.pos.0 = 0.
        }
        else if aux_x > screen_width() - self.size.0
        {
            self.pos.0 = screen_width() - self.size.0;
        }
        else 
        {
            self.pos.0 = aux_x;
        }
        
        if aux_y < 0. 
        {
            self.pos.1 = 0.
        }
        else if aux_y > screen_height() - self.size.1
        {
            self.pos.1 = screen_height() - self.size.1;
        }
        else 
        {
            self.pos.1 = aux_y;
        }
        self.crosshair.handle_movement(self.get_center());
    }

    fn get_type(&self) -> crate::common_traits::EntityTipes {
        crate::common_traits::EntityTipes::Player
    }

    fn get_x_range(&self) -> (f32, f32) {
        (self.pos.0, self.pos.0 + self.size.0)
    }

    fn get_y_range(&self) -> (f32,f32) {
        (self.pos.1, self.pos.1 + self.size.1)
    }
}