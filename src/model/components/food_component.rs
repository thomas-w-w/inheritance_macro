#[derive(Clone, Debug)]
pub(crate) struct FoodComponent {
    pub food_capacity: i32,
}

impl FoodComponent {
    pub(crate) fn eat(&mut self, chunk: i32) -> bool {
        if self.food_capacity > chunk {
            self.food_capacity -= chunk;
            return true;
        }
        false
    }
}
