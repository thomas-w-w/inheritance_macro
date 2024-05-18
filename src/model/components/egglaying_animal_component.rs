use crate::model::components::animal_component::AnimalComponent;

pub(crate) const INIT_EGGS: u32 = 3;

#[derive(Clone, Debug)]
pub(crate) struct EgglayingAnimalComponent {
    pub(crate) eggs: u32,
}
impl EgglayingAnimalComponent {
    pub(crate) fn try_reproduce(
        &mut self,
        animal: &mut AnimalComponent,
    ) -> Option<EgglayingAnimalComponent> {
        if self.eggs > 0 {
            animal.calories.checked_sub(50).map(|remaining_calories| {
                animal.calories = remaining_calories;
                self.eggs -= 1;
                EgglayingAnimalComponent { eggs: INIT_EGGS }
            })
        } else {
            None
        }
    }
}
