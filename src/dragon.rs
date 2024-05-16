pub(crate) mod bird;
pub(crate) mod lizard;

use crate::dragon::bird::animal::obj::*;
use crate::dragon::bird::animal::*;

use bird::BirdComponent;
use bird::BirdTrait;

use lizard::LizardComponent;
use lizard::LizardTrait;

#[derive(Debug)]
struct DragonComponent {
    etanol_liters: u32,
}

impl DragonComponent {
    fn fire(&self) {
        println!("DragonArchetype::fire");
    }
    fn try_reproduce(
        &mut self,
        bird: &mut BirdComponent,
        lizard: &mut LizardComponent,
        animal: &mut AnimalComponent,
    ) -> Option<DragonComponent> {
        if bird.eggs > 0 {
            animal.calories.checked_sub(50).map(|remaining_calories| {
                animal.calories = remaining_calories;
                bird.eggs -= 1;
                DragonComponent {
                    etanol_liters: self.etanol_liters.clone(),
                }
            })
        } else {
            None
        }
    }
}

trait DragonTrait: BirdTrait + LizardTrait {
    fn fire(&self);
}

#[derive(Debug)]
struct DragonArchetype {
    dragon: DragonComponent,
    bird: BirdComponent,
    lizard: LizardComponent,
    animal: AnimalComponent,
    obj: ObjComponent,
}

impl DragonArchetype {
    pub fn new(
        dragon: DragonComponent,
        bird: BirdComponent,
        lizard: LizardComponent,
        animal: AnimalComponent,
        obj: ObjComponent,
    ) -> Self {
        Self {
            dragon,
            bird,
            lizard,
            animal,
            obj,
        }
    }
}

impl ObjTrait for DragonArchetype {}

impl AnimalTrait for DragonArchetype {
    type Offspring = DragonArchetype;
    fn eat(&mut self, calories: u32) {
        self.animal.eat(calories)
    }

    fn try_reproduce(&mut self) -> Option<Self::Offspring> {
        self.dragon
            .try_reproduce(&mut self.bird, &mut self.lizard, &mut self.animal)
            .map(|dragon: DragonComponent| Self::Offspring {
                dragon,
                bird: self.bird.clone(),
                lizard: self.lizard.clone(),
                animal: self.animal.clone(),
                obj: ObjComponent {
                    obj_id: ObjComponent::new_id(),
                    parent_id: Some(self.obj.obj_id.clone()),
                    obj_type: ObjType::Dragon,
                },
            })
    }
}

impl BirdTrait for DragonArchetype {
    fn peep(&self) {
        self.bird.peep()
    }
}

impl LizardTrait for DragonArchetype {
    fn crawl(&self) {
        self.lizard.crawl();
    }
}

impl DragonTrait for DragonArchetype {
    fn fire(&self) {
        self.dragon.fire();
    }
}

pub fn dragon_main() {
    let mut dragon = DragonArchetype::new(
        DragonComponent {
            etanol_liters: 1000,
        },
        BirdComponent { eggs: 3 },
        LizardComponent { eggs: 3 },
        AnimalComponent { calories: 10 },
        ObjComponent {
            obj_id: ObjComponent::new_id(),
            parent_id: None,
            obj_type: ObjType::Dragon,
        },
    );
    dragon.fire();
    dragon.eat(50);
    dragon.peep();
    dragon.crawl();
    println!("\r\nDragon: {:?}", dragon);
    dragons_only(&dragon);
    if let Some(mut new_dragon) = dragon.try_reproduce() {
        new_dragon.eat(50);
        println!("\r\nChild dragon: {:?}", new_dragon);
        dragons_only(&new_dragon);
        if let Some(mut new_new_dragon) = new_dragon.try_reproduce() {
            new_new_dragon.eat(50);
            dragons_only(&new_dragon);
            println!("\r\nChild dragon: {:?}", new_dragon);
            if let Some(mut new_new_new_dragon) = new_new_dragon.try_reproduce() {
                new_new_dragon.eat(50);
                dragons_only(&new_new_new_dragon);
                println!("\r\nGrand grand child dragon: {:?}", new_new_new_dragon);
                if let Some(mut new_new_new_new_dragon) = new_new_new_dragon.try_reproduce() {
                    new_new_new_new_dragon.eat(50);
                    dragons_only(&new_new_new_new_dragon);
                    println!(
                        "\r\nGrand grand grand child dragon: {:?}",
                        new_new_new_new_dragon
                    );
                } else {
                    println!(
                        "\r\nFail reproduce dragon {}",
                        new_new_new_dragon.obj.obj_id.clone()
                    );
                }
            } else {
                println!(
                    "\r\nFail reproduce dragon {}",
                    new_new_dragon.obj.obj_id.clone()
                );
            }
        } else {
            println!(
                "\r\nFail reproduce dragon {}",
                new_dragon.obj.obj_id.clone()
            );
        }
    } else {
        println!("\r\nFail reproduce dragon {}", dragon.obj.obj_id.clone());
    }
}

fn dragons_only(dragon: &impl DragonTrait) {
    dragon.fire();
}
