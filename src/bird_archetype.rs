#[derive(Clone, Debug)]
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
    fn this(&mut self) -> &mut Bird;
    fn eggs(&mut self) -> u32 {
        self.this().bird.eggs
    }
    fn animal(&mut self) -> &mut Animal {
        &mut self.this().animal
    }
    fn bird(&mut self) -> &mut BirdArchetype {
        &mut self.this().bird
    }

    fn peep(&self);

    fn try_reproduce(&mut self) -> Option<Bird> {
        self.try_reproduce_mf().map(|bird| Bird {
            animal: self.this().animal().clone(),
            bird: bird,
        })
    }

    fn try_reproduce_mf(&mut self) -> Option<BirdArchetype> {
        if self.this().bird.eggs > 0 {
            self.this()
                .animal()
                .calories
                .checked_sub(50)
                .map(|remaining_calories| {
                    self.this().animal().calories = remaining_calories;
                    self.this().bird.eggs -= 1;
                    BirdArchetype {
                        eggs: self.this().bird.eggs,
                    }
                })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct BirdArchetype {
    eggs: u32,
}

impl BirdArchetype {
    fn peep(&self) {
        println!("BirdArchetype::peep");
    }
}
#[derive(Debug, Clone)]
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
    fn this(&mut self) -> &mut Bird {
        self
    }

    fn peep(&self) {
        println!("IBird for Bird::peep()");
        self.bird.peep()
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
