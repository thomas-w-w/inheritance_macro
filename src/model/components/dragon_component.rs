use std::sync::{Arc, Mutex};

use super::{animal_component::AnimalComponent, food_component::FoodComponent};

#[derive(Debug, Clone)]
pub(crate) struct DragonComponent {
    pub(crate) etanol_liters: u32,
}

impl DragonComponent {
    pub(crate) fn fire(
        &mut self,
        animal: &mut AnimalComponent,
        shared_food: Arc<Mutex<FoodComponent>>,
    ) -> bool {
        let mut fire_capacity = self.etanol_liters.clone();
        //need 10+ fire to dire
        while fire_capacity < 10 {
            if animal.calories > 20 {
                if animal.calories >= 20 {
                    self.etanol_liters += 10;
                    animal.calories -= 20;
                }
                fire_capacity = self.etanol_liters.clone();
                break;
            }

            let ate = animal.eat(Arc::clone(&shared_food), 10);
            if !ate {
                break;
            } else {
                // 20 food reserve => 10 fire capacity
                //1 fire = 2 food
                if animal.calories >= 20 {
                    self.etanol_liters += 10;
                    animal.calories -= 20;
                }
                fire_capacity = self.etanol_liters.clone();
            }
        }

        if fire_capacity >= 10 {
            self.etanol_liters = fire_capacity - 10;
            return true;
        }
        false
    }
}
