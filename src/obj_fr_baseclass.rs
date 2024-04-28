use std::fmt::Display;

#[derive(Debug)]
pub enum ObjType {
    Obj,
    Animal,
    Bird,
}
impl Display for ObjType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjType::Obj => write!(f, "Obj"),
            ObjType::Animal => write!(f, "Animal"),
            ObjType::Bird => write!(f, "Bird"),
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

//############ NON-DYNAMIC PROGRAMMING #/*
impl<T> IObj for T
where
    T: IAnimal,
{
    fn as_obj(&self) -> &Obj {
        &self.as_animal().obj
    }
}
fn print_bird(bird: Bird) {
    println!(
        "id: {}, obj type: {}, given name: {}",
        bird.get_id(),
        bird.get_obj_type(),
        bird.get_given_name()
    );
}
// */
//############ DYNAMIC PROGRAMMING #
/*
impl<T> IObj for dyn IAnimal<Type = T> {
    fn as_obj(&self) -> &Obj {
        &self.as_animal().obj
    }
}
fn print_bird(bird: Bird) {
    let dyn_bird = &bird as &dyn IAnimal<Type = i32>;
    println!(
        "id: {}, obj_type: {}, value: {}",
        dyn_bird.get_id(),
        dyn_bird.get_obj_type(),
        dyn_bird.get_value()
    );
}
// */
pub fn obj_main() {
    let bird: Bird = Bird::new("bird-1", ObjType::Bird, "Birdie".to_owned());

    print_bird(bird);
}
