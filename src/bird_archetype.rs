#[derive(Clone)]
struct AnimalArchetype {
    calories: u32,
}

impl AnimalArchetype {
    fn eat(&mut self, calories: u32) {
        self.calories += calories;
    }
}

struct Animal {
    animal: AnimalArchetype,
}

trait IAnimal {
    fn eat(&mut self, calories: u32);
}

impl IAnimal for Animal {
    fn eat(&mut self, calories: u32) {
        self.animal.eat(calories);
    }
}

trait IBird: IAnimal {
    type Offspring;
    fn peep(&self);
    fn try_reproduce(&mut self) -> Option<Self::Offspring>;
}

struct BirdArchetype {
    animal: AnimalArchetype,
    eggs: u32,
}

impl BirdArchetype {
    fn peep(&self) {
        println!("BirdArchetype::peep");
    }
    fn try_reproduce(&mut self) -> Option<BirdArchetype> {
        if self.eggs > 0 {
            self.animal
                .calories
                .checked_sub(50)
                .map(|remaining_calories| {
                    self.animal.calories = remaining_calories;
                    self.eggs -= 1;
                    BirdArchetype {
                        animal: self.animal.clone(),
                        eggs: self.eggs,
                    }
                })
        } else {
            None
        }
    }
}

struct Bird {
    bird: BirdArchetype,
}

impl Bird {
    pub fn new(bird: BirdArchetype) -> Self {
        Self { bird }
    }
}

impl IAnimal for Bird {
    fn eat(&mut self, calories: u32) {
        self.bird.animal.eat(calories)
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
            .try_reproduce()
            .map(|bird| Self::Offspring { bird })
    }
}

pub fn bird_main() {
    let mut bird = Bird::new(BirdArchetype {
        animal: AnimalArchetype { calories: 10 },
        eggs: 3,
    });
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
