#[derive(Clone, Copy)]
pub enum Square {
    Solid,
    Empty,
    Letter(char),
}
