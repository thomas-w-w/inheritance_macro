#[derive(Clone)]
struct Animal {
    calories: u32,
}

trait IAnimal {
    fn eat(&mut self, calories: u32);
}

impl IAnimal for Animal {
    fn eat(&mut self, calories: u32) {
        self.calories += calories;
    }
}

trait IBird: IAnimal {
    type Offspring;
    fn peep(&self);
    fn try_reproduce(&mut self) -> Option<Self::Offspring>;
}

struct BirdArchetype {
    eggs: u32,
}

impl BirdArchetype {
    fn peep(&self) {
        println!("BirdArchetype::peep");
    }
    fn try_reproduce(&mut self, animal: &mut Animal) -> Option<BirdArchetype> {
        if self.eggs > 0 {
            animal.calories.checked_sub(50).map(|remaining_calories| {
                animal.calories = remaining_calories;
                self.eggs -= 1;
                BirdArchetype { eggs: self.eggs }
            })
        } else {
            None
        }
    }
}

struct Bird {
    animal: Animal,
    bird: BirdArchetype,
}

impl Bird {
    pub fn new(animal: Animal, bird: BirdArchetype) -> Self {
        Self { animal, bird }
    }
}

impl IAnimal for Bird {
    fn eat(&mut self, calories: u32) {
        self.animal.eat(calories)
    }
}

impl IBird for Bird {
    type Offspring = Bird;

    fn peep(&self) {
        println!("IBird for Bird::peep()");
        self.bird.peep()
    }

    fn try_reproduce(&mut self) -> Option<Self::Offspring> {
        self.bird
            .try_reproduce(&mut self.animal)
            .map(|bird| Self::Offspring {
                animal: self.animal.clone(),
                bird,
            })
    }
}

pub fn bird_main() {
    let mut bird = Bird::new(Animal { calories: 10 }, BirdArchetype { eggs: 3 });
    bird.peep();
    bird.eat(50);
    if let Some(mut new_bird) = bird.try_reproduce() {
        birds_only(&bird);
        birds_only(&new_bird);
        new_bird.eat(50);
    }
}

fn birds_only(bird: &impl IBird) {
    println!("birds_only::peep()");
    bird.peep();
}
