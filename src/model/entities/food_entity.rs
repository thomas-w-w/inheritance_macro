use crate::model::components::food_component::FoodComponent;
use crate::model::traits::food_trait::FoodTrait;
use crate::model::traits::obj_trait::ObjTrait;

#[derive(Debug)]
pub(crate) struct FoodEntity {
    food: FoodComponent,
}

impl FoodEntity {
    pub fn new(food: FoodComponent) -> Self {
        Self { food }
    }
}

impl ObjTrait for FoodEntity {}

impl FoodTrait for FoodEntity {
    fn eat(&mut self, chunk: i32) -> bool {
        self.food.eat(chunk)
    }
}
