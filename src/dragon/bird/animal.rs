pub(crate) mod obj;

use obj::{ObjArchetype, ObjTrait};

#[derive(Clone)]
pub(crate) struct AnimalArchetype {
    pub(crate) obj: ObjArchetype,
    pub(crate) calories: u32,
}

impl AnimalArchetype {
    pub(crate) fn eat(&mut self, calories: u32) {
        self.calories += calories;
    }
}

pub(crate) trait AnimalTrait: ObjTrait {
    type Offspring;
    fn eat(&mut self, calories: u32);
    fn try_reproduce(&mut self) -> Option<Self::Offspring>;
}

struct Animal {
    animal: AnimalArchetype,
}

impl ObjTrait for Animal {}

impl AnimalTrait for Animal {
    fn eat(&mut self, calories: u32) {
        self.animal.eat(calories);
    }

    type Offspring = Animal;

    fn try_reproduce(&mut self) -> Option<Self::Offspring> {
        todo!()
    }
}
