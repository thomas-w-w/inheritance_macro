use crate::dragon::bird::animal::obj::*;
use crate::dragon::bird::animal::*;
use crate::egg_laying_animal::{EggLayingAnimalComponent, EggLayingAnimalTrait, INIT_EGGS};

#[derive(Clone, Debug)]
pub(crate) struct LizardComponent {}

impl LizardComponent {
    pub(crate) fn crawl(&self) {
        println!("LizardArchetype::crawl");
    }

    pub(crate) fn try_reproduce(
        &mut self,
        egg_laying_animal: &mut EggLayingAnimalComponent,
        animal: &mut AnimalComponent,
    ) -> Option<LizardComponent> {
        if egg_laying_animal.eggs > 0 {
            animal.calories.checked_sub(50).map(|remaining_calories| {
                animal.calories = remaining_calories;
                egg_laying_animal.eggs -= 1;
                LizardComponent {}
            })
        } else {
            None
        }
    }
}

pub(crate) trait LizardTrait: EggLayingAnimalTrait {
    fn crawl(&self);
}

#[derive(Debug)]
struct LizardArchetype {
    lizard: LizardComponent,
    egg_laying_animal: EggLayingAnimalComponent,
    animal: AnimalComponent,
    obj: ObjComponent,
}

impl LizardArchetype {
    pub fn new(
        lizard: LizardComponent,
        egg_laying_animal: EggLayingAnimalComponent,
        animal: AnimalComponent,
        obj: ObjComponent,
    ) -> Self {
        Self {
            lizard,
            egg_laying_animal,
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
            .try_reproduce(&mut self.egg_laying_animal, &mut self.animal)
            .map(|lizard| Self::Offspring {
                lizard: lizard,
                egg_laying_animal: EggLayingAnimalComponent { eggs: INIT_EGGS },
                animal: self.animal.clone(),
                obj: ObjComponent {
                    obj_id: ObjComponent::new_id(),
                    parent_id: Some(self.obj.obj_id.clone()),
                    obj_type: ObjType::Lizard,
                },
            })
    }
}

impl EggLayingAnimalTrait for LizardArchetype {}

impl LizardTrait for LizardArchetype {
    fn crawl(&self) {
        self.lizard.crawl()
    }
}

pub fn lizard_main() {
    let mut lizard = LizardArchetype::new(
        LizardComponent {},
        EggLayingAnimalComponent { eggs: INIT_EGGS },
        AnimalComponent { calories: 10 },
        ObjComponent {
            obj_id: ObjComponent::new_id(),
            parent_id: None,
            obj_type: ObjType::Lizard,
        },
    );
    lizard.crawl();
    lizard.eat(50);
    lizards_only(&lizard);
    println!("\r\nLizard: {:?}", lizard);
    if let Some(mut new_lizard) = lizard.try_reproduce() {
        new_lizard.eat(50);
        lizards_only(&new_lizard);
        println!("\r\nChild lizard: {:?}", new_lizard);
    }
}

fn lizards_only(lizard: &impl LizardTrait) {
    lizard.crawl();
}
