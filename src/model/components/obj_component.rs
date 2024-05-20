use std::fmt::Display;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Copy)]
pub(crate) enum ObjType {
    Bird,
    Lizard,
    Dragon,
}

impl Display for ObjType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjType::Bird => write!(f, "Bird"),
            ObjType::Lizard => write!(f, "Lizard"),
            ObjType::Dragon => write!(f, "Dragon"),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ObjComponent {
    pub(crate) obj_id: String,
    pub(crate) parent_id: Option<String>,
    pub(crate) obj_type: ObjType,
}

impl ObjComponent {
    pub(crate) fn new_id() -> String {
        format!("{}", Uuid::new_v4())
    }
}
