use crate::Id;

#[derive(Debug, Clone)]
pub struct Actor {
    id: Id,
    gender: Gender,
    short_description: String,
    keywords: Vec<String>,
}

impl Actor {
    pub fn id(&self) -> Id {
        self.id
    }

    pub fn gender(&self) -> Gender {
        self.gender
    }

    pub fn short_desc_the(&self) -> String {
        format!("the {}", &self.short_description)
    }

    pub fn short_desc_a(&self) -> String {
        format!("a {}", &self.short_description)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Gender {
    NonBinary,
    Male,
    Female,
}
