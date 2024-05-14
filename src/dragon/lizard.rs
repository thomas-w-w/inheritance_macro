use crate::dragon::bird::animal::obj::*;
use crate::dragon::bird::animal::*;

#[derive(Clone)]
pub(crate) struct LizardArchetype {
    pub(crate) animal: AnimalArchetype,
    pub(crate) eggs: u32,
}

impl LizardArchetype {
    pub(crate) fn crawl(&self) {
        println!("LizardArchetype::crawl");
    }

    pub(crate) fn try_reproduce(&mut self) -> Option<LizardArchetype> {
        if self.eggs > 0 {
            self.animal
                .calories
                .checked_sub(50)
                .map(|remaining_calories| {
                    self.animal.calories = remaining_calories;
                    self.eggs -= 1;
                    LizardArchetype {
                        animal: self.animal.clone(),
                        eggs: self.eggs,
                    }
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
}

impl Lizard {
    pub fn new(lizard: LizardArchetype) -> Self {
        Self { lizard }
    }
}

impl ObjTrait for Lizard {}

impl AnimalTrait for Lizard {
    type Offspring = Lizard;
    fn eat(&mut self, calories: u32) {
        self.lizard.animal.eat(calories)
    }
    fn try_reproduce(&mut self) -> Option<Self::Offspring> {
        self.lizard
            .try_reproduce()
            .map(|lizard| Self::Offspring { lizard })
    }
}

impl LizardTrait for Lizard {
    fn crawl(&self) {
        self.lizard.crawl()
    }
}

pub fn lizard_main() {
    let mut lizard = Lizard::new(LizardArchetype {
        animal: AnimalArchetype {
            obj: ObjArchetype {
                obj_id: "1".to_string(),
                obj_type: ObjType::Lizard,
            },
            calories: 10,
        },
        eggs: 3,
    });
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
