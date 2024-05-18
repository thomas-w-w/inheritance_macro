#[derive(Clone, Debug)]
pub(crate) struct AnimalComponent {
    pub(crate) calories: u32,
}

impl AnimalComponent {
    pub(crate) fn eat(&mut self, calories: u32) {
        self.calories += calories;
    }
}
