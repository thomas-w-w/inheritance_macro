use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ObjType {
    Obj,
    Food,
    Animal,
    Bird,
    Lizard,
    Dragon,
}

impl Display for ObjType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjType::Obj => write!(f, "Obj"),
            ObjType::Animal => write!(f, "Animal"),
            ObjType::Bird => write!(f, "Bird"),
            ObjType::Lizard => write!(f, "Lizard"),
            ObjType::Dragon => write!(f, "Dragon"),
            ObjType::Food => write!(f, "Food"),
        }
    }
}

#[derive(Clone)]
pub(crate) struct ObjArchetype {
    pub(crate) obj_id: String,
    pub(crate) obj_type: ObjType,
}

impl ObjArchetype {}

pub(crate) trait ObjTrait {}

struct Obj {}

impl ObjTrait for Obj {}
