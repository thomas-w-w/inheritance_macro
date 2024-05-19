use std::sync::{Arc, Mutex};

use super::food_component::FoodComponent;

#[derive(Clone, Debug)]
pub(crate) struct AnimalComponent {
    pub(crate) calories: u32,
}

impl AnimalComponent {
    pub(crate) fn eat(&mut self, shared_food: Arc<Mutex<FoodComponent>>, calories: u32) -> bool {
        let mut food = shared_food.lock().unwrap();

        let mut food_capacity = food.food_capacity;

        if food_capacity >= 100 {
            food.food_capacity -= 100;

            food_capacity = food.food_capacity;

            self.calories += 100;

            return true;
        }
        false
    }
}
