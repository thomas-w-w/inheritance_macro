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
trait IAnimal {
    fn as_animal(&self) -> &Animal;
    fn get_given_name(&self) -> &String {
        &self.as_animal().given_name
    }
}
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
}
trait IBird {
    fn as_bird(&self) -> &Bird;
    fn get_given_name(&self) -> &String {
        &self.as_bird().as_animal().given_name
    }
}
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
}
trait ILizard {
    fn as_lizard(&self) -> &Lizard;
    fn get_given_name(&self) -> &String {
        &self.as_lizard().as_animal().given_name
    }
}

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
    fn get_given_name(&self) -> &String {
        &self.as_lizard().as_animal().given_name
    }
}
impl IBird for Dragon {
    fn as_bird(&self) -> &Bird {
        &self.bird
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
impl<T> IAnimal for T
where
    T: IBird + ILizard,
{
    fn as_animal(&self) -> &Animal {
        &self.as_bird().animal
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
    let bird: Bird = Bird::new("bird-1", ObjType::Bird, "Birdie".to_owned());

    print_bird(bird);

    let lizard: Lizard = Lizard::new("lizard-1", ObjType::Lizard, "Lizzie".to_owned());

    print_lizard(lizard);

    let dragon: Dragon = Dragon::new("dragon-1", ObjType::Dragon, "Il Dragone".to_owned());

    print_dragon(dragon);
}
