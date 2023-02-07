pub type Position = (f32, f32);

pub fn normalize(pos:Position) -> Position
{
    let mut aux = (pos.0.powi(2) + pos.1.powi(2)).sqrt();
    if aux == 0.
    {
        aux = 1.;
    }
    (pos.0 / aux, pos.1 /aux)
}
