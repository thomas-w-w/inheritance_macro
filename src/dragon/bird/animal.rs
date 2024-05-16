pub(crate) mod obj;

use obj::ObjTrait;

#[derive(Clone, Debug)]
pub(crate) struct AnimalComponent {
    pub(crate) calories: u32,
}

impl AnimalComponent {
    pub(crate) fn eat(&mut self, calories: u32) {
        self.calories += calories;
    }
}

pub(crate) trait AnimalTrait: ObjTrait {
    type Offspring;
    fn eat(&mut self, calories: u32);
    fn try_reproduce(&mut self) -> Option<Self::Offspring>;
}
