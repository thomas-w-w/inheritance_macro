use std::sync::{Arc, Mutex};

use super::food_component::FoodComponent;

#[derive(Clone, Debug)]
pub(crate) struct AnimalComponent {
    pub(crate) calories: u32,
    pub(crate) given_name: String,
}

impl AnimalComponent {
    pub(crate) fn eat(&mut self, shared_food: Arc<Mutex<FoodComponent>>, calories: u32) -> bool {
        let mut food = shared_food.lock().unwrap();

        let food_capacity = food.food_capacity;

        if food_capacity >= calories {
            food.food_capacity -= calories;

            self.calories += calories;

            println!(
                "AnimalComponent::eat: {} ate {} calories. GLOBAL food_capacity: {:?}\r\n",
                self.given_name, calories, food.food_capacity
            );

            return true;
        }
        false
    }
}
