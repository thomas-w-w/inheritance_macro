use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

type Pointer<T> = Arc<Mutex<T>>;

#[derive(Clone, Debug, PartialEq)]
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
        let animal_pointer = self.as_animal();
        let animal = animal_pointer.lock().unwrap();
        let mut food = animal.shared_food.lock().unwrap();

        let mut food_capacity = food.food_capacity;

        if food_capacity >= 100 {
            food.food_capacity -= 100;

            food_capacity = food.food_capacity;
            let s = format!(
                "Animal given name: {}, food reserve: {}",
                animal.given_name, animal.food_reserve
            );

            println!("IAnimal::eat(): eater: {s}");

            //prevent dead lock
            drop(food);
            drop(animal);

            self.set_food_reserve(self.get_food_reserve() + 100);

            println!(
                "IAnimal::eat(): {} ate+100, food reserve: {}, global food capacity: {}",
                self.get_id(),
                self.get_food_reserve(),
                food_capacity
            );

            return true;
        } else {
            //prevent dead lock
            drop(food);
            drop(animal);

            println!(
                "IAnimal::eat(): FAILED for {}, global food capacity less than 100.",
                self.get_id()
            );
        }
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
impl<T> IObj for T
where
    T: IAnimal,
{
    fn as_obj(&self) -> Pointer<Obj> {
        Arc::clone(&self.as_animal().lock().as_ref().unwrap().obj)
    }
}

pub trait IBird: IAnimal {
    fn as_red_bird(&self) -> Option<&Bird> {
        None
    }
    fn as_mut_bird(&mut self) -> Option<&mut Bird> {
        None
    }
    fn as_bird(&self) -> Option<Pointer<Bird>> {
        None
    }

    fn get_wing_span(&self) -> i32 {
        if ObjType::Bird == self.get_obj_type() {
            self.as_red_bird().unwrap().wing_span.to_owned()
        } else {
            self.as_bird().unwrap().lock().unwrap().wing_span.to_owned()
        }
    }
    fn set_wing_span(&mut self, wing_span: i32) {
        if ObjType::Lizard == self.get_obj_type() {
            self.as_mut_bird().unwrap().wing_span = wing_span;
        } else {
            self.as_bird().unwrap().lock().unwrap().wing_span = wing_span;
        }
    }
    fn get_maximum_speed(&self) -> i32 {
        if ObjType::Bird == self.get_obj_type() {
            self.as_red_bird().unwrap().maximum_speed.to_owned()
        } else {
            self.as_bird()
                .unwrap()
                .lock()
                .unwrap()
                .maximum_speed
                .to_owned()
        }
    }
    fn set_maximum_speed(&mut self, maximum_speed: i32) {
        if ObjType::Lizard == self.get_obj_type() {
            self.as_mut_bird().unwrap().maximum_speed = maximum_speed;
        } else {
            self.as_bird().unwrap().lock().unwrap().maximum_speed = maximum_speed;
        }
    }
}

#[derive(Clone, Debug)]
pub struct Bird {
    animal: Pointer<Animal>,
    maximum_speed: i32,
    wing_span: i32,
}

impl IBird for Bird {
    fn as_red_bird(&self) -> Option<&Bird> {
        Some(self)
    }

    fn as_mut_bird(&mut self) -> Option<&mut Bird> {
        Some(self)
    }
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
        Arc::clone(&self.animal)
    }
}

pub trait ILizard: IAnimal {
    fn as_red_lizard(&self) -> Option<&Lizard> {
        None
    }
    fn as_mut_lizard(&mut self) -> Option<&mut Lizard> {
        None
    }
    fn as_lizard(&self) -> Option<Pointer<Lizard>> {
        None
    }
    fn get_number_of_claws(&self) -> i32 {
        if ObjType::Lizard == self.get_obj_type() {
            self.as_red_lizard().unwrap().number_of_claws.to_owned()
        } else {
            self.as_lizard()
                .unwrap()
                .lock()
                .unwrap()
                .number_of_claws
                .to_owned()
        }
    }
    fn set_number_of_claws(&mut self, number_of_claws: i32) {
        if ObjType::Lizard == self.get_obj_type() {
            self.as_mut_lizard().unwrap().number_of_claws = number_of_claws;
        } else {
            self.as_lizard().unwrap().lock().unwrap().number_of_claws = number_of_claws;
        }
    }

    fn get_scale_colors(&self) -> String {
        if ObjType::Lizard == self.get_obj_type() {
            self.as_red_lizard().unwrap().scale_colors.to_owned()
        } else {
            self.as_lizard()
                .unwrap()
                .lock()
                .unwrap()
                .scale_colors
                .to_owned()
        }
    }
    fn set_scale_colors(&mut self, scale_colors: String) {
        if ObjType::Lizard == self.get_obj_type() {
            self.as_mut_lizard().unwrap().scale_colors = scale_colors;
        } else {
            self.as_lizard().unwrap().lock().unwrap().scale_colors = scale_colors;
        }
    }
}

#[derive(Clone, Debug)]
pub struct Lizard {
    animal: Pointer<Animal>,
    number_of_claws: i32,
    scale_colors: String,
}

impl ILizard for Lizard {
    fn as_mut_lizard(&mut self) -> Option<&mut Lizard> {
        Some(self)
    }
    fn as_red_lizard(&self) -> Option<&Lizard> {
        Some(self)
    }
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
        Arc::clone(&self.animal)
    }
}

/// Finally, it’s possible to implement a trait for all classes
///  that implement one of a number of other traits.
///
/// It requires specialization, which is a nightly feature
/// for now (though there is a workaround available,
/// even packed in a macro crate if you don’t want to
/// write out all the boilerplate required).
///
/// Traits may very well inherit from each other,
/// though they only prescribe behavior, not data.
///
/// https://blog.logrocket.com/understanding-inheritance-other-limitations-rust/#Inheritance%20in%20Rust
///
/// This Rust guide was updated on 3 Aug, 2022
/// May 14, 2021 Andre "llogiq" Bogus
///  
/// https://github.com/dtolnay/case-studies/blob/master/autoref-specialization/README.md
///
impl<T> IAnimal for T
where
    T: IBird + ILizard,
{
    fn as_animal(&self) -> Pointer<Animal> {
        Arc::clone(&self.as_bird().unwrap().lock().unwrap().animal)
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
        //need 10+ fire to dire
        while fire_capacity < 10 {
            if self.get_food_reserve() > 20 {
                if self.get_food_reserve().to_owned() >= 20 {
                    self.set_fire_capacity(self.get_fire_capacity() + 10);
                    self.set_food_reserve(self.get_food_reserve() - 20);
                }
                fire_capacity = self.get_fire_capacity().clone();
                break;
            }

            let ate = self.eat();
            if !ate {
                break;
            } else {
                // 20 food reserve => 10 fire capacity
                //1 fire = 2 food
                if self.get_food_reserve().to_owned() >= 20 {
                    self.set_fire_capacity(self.get_fire_capacity() + 10);
                    self.set_food_reserve(self.get_food_reserve() - 20);
                }
                fire_capacity = self.get_fire_capacity().clone();
            }
        }

        if fire_capacity >= 10 {
            self.set_fire_capacity(fire_capacity - 10);
            return true;
        }
        false
    }
}

#[derive(Clone, Debug)]
pub struct Dragon {
    bird: Pointer<Bird>,
    lizard: Pointer<Lizard>,
    fire_capacity: i32,
}

impl Dragon {
    pub(crate) fn new(bird: Pointer<Bird>, lizard: Pointer<Lizard>, fire_capacity: i32) -> Self {
        Self {
            bird: bird,
            lizard: lizard,
            fire_capacity,
        }
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

///"Lock data, not code" is enforced in Rust.
/// https://brson.github.io/rust-anthology/1/fearless-concurrency.html

impl<T> IBird for T
where
    T: IDragon,
{
    fn as_bird(&self) -> Option<Pointer<Bird>> {
        Some(Arc::clone(&self.as_dragon().bird))
    }
}
impl<T> ILizard for T
where
    T: IDragon,
{
    fn as_lizard(&self) -> Option<Pointer<Lizard>> {
        Some(Arc::clone(&self.as_dragon().lizard))
    }
}
