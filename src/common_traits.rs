pub trait EntityTrait
{
    fn draw(&self);

    fn move_entity(& mut self);

    fn get_type(&self) -> EntityTipes;

    fn get_x_range(&self) -> (f32, f32);

    fn get_y_range(&self) -> (f32,f32);

    fn check_colides(&self, other: Box<&dyn EntityTrait>) -> bool
    {
        let my_x = self.get_x_range();
        let my_y = self.get_y_range();

        let other_x = other.get_x_range();
        let other_y = other.get_y_range();

        if my_x.1 >= other_x.0 &&
        my_x.0 <= other_x.1 &&
        my_y.1 >= other_y.0 &&
        my_y.0 <= other_y.1  {
            true
        }
        else {
            false
        } 
    }
}
pub enum EntityTipes
{
    Projectile,
    Player,
    Enemy
}