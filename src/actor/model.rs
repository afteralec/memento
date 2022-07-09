#[derive(Debug, Copy, Clone)]
pub struct Actor {
    id: i64,
    gender: Gender,

}

#[derive(Debug, Copy, Clone)]
pub enum Gender {
    NonBinary,
    Male,
    Female,
}
