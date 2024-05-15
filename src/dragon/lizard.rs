use crate::dragon::bird::animal::obj::*;
use crate::dragon::bird::animal::*;

#[derive(Clone)]
pub(crate) struct LizardArchetype {
    pub(crate) eggs: u32,
}

impl LizardArchetype {
    pub(crate) fn crawl(&self) {
        println!("LizardArchetype::crawl");
    }

    pub(crate) fn try_reproduce(
        &mut self,
        animal: &mut AnimalArchetype,
    ) -> Option<LizardArchetype> {
        if self.eggs > 0 {
            animal.calories.checked_sub(50).map(|remaining_calories| {
                animal.calories = remaining_calories;
                self.eggs -= 1;
                LizardArchetype { eggs: self.eggs }
            })
        } else {
            None
        }
    }
}

pub(crate) trait LizardTrait: AnimalTrait {
    fn crawl(&self);
}

struct Lizard {
    lizard: LizardArchetype,
    animal: AnimalArchetype,
    obj: ObjArchetype,
}

impl Lizard {
    pub fn new(lizard: LizardArchetype, animal: AnimalArchetype, obj: ObjArchetype) -> Self {
        Self {
            lizard,
            animal,
            obj,
        }
    }
}

impl ObjTrait for Lizard {}

impl AnimalTrait for Lizard {
    type Offspring = Lizard;
    fn eat(&mut self, calories: u32) {
        self.animal.eat(calories)
    }
    fn try_reproduce(&mut self) -> Option<Self::Offspring> {
        self.lizard
            .try_reproduce(&mut self.animal)
            .map(|lizard| Self::Offspring {
                lizard: lizard,
                animal: self.animal.clone(),
                obj: self.obj.clone(),
            })
    }
}

impl LizardTrait for Lizard {
    fn crawl(&self) {
        self.lizard.crawl()
    }
}

pub fn lizard_main() {
    let mut lizard = Lizard::new(
        LizardArchetype { eggs: 3 },
        AnimalArchetype { calories: 10 },
        ObjArchetype {
            obj_id: "lizard#1".to_string(),
            obj_type: ObjType::Lizard,
        },
    );
    lizard.crawl();
    lizard.eat(50);
    if let Some(mut new_lizard) = lizard.try_reproduce() {
        lizards_only(&lizard);
        lizards_only(&new_lizard);
        new_lizard.eat(50);
    }
}

fn lizards_only(lizard: &impl LizardTrait) {
    lizard.crawl();
}
