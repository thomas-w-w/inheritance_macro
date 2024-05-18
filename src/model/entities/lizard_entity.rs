use crate::model::components::{
    animal_component::AnimalComponent,
    egglaying_animal_component::{EgglayingAnimalComponent, INIT_EGGS},
    lizard_component::LizardComponent,
    obj_component::{ObjComponent, ObjType},
};
use crate::model::traits::{
    animal_trait::AnimalTrait, egglaying_animal_trait::EgglayingAnimalTrait,
    lizard_trait::LizardTrait, obj_trait::ObjTrait,
};

#[derive(Debug)]
struct LizardEntity {
    lizard: LizardComponent,
    egg_laying_animal: EgglayingAnimalComponent,
    animal: AnimalComponent,
    obj: ObjComponent,
}

impl LizardEntity {
    pub fn new(
        lizard: LizardComponent,
        egg_laying_animal: EgglayingAnimalComponent,
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

impl ObjTrait for LizardEntity {}

impl AnimalTrait for LizardEntity {
    type Offspring = LizardEntity;
    fn eat(&mut self, calories: u32) {
        self.animal.eat(calories)
    }

    fn try_reproduce(&mut self) -> Option<Self::Offspring> {
        self.egg_laying_animal
            .try_reproduce(&mut self.animal)
            .map(|egg_laying_animal| Self::Offspring {
                lizard: self.lizard.clone(),
                egg_laying_animal,
                animal: self.animal.clone(),
                obj: ObjComponent {
                    obj_id: ObjComponent::new_id(),
                    parent_id: Some(self.obj.obj_id.clone()),
                    obj_type: ObjType::Bird,
                },
            })
    }
}

impl EgglayingAnimalTrait for LizardEntity {}

impl LizardTrait for LizardEntity {
    fn crawl(&self) {
        self.lizard.crawl()
    }
}

pub fn lizard_main() {
    let mut lizard = LizardEntity::new(
        LizardComponent {},
        EgglayingAnimalComponent { eggs: INIT_EGGS },
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
