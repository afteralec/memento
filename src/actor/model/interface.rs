use super::proxy::ActorProxy;
use crate::{messaging::traits::ProvideProxy, Id};

#[derive(Debug)]
pub struct Actor {
    id: Id,
    gender: Gender,
    short_description: String,
    keywords: Vec<String>,
    last_room_id: Option<Id>,
}

impl ProvideProxy<ActorProxy> for Actor {}

impl Actor {
    pub fn new(
        id: i64,
        gender: &str,
        short_description: &str,
        keywords: &Vec<String>,
        last_room_id: Option<Id>,
    ) -> Self {
        let gender = match &gender.to_lowercase()[..] {
            "nonbinary" => Gender::NonBinary,
            "male" => Gender::Male,
            "female" => Gender::Female,
            _ => {
                // @TODO: Move this panic into actor loading and have the constructor require a valid Gender enum
                panic!(
                    "invalid gender at creation of actor with id: {}, got invalid string: {}",
                    id, gender
                );
            }
        };

        Actor {
            id: Id(id),
            gender,
            short_description: short_description.to_owned(),
            keywords: keywords.to_owned(),
            last_room_id,
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn gender(&self) -> Gender {
        self.gender
    }

    pub fn short_description(&self) -> String {
        self.short_description.clone()
    }

    pub fn keywords(&self) -> Vec<String> {
        self.keywords.clone()
    }

    pub fn last_room_id(&self) -> Option<Id> {
        self.last_room_id
    }

    pub fn self_gendered(&self) -> String {
        match self.gender {
            Gender::Male => "himself",
            Gender::Female => "herself",
            Gender::NonBinary => "themself",
        }
        .to_owned()
    }

    pub fn possessive_gendered(&self) -> String {
        match self.gender {
            Gender::Male => "his",
            Gender::Female => "her",
            Gender::NonBinary => "their",
        }
        .to_owned()
    }

    pub fn short_desc_the(&self) -> String {
        format!("the {}", &self.short_description)
    }

    pub fn short_desc_a(&self) -> String {
        format!("a {}", &self.short_description)
    }

    pub fn keyword_iter(&self) -> std::slice::Iter<String> {
        self.keywords.iter()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Gender {
    NonBinary,
    Male,
    Female,
}
