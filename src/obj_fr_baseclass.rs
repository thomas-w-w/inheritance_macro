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
        println!("get_id i");
        let as_obj_arc_clone = self.as_obj();
        println!("get_id ii");
        let as_obj_clone = as_obj_arc_clone.lock();
        println!("get_id iii");
        let as_obj_clone_as_ref_mtx = as_obj_clone.as_ref().unwrap();
        println!("get_id iv");
        println!("get_id after mtxg_as_obj");
        let id_clone = as_obj_clone_as_ref_mtx.id.clone();
        // // println!("get_id after id_clone");
        // // //self.as_obj().lock().unwrap().id.clone()
        id_clone
        //"foo id".to_string()
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
        println!("get_given_name: start");

        let a = self.as_animal();

        println!("get_given_name: i");

        let b = Arc::clone(&a);

        println!("get_given_name: ii");

        let c = b.lock();

        println!("get_given_name: iii");

        let d = c.as_ref().unwrap();

        println!("get_given_name: iv");

        let e = d.given_name.clone();

        println!("get_given_name: v");

        e
        // self.as_animal().lock().unwrap().given_name.clone()
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

        let given_name = c.given_name.clone();
        let d = &c.shared_food;

        println!("eat: before lock d: {:?}", d);
        let e = d.lock();
        println!("eat: after lock d: {:?}", d);
        let food = e.unwrap();

        let food_capacity = food.food_capacity.clone();

        println!(
            "eat: lock acquired ii ii ii ii i, given name: {}",
            given_name
        );

        drop(food);
        drop(b);

        println!("eat: strong_count &a: {}", Arc::strong_count(&a));

        println!("eat: get_id(): {}", self.get_id());
        println!("eat: self.get_obj_type(): {}", self.get_obj_type());

        let food_reserve = self.get_food_reserve();
        println!("eat: food_reserve: {}", food_reserve);
        println!("eat: food_capacity: {}", food_capacity);

        // println!(
        //     "{} {} ate, remaining food reserve: {}, remaining food_capacity: {}",
        //     self.get_obj_type(),
        //     self.get_id(),
        //     self.get_food_reserve(),
        //     food.food_capacity
        // );

        // // println!("eat food.food_capacity: {}", food.food_capacity);

        if food_capacity >= 100 {
            let a = Arc::clone(&self.as_animal());
            let b = a.lock();
            let c = b.as_ref().unwrap();
            let d = &c.shared_food;
            let e = d.lock();
            let mut food = e.unwrap();
            //food.food_capacity -= 100;
            //self.set_food_reserve(100);
            food.food_capacity -= 100;
            let food_capacity = food.food_capacity;
            drop(food);
            drop(b);
            self.set_food_reserve(self.get_food_reserve() + 100);
            println!(
                "eat: {} {} ate, remaining food reserve: {}, remaining food_capacity: {}",
                self.get_obj_type(),
                self.get_id(),
                self.get_food_reserve(),
                food_capacity
            );
            return true;
        }
        println!("eat: end -- false");
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

        while (fire_capacity < 10) {
            println!("fire: while loop, fire_capacity: {fire_capacity}");
            let ate = self.eat();
            if !ate {
                println!(
                    "fire: {:?} Failed to eat while fire capacity too low. Break.",
                    self.as_dragon().clone()
                );
                break;
            } else {
                // 20 food reserve => 10 fire capacity
                fire_capacity = self.get_fire_capacity().clone();
                if self.get_food_reserve().to_owned() >= 20 {
                    self.set_fire_capacity(self.get_fire_capacity() + 10);
                    self.set_food_reserve(self.get_food_reserve() - 20);
                }
                println!(
                    "fire: ATE, fire capacity: {}, food reserve: {}.",
                    self.get_fire_capacity(),
                    self.get_food_reserve()
                );
            }
        }

        if fire_capacity >= 10 {
            self.set_fire_capacity(fire_capacity - 10);
            println!(
                "Dragon {} fired, remaining fire capacity: {}",
                self.get_given_name(),
                self.get_fire_capacity()
            );
            return true;
        }
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
        let a = Arc::clone(&self.as_animal());
        let b = a.lock();
        let c = b.as_ref().unwrap();
        Arc::clone(&c.obj)
    }
}
/// https://doc.rust-lang.org/rust-by-example/generics/where.html
impl<T> IAnimal for T
where
    T: IBird + ILizard,
{
    fn as_animal(&self) -> Pointer<Animal> {
        // // // let a = self.as_bird();
        // // // let b = &a.animal;
        // // // Arc::clone(b)
        // let a = Arc::clone(&self.as_animal());
        // let b = a.lock();
        // let c = b.as_ref().unwrap();
        // Arc::clone(&c.obj)
        // Arc::clone(&self.as_bird().animal)

        println!("impl<T> IAnimal for T where T: IBird + ILizard::as_animal: i");
        let a = self.as_bird();
        println!("impl<T> IAnimal for T where T: IBird + ILizard::as_animal: ii");
        let b = &a.animal;
        println!(
            "impl<T> IAnimal for T where T: IBird + ILizard::as_animal: iii/end: {:?}",
            b
        );
        println!(
            "impl<T> IAnimal for T where T: strong_count of b: {:?}",
            Arc::strong_count(b)
        );
        let data = b.is_poisoned();
        println!(": is poisoned: {}. End", data);
        Arc::clone(b)
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

//https://medium.com/comsystoreply/28-days-of-rust-part-2-composition-over-inheritance-cab1b106534a#id_token=eyJhbGciOiJSUzI1NiIsImtpZCI6ImFjM2UzZTU1ODExMWM3YzdhNzVjNWI2NTEzNGQyMmY2M2VlMDA2ZDAiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMTYyOTYwMzU4MzQtazFrNnFlMDYwczJ0cDJhMmphbTRsamRjbXMwMHN0dGcuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMTYyOTYwMzU4MzQtazFrNnFlMDYwczJ0cDJhMmphbTRsamRjbXMwMHN0dGcuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMDg1MTI2MDQ3MjEyNzU4ODQzMjQiLCJlbWFpbCI6InRob21hcy53ZXN0ZXJnYXJkQGdtYWlsLmNvbSIsImVtYWlsX3ZlcmlmaWVkIjp0cnVlLCJuYmYiOjE3MTQzMzgzNTgsIm5hbWUiOiJUaG9tYXMgV2VzdGVyZ2FyZCIsInBpY3R1cmUiOiJodHRwczovL2xoMy5nb29nbGV1c2VyY29udGVudC5jb20vYS9BQ2c4b2NKMTF4amowR0JMc1BJb3dVSUEySkV4TW9hRHZXeGJ0VUFHRmNMS25XMmF1YU9wS0E9czk2LWMiLCJnaXZlbl9uYW1lIjoiVGhvbWFzIiwiZmFtaWx5X25hbWUiOiJXZXN0ZXJnYXJkIiwiaWF0IjoxNzE0MzM4NjU4LCJleHAiOjE3MTQzNDIyNTgsImp0aSI6ImMwY2IyZTllNDg5YWE0NzcxYjc0NzBhZDJkNGUzMjA2ZGIxM2IyMjkifQ.NtnmCLmOqm2aTywS2BpXwGiqhWMnJmQSgm6dew6e-ptmq2nU5t7IK85NKyPXULvU_E2IZKUhiGYxRaeE7wCn070Vsj4QtV_KU0uJ-pCZYj4D7NL86WOUwvnyeUwjBhj5bgoAos0iwmUWL2QHa2UnRvnYdaTyKtmbw9kSAw4N0iaNPwWfzyo1k2FRq_v0qOHDZWEoSZYmLdxeBZ5xbZrzCZm26t1_0M7BjZs03R174yUsxYlvc6ZfgpdL_qQ1X4HYaKq9GDL4v1GbOUBni0RtRfKahpn4RIX6161CYicb-WaYuVMKj4_dfJ4z4G_Ofvnz3Z10e3M4aSSNZ5XpPuPKYA
