#[derive(Clone, Debug)]
pub(crate) struct FoodComponent {
    pub food_capacity: u32,
}

impl FoodComponent {
    pub(crate) fn eat(&mut self, chunk: u32) -> bool {
        if self.food_capacity > chunk {
            self.food_capacity -= chunk;
            return true;
        }
        false
    }
}
