pub(crate) mod animal;

use crate::dragon::bird::animal::obj::*;
use animal::{AnimalArchetype, AnimalTrait};

#[derive(Clone)]
pub(crate) struct BirdArchetype {
    pub(crate) eggs: u32,
}

impl BirdArchetype {
    pub(crate) fn peep(&self) {
        println!("BirdArchetype::peep");
    }

    pub(crate) fn try_reproduce(&mut self, animal: &mut AnimalArchetype) -> Option<BirdArchetype> {
        if self.eggs > 0 {
            animal.calories.checked_sub(50).map(|remaining_calories| {
                animal.calories = remaining_calories;
                self.eggs -= 1;
                BirdArchetype { eggs: self.eggs }
            })
        } else {
            None
        }
    }
}

pub(crate) trait BirdTrait: AnimalTrait {
    fn peep(&self);
}

struct Bird {
    bird: BirdArchetype,
    animal: AnimalArchetype,
    obj: ObjArchetype,
}

impl Bird {
    pub fn new(bird: BirdArchetype, animal: AnimalArchetype, obj: ObjArchetype) -> Self {
        Self { bird, animal, obj }
    }
}

impl ObjTrait for Bird {}

impl AnimalTrait for Bird {
    type Offspring = Bird;
    fn eat(&mut self, calories: u32) {
        self.animal.eat(calories)
    }
    fn try_reproduce(&mut self) -> Option<Self::Offspring> {
        self.bird
            .try_reproduce(&mut self.animal)
            .map(|bird| Self::Offspring {
                bird: bird,
                animal: self.animal.clone(),
                obj: self.obj.clone(),
            })
    }
}

impl BirdTrait for Bird {
    fn peep(&self) {
        self.bird.peep()
    }
}

pub fn bird_main() {
    let mut bird = Bird::new(
        BirdArchetype { eggs: 3 },
        AnimalArchetype { calories: 10 },
        ObjArchetype {
            obj_id: "bird#1".to_string(),
            obj_type: ObjType::Bird,
        },
    );
    bird.peep();
    bird.eat(50);
    if let Some(mut new_bird) = bird.try_reproduce() {
        birds_only(&bird);
        birds_only(&new_bird);
        new_bird.eat(50);
    }
}

fn birds_only(bird: &impl BirdTrait) {
    bird.peep();
}
