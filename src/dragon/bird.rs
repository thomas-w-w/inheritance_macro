pub(crate) mod animal;

use crate::dragon::bird::animal::obj::*;
use animal::{AnimalComponent, AnimalTrait};

#[derive(Clone, Debug)]
pub(crate) struct BirdComponent {
    pub(crate) eggs: u32,
}
impl BirdComponent {
    pub(crate) fn peep(&self) {
        println!("BirdArchetype::peep");
    }

    pub(crate) fn try_reproduce(&mut self, animal: &mut AnimalComponent) -> Option<BirdComponent> {
        if self.eggs > 0 {
            animal.calories.checked_sub(50).map(|remaining_calories| {
                animal.calories = remaining_calories;
                self.eggs -= 1;
                BirdComponent { eggs: self.eggs }
            })
        } else {
            None
        }
    }
}

pub(crate) trait BirdTrait: AnimalTrait {
    fn peep(&self);
}

#[derive(Debug)]
struct BirdArchetype {
    bird: BirdComponent,
    animal: AnimalComponent,
    obj: ObjComponent,
}

impl BirdArchetype {
    pub fn new(bird: BirdComponent, animal: AnimalComponent, obj: ObjComponent) -> Self {
        Self { bird, animal, obj }
    }
}

impl ObjTrait for BirdArchetype {}

impl AnimalTrait for BirdArchetype {
    type Offspring = BirdArchetype;
    fn eat(&mut self, calories: u32) {
        self.animal.eat(calories)
    }
    fn try_reproduce(&mut self) -> Option<Self::Offspring> {
        self.bird
            .try_reproduce(&mut self.animal)
            .map(|bird| Self::Offspring {
                bird: bird,
                animal: self.animal.clone(),
                obj: ObjComponent {
                    obj_id: ObjComponent::new_id(),
                    parent_id: Some(self.obj.obj_id.clone()),
                    obj_type: ObjType::Bird,
                },
            })
    }
}

impl BirdTrait for BirdArchetype {
    fn peep(&self) {
        self.bird.peep()
    }
}

pub fn bird_main() {
    let mut bird = BirdArchetype::new(
        BirdComponent { eggs: 3 },
        AnimalComponent { calories: 10 },
        ObjComponent {
            obj_id: ObjComponent::new_id(),
            parent_id: None,
            obj_type: ObjType::Bird,
        },
    );
    bird.peep();
    bird.eat(50);
    birds_only(&bird);
    println!("\r\nBird: {:?}", bird);
    if let Some(mut new_bird) = bird.try_reproduce() {
        birds_only(&new_bird);
        new_bird.eat(50);
        println!("\r\nChild bird: {:?}", new_bird);
    }
}

fn birds_only(bird: &impl BirdTrait) {
    bird.peep();
}
