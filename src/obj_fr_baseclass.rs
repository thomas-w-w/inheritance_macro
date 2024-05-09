use std::{
    cell::RefCell,
    fmt::Display,
    rc::Rc,
    sync::{Arc, Mutex},
};

type Pointer<T> = Arc<Mutex<T>>;

#[derive(Clone, Debug)]
pub enum ObjType {
    Obj,
    Food,
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
            ObjType::Food => write!(f, "Food"),
        }
    }
}

#[derive(Clone, Debug)]
struct World {
    animals: Vec<Animal>,
    food: Option<Arc<Mutex<Food>>>,
}
pub trait IObj {
    fn as_obj(&self) -> Pointer<Obj>;
    fn get_id(&self) -> String {
        self.as_obj().lock().unwrap().id.to_owned()
    }
    fn get_obj_type(&self) -> ObjType {
        self.as_obj().lock().unwrap().obj_type.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Obj {
    id: String,
    obj_type: ObjType,
}
impl Obj {
    pub fn new(id: String, obj_type: ObjType) -> Self {
        Self {
            id: id,
            obj_type: obj_type,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Food {
    obj: Obj,
    pub food_capacity: i32,
}
impl Food {
    pub fn new(id: String, obj_type: ObjType, food_capacity: i32) -> Self {
        Self {
            obj: Obj::new(id, obj_type),
            food_capacity,
        }
    }
}
trait IFood: IObj {
    fn as_food(&self) -> &Food;
    fn as_mut_food(&mut self) -> &mut Food;
    fn eat(&mut self, chunk: i32) -> bool {
        if self.as_mut_food().food_capacity > chunk {
            self.as_mut_food().food_capacity -= chunk;
            return true;
        }
        false
    }
}

pub trait IAnimal: IObj {
    fn as_animal(&self) -> Pointer<Animal>;

    fn get_given_name(&self) -> String {
        self.as_animal().lock().unwrap().given_name.to_owned()
    }
    fn set_given_name(&mut self, given_name: String) {
        self.as_animal().lock().unwrap().given_name = given_name;
    }
    fn get_food_reserve(&self) -> i32 {
        self.as_animal().lock().unwrap().food_reserve.clone()
    }
    fn set_food_reserve(&mut self, food_reserve: i32) {
        self.as_animal().lock().unwrap().food_reserve = food_reserve;
    }
    fn get_shared_food(&self) -> Arc<Mutex<Food>> {
        Arc::clone(&self.as_animal().lock().unwrap().shared_food)
    }

    fn eat(&mut self) -> bool {
        println!("eat start");
        let a = Arc::clone(&self.as_animal());
        let b = a.lock();
        let c = b.as_ref().unwrap();
        let d = &c.shared_food;
        let e = d.lock();
        let food = e.unwrap();
        let food_capacity = food.food_capacity.clone();
        // prevent dead lock below
        drop(food);
        drop(b);

        println!("eat: strong_count &a: {}", Arc::strong_count(&a));

        if food_capacity >= 100 {
            let a = Arc::clone(&self.as_animal());
            let b = a.lock();
            let c = b.as_ref().unwrap();
            let d = &c.shared_food;
            let e = d.lock();
            let mut food = e.unwrap();

            food.food_capacity -= 100;
            let food_capacity = food.food_capacity;

            //prevent dead lock
            drop(food);
            drop(b);

            self.set_food_reserve(self.get_food_reserve() + 100);

            println!();
            println!(
                "eat: {} {} ate, remaining food reserve: {}, remaining food_capacity: {}",
                self.get_obj_type(),
                self.get_id(),
                self.get_food_reserve(),
                food_capacity
            );
            println!();
            return true;
        }
        println!();
        println!("eat: end -- false");
        println!();

        false
    }
}

#[derive(Clone, Debug)]
pub struct Animal {
    obj: Pointer<Obj>,
    given_name: String,
    food_reserve: i32,
    shared_food: Arc<Mutex<Food>>,
}
impl Animal {
    pub fn new(
        obj: Pointer<Obj>,
        given_name: String,
        food_reserve: i32,
        shared_food: Arc<Mutex<Food>>,
    ) -> Self {
        Self {
            obj: obj,
            given_name,
            food_reserve,
            shared_food,
        }
    }
}

pub trait IBird: IAnimal {
    fn as_bird(&self) -> &Bird;
    fn as_mut_bird(&mut self) -> &mut Bird;

    fn get_wing_span(&self) -> &i32 {
        &self.as_bird().wing_span
    }
    fn set_wing_span(&mut self, wing_span: i32) {
        self.as_mut_bird().wing_span = wing_span;
    }
    fn get_maximum_speed(&self) -> &i32 {
        &self.as_bird().maximum_speed
    }
    fn set_maximum_speed(&mut self, maximum_speed: i32) {
        self.as_mut_bird().maximum_speed = maximum_speed;
    }
}

#[derive(Clone, Debug)]
pub struct Bird {
    animal: Pointer<Animal>,
    maximum_speed: i32,
    wing_span: i32,
}
impl Bird {
    pub fn new(animal: Pointer<Animal>, maximum_speed: i32, wing_span: i32) -> Self {
        Self {
            animal: animal,
            maximum_speed: maximum_speed,
            wing_span: wing_span,
        }
    }
}
impl IAnimal for Bird {
    fn as_animal(&self) -> Pointer<Animal> {
        println!("impl IAnimal for Bird::as_animal: start / end");
        Arc::clone(&self.animal)
    }
}

impl IBird for Bird {
    fn as_bird(&self) -> &Bird {
        self
    }

    fn as_mut_bird(&mut self) -> &mut Bird {
        self
    }
}
pub trait ILizard: IAnimal {
    fn as_lizard(&self) -> &Lizard;
    fn as_mut_lizard(&mut self) -> &mut Lizard;
    fn get_number_of_claws(&self) -> &i32 {
        &self.as_lizard().number_of_claws
    }
    fn set_number_of_claws(&mut self, number_of_claws: i32) {
        (self.as_mut_lizard()).number_of_claws = number_of_claws;
    }
    fn get_scale_colors(&self) -> &String {
        &self.as_lizard().scale_colors
    }
    fn set_scale_colors(&mut self, scale_colors: String) {
        (self.as_mut_lizard()).scale_colors = scale_colors;
    }
}
impl ILizard for Lizard {
    fn as_lizard(&self) -> &Lizard {
        self
    }

    fn as_mut_lizard(&mut self) -> &mut Lizard {
        self
    }
}

#[derive(Clone, Debug)]
pub struct Lizard {
    animal: Pointer<Animal>,
    number_of_claws: i32,
    scale_colors: String,
}
impl Lizard {
    pub fn new(animal: Pointer<Animal>, number_of_claws: i32, scale_colors: String) -> Self {
        Self {
            animal: animal,
            number_of_claws,
            scale_colors,
        }
    }
}
impl IAnimal for Lizard {
    fn as_animal(&self) -> Pointer<Animal> {
        println!("impl IAnimal for Lizard::as_animal: start / end");
        Arc::clone(&self.animal)
    }
}

pub trait IDragon: IBird + ILizard {
    fn as_dragon(&self) -> &Dragon;
    fn as_mut_dragon(&mut self) -> &mut Dragon;
    fn get_fire_capacity(&self) -> &i32 {
        &self.as_dragon().fire_capacity
    }
    fn set_fire_capacity(&mut self, fire_capacity: i32) {
        (self.as_mut_dragon()).fire_capacity = fire_capacity;
    }
    fn fire(&mut self) -> bool {
        let mut fire_capacity = self.get_fire_capacity().clone();

        println!("fire: fire_capacity: {fire_capacity}");

        //need 10+ fire to dire
        while fire_capacity < 10 {
            println!("fire: while loop, fire_capacity: {fire_capacity}");
            if self.get_food_reserve() > 20 {
                if self.get_food_reserve().to_owned() >= 20 {
                    self.set_fire_capacity(self.get_fire_capacity() + 10);
                    self.set_food_reserve(self.get_food_reserve() - 20);
                }
                fire_capacity = self.get_fire_capacity().clone();
                println!("fire:");
                println!("fire: food reserve converted to fire, self.get_food_reserve(): {}, self.get_fire_capacity(): {}", self.get_food_reserve(), self.get_fire_capacity());
                println!("fire:");
                break;
            }

            let ate = self.eat();
            if !ate {
                println!();
                println!(
                    "fire: {:?} Failed to eat while fire capacity too low. Break.",
                    self.as_dragon().clone()
                );
                println!();
                break;
            } else {
                // 20 food reserve => 10 fire capacity
                //1 fire = 2 food
                if self.get_food_reserve().to_owned() >= 20 {
                    self.set_fire_capacity(self.get_fire_capacity() + 10);
                    self.set_food_reserve(self.get_food_reserve() - 20);
                }
                fire_capacity = self.get_fire_capacity().clone();

                println!();
                println!(
                    "fire: ATE, fire capacity: {}, food reserve: {}.",
                    fire_capacity,
                    self.get_food_reserve()
                );
                println!();
            }
        }

        if fire_capacity >= 10 {
            self.set_fire_capacity(fire_capacity - 10);
            println!(
                "fire: Dragon {} fired, remaining fire capacity: {}",
                self.get_given_name(),
                self.get_fire_capacity()
            );
            return true;
        }
        println!(
            "fire: Dragon {} DID NOT fire, remaining fire capacity: {} and {}",
            self.get_given_name(),
            self.get_fire_capacity(),
            fire_capacity
        );
        false
    }
}
impl IDragon for Dragon {
    fn as_dragon(&self) -> &Dragon {
        self
    }

    fn as_mut_dragon(&mut self) -> &mut Dragon {
        self
    }
}
#[derive(Clone, Debug)]
pub struct Dragon {
    bird: Bird,
    lizard: Lizard,
    fire_capacity: i32,
}
impl Dragon {
    pub(crate) fn new(bird: Bird, lizard: Lizard, fire_capacity: i32) -> Self {
        Self {
            bird: bird,
            lizard: lizard,
            fire_capacity,
        }
    }
}
impl<T> IObj for T
where
    T: IAnimal,
{
    fn as_obj(&self) -> Pointer<Obj> {
        Arc::clone(&self.as_animal().lock().as_ref().unwrap().obj)
    }
}
impl<T> IAnimal for T
where
    T: IBird + ILizard,
{
    fn as_animal(&self) -> Pointer<Animal> {
        Arc::clone(&self.as_bird().animal)
    }
}
impl<T> IBird for T
where
    T: IDragon,
{
    fn as_bird(&self) -> &Bird {
        &self.as_dragon().bird
    }
    fn as_mut_bird(&mut self) -> &mut Bird {
        &mut self.as_mut_dragon().bird
    }
}
impl<T> ILizard for T
where
    T: IDragon,
{
    fn as_lizard(&self) -> &Lizard {
        &self.as_dragon().lizard
    }
    fn as_mut_lizard(&mut self) -> &mut Lizard {
        &mut self.as_mut_dragon().lizard
    }
}
