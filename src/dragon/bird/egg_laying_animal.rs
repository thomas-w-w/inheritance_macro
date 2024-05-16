use crate::dragon::bird::animal::{AnimalComponent, AnimalTrait};

pub(crate) const INIT_EGGS: u32 = 3;

#[derive(Clone, Debug)]
pub(crate) struct EggLayingAnimalComponent {
    pub(crate) eggs: u32,
}
impl EggLayingAnimalComponent {
    pub(crate) fn try_reproduce(
        &mut self,
        animal: &mut AnimalComponent,
    ) -> Option<EggLayingAnimalComponent> {
        if self.eggs > 0 {
            animal.calories.checked_sub(50).map(|remaining_calories| {
                animal.calories = remaining_calories;
                self.eggs -= 1;
                EggLayingAnimalComponent { eggs: INIT_EGGS }
            })
        } else {
            None
        }
    }
}

pub(crate) trait EggLayingAnimalTrait: AnimalTrait {}
