use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum ObjType {
    Obj,
    Animal,
    Bird,
    Lizard,
    Dragon,
}
impl Display for ObjType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjType::Obj => write!(f, "Obj"),
            ObjType::Animal => write!(f, "Animal"),
            ObjType::Bird => write!(f, "Bird"),
            ObjType::Lizard => write!(f, "Lizard"),
            ObjType::Dragon => write!(f, "Dragon"),
        }
    }
}

#[derive(Clone, Debug)]
struct Obj {
    id: String,
    obj_type: ObjType,
}
impl Obj {
    fn new(id: &str, obj_type: ObjType) -> Self {
        Self {
            id: id.to_string(),
            obj_type: obj_type,
        }
    }
}
trait IObj {
    fn as_obj(&self) -> &Obj;
    fn get_id(&self) -> &str {
        &self.as_obj().id
    }
    fn get_obj_type(&self) -> &ObjType {
        &self.as_obj().obj_type
    }
}
#[derive(Clone, Debug)]
struct Animal {
    obj: Obj,
    given_name: String,
}
impl Animal {
    fn new(id: &str, obj_type: ObjType, given_name: String) -> Self {
        Self {
            obj: Obj::new(id, obj_type),
            given_name,
        }
    }
}
trait IAnimal: IObj {
    fn as_animal(&self) -> &Animal;
    fn as_mut_animal(&mut self) -> &mut Animal;
    fn get_given_name(&self) -> &String {
        &self.as_animal().given_name
    }
    fn set_given_name(&mut self, given_name: String) {
        (self.as_mut_animal()).given_name = given_name;
    }
}
#[derive(Clone, Debug)]
struct Bird {
    animal: Animal,
}
impl Bird {
    fn new(id: &str, obj_type: ObjType, given_name: String) -> Self {
        Self {
            animal: Animal::new(id, obj_type, given_name),
        }
    }
}
impl IAnimal for Bird {
    fn as_animal(&self) -> &Animal {
        &self.animal
    }
    fn as_mut_animal(&mut self) -> &mut Animal {
        &mut self.animal
    }
}
trait IBird: IAnimal {
    fn as_bird(&self) -> &Bird;
    fn as_mut_bird(&mut self) -> &mut Bird;
}
#[derive(Clone, Debug)]
struct Lizard {
    animal: Animal,
}
impl Lizard {
    fn new(id: &str, obj_type: ObjType, given_name: String) -> Self {
        Self {
            animal: Animal::new(id, obj_type, given_name),
        }
    }
}
impl IAnimal for Lizard {
    fn as_animal(&self) -> &Animal {
        &self.animal
    }
    fn as_mut_animal(&mut self) -> &mut Animal {
        &mut self.animal
    }
}
trait ILizard: IAnimal {
    fn as_lizard(&self) -> &Lizard;
}

trait IDragon: IBird + ILizard {}
#[derive(Clone, Debug)]
struct Dragon {
    bird: Bird,
    lizard: Lizard,
}
impl Dragon {
    fn new(id: &str, obj_type: ObjType, given_name: String) -> Self {
        Self {
            bird: Bird::new(id, obj_type.clone(), given_name.clone()),
            lizard: Lizard::new(id, obj_type.clone(), given_name.clone()),
        }
    }
}
impl IBird for Dragon {
    fn as_bird(&self) -> &Bird {
        &self.bird
    }
    fn as_mut_bird(&mut self) -> &mut Bird {
        &mut self.bird
    }
}
impl ILizard for Dragon {
    fn as_lizard(&self) -> &Lizard {
        &self.lizard
    }
}
impl<T> IObj for T
where
    T: IAnimal,
{
    fn as_obj(&self) -> &Obj {
        &self.as_animal().obj
    }
}
/// https://doc.rust-lang.org/rust-by-example/generics/where.html
impl<T> IAnimal for T
where
    T: IBird + ILizard,
{
    fn as_animal(&self) -> &Animal {
        &self.as_bird().animal
    }
    fn as_mut_animal(&mut self) -> &mut Animal {
        &mut self.as_mut_bird().animal
    }
}

fn print_bird(bird: Bird) {
    println!(
        "Bird id: {}, obj type: {}, given name: {}",
        bird.get_id(),
        bird.get_obj_type(),
        bird.get_given_name()
    );
}

fn print_lizard(lizard: Lizard) {
    println!(
        "Lizard id: {}, obj type: {}, given name: {}",
        lizard.get_id(),
        lizard.get_obj_type(),
        lizard.get_given_name()
    );
}

fn print_dragon(dragon: Dragon) {
    println!(
        "Dragon id: {}, obj type: {}, given name: {}",
        dragon.get_id(),
        dragon.get_obj_type(),
        dragon.get_given_name()
    );
}

pub fn obj_main() {
    let mut bird: Bird = Bird::new("bird-1", ObjType::Bird, "Birdie".to_owned());
    print_bird(bird.clone());
    bird.set_given_name("Birdie Num".to_owned());
    print_bird(bird.clone());

    let mut lizard: Lizard = Lizard::new("lizard-1", ObjType::Lizard, "Lizzie".to_owned());
    print_lizard(lizard.clone());
    lizard.set_given_name("Lizzy the Busy".to_owned());
    print_lizard(lizard.clone());

    let mut dragon: Dragon = Dragon::new("dragon-1", ObjType::Dragon, "Il Dragone".to_owned());
    print_dragon(dragon.clone());
    dragon.set_given_name("Il Dragone Gigante".to_owned());
    print_dragon(dragon.clone());
}