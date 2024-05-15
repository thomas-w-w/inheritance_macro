use crate::dragon::bird::animal::obj::*;
use crate::dragon::bird::animal::*;

#[derive(Clone)]
pub(crate) struct LizardComponent {
    pub(crate) eggs: u32,
}

impl LizardComponent {
    pub(crate) fn crawl(&self) {
        println!("LizardArchetype::crawl");
    }

    pub(crate) fn try_reproduce(
        &mut self,
        animal: &mut AnimalComponent,
    ) -> Option<LizardComponent> {
        if self.eggs > 0 {
            animal.calories.checked_sub(50).map(|remaining_calories| {
                animal.calories = remaining_calories;
                self.eggs -= 1;
                LizardComponent { eggs: self.eggs }
            })
        } else {
            None
        }
    }
}

pub(crate) trait LizardTrait: AnimalTrait {
    fn crawl(&self);
}

struct LizardArchetype {
    lizard: LizardComponent,
    animal: AnimalComponent,
    obj: ObjComponent,
}

impl LizardArchetype {
    pub fn new(lizard: LizardComponent, animal: AnimalComponent, obj: ObjComponent) -> Self {
        Self {
            lizard,
            animal,
            obj,
        }
    }
}

impl ObjTrait for LizardArchetype {}

impl AnimalTrait for LizardArchetype {
    type Offspring = LizardArchetype;
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

impl LizardTrait for LizardArchetype {
    fn crawl(&self) {
        self.lizard.crawl()
    }
}

pub fn lizard_main() {
    let mut lizard = LizardArchetype::new(
        LizardComponent { eggs: 3 },
        AnimalComponent { calories: 10 },
        ObjComponent {
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
