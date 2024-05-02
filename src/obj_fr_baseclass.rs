use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

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

#[derive(Clone, Debug)]
pub struct Obj {
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
pub struct Food {
    obj: Obj,
    pub food_capacity: i32,
}
impl Food {
    pub fn new(id: &str, obj_type: ObjType, food_capacity: i32) -> Self {
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
#[derive(Clone, Debug)]
struct Animal {
    obj: Obj,
    given_name: String,
    food_reserve: i32,
}
impl Animal {
    fn new(id: &str, obj_type: ObjType, given_name: String, food_reserve: i32) -> Self {
        Self {
            obj: Obj::new(id, obj_type),
            given_name,
            food_reserve,
        }
    }
}
pub trait IAnimal: IObj {
    fn as_animal(&self) -> &Animal;
    fn as_mut_animal(&mut self) -> &mut Animal;
    fn get_given_name(&self) -> &String {
        &self.as_animal().given_name
    }
    fn set_given_name(&mut self, given_name: String) {
        (self.as_mut_animal()).given_name = given_name;
    }
    fn get_food_reserve(&self) -> &i32 {
        &self.as_animal().food_reserve
    }
    fn set_food_reserve(&mut self, food_reserve: i32) {
        (self.as_mut_animal()).food_reserve = food_reserve;
    }
    //    fn eat_meat(&self, shared_food: Arc<Mutex<Food>>) -> _ {

    fn eat(&mut self, shared_food: Arc<Mutex<Food>>) -> bool {
        println!("eat start");

        let try_lock_result = shared_food.try_lock();

        let mut food: std::sync::MutexGuard<Food>;

        match try_lock_result {
            Ok(result) => {
                food = result;
                println!("eat: ok aquire lock")
            }
            Err(err) => {
                println!("eat: error aquire lock: {:?}", err);
                panic!();
            }
        }

        //let mut food = shared_food.try_lock().unwrap();

        println!("eat lock acquired");

        if (food.food_capacity >= 100) {
            food.food_capacity -= 100;
            self.as_mut_animal().food_reserve += 100;
            println!(
                "{} {} ate, remaining food reserve: {}, remaining food_capacity: {}",
                self.get_obj_type(),
                self.get_id(),
                self.get_food_reserve(),
                food.food_capacity
            );
            return true;
        }
        println!("eat end -- false");
        false
    }
}

fn get_world() -> World {
    todo!()
}
#[derive(Clone, Debug)]
pub struct Bird {
    animal: Animal,
    maximum_speed: i32,
    wing_span: i32,
}
impl Bird {
    fn new(
        id: &str,
        obj_type: ObjType,
        given_name: String,
        food_reserve: i32,
        maximum_speed: i32,
        wing_span: i32,
    ) -> Self {
        Self {
            animal: Animal::new(id, obj_type, given_name, food_reserve),
            maximum_speed: maximum_speed,
            wing_span: wing_span,
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
pub trait IBird: IAnimal {
    fn as_bird(&self) -> &Bird;
    fn as_mut_bird(&mut self) -> &mut Bird;

    fn get_wing_span(&self) -> &i32 {
        &self.as_bird().wing_span
    }
    fn set_wing_span(&mut self, wing_span: i32) {
        (self.as_mut_bird()).wing_span = wing_span;
    }
    fn get_maximum_speed(&self) -> &i32 {
        &self.as_bird().maximum_speed
    }
    fn set_maximum_speed(&mut self, maximum_speed: i32) {
        (self.as_mut_bird()).maximum_speed = maximum_speed;
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

#[derive(Clone, Debug)]
pub struct Lizard {
    animal: Animal,
    number_of_claws: i32,
    scale_colors: String,
}
impl Lizard {
    fn new(
        id: &str,
        obj_type: ObjType,
        given_name: String,
        food_reserve: i32,
        number_of_claws: i32,
        scale_colors: String,
    ) -> Self {
        Self {
            animal: Animal::new(id, obj_type, given_name, food_reserve),
            number_of_claws,
            scale_colors,
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

pub trait IDragon: IBird + ILizard {
    fn as_dragon(&self) -> &Dragon;
    fn as_mut_dragon(&mut self) -> &mut Dragon;
    fn get_fire_capacity(&self) -> &i32 {
        &self.as_dragon().fire_capacity
    }
    fn set_fire_capacity(&mut self, fire_capacity: i32) {
        (self.as_mut_dragon()).fire_capacity = fire_capacity;
    }
    fn fire(&mut self, shared_food: Arc<Mutex<Food>>) -> bool {
        let fire_capacity = self.get_fire_capacity().clone();

        println!("fire: fire_capacity: {fire_capacity}");

        while (fire_capacity <= 10) {
            println!("fire: while loop");
            let ate = self.eat(Arc::clone(&shared_food));
            if !ate {
                println!(
                    "{:?} Failed to eat while fire capacity too low. Break.",
                    self.as_dragon().clone()
                );
                break;
            } else {
                if self.get_food_reserve().to_owned() > 100 {
                    self.set_fire_capacity(10);
                    self.set_food_reserve(10);
                }
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
    pub(crate) fn new(
        id: &str,
        obj_type: ObjType,
        given_name: String,
        food_reserve: i32,
        maximum_speed: i32,
        wing_span: i32,
        number_of_claws: i32,
        scale_colors: String,
        fire_capacity: i32,
    ) -> Self {
        Self {
            bird: Bird::new(
                id,
                obj_type.clone(),
                given_name.clone(),
                food_reserve,
                maximum_speed,
                wing_span,
            ),
            lizard: Lizard::new(
                id,
                obj_type.clone(),
                given_name.clone(),
                food_reserve,
                number_of_claws,
                scale_colors,
            ),
            fire_capacity,
        }
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

pub fn obj_main() {
    let mut shared_food = Arc::new(Mutex::new(Food::new("food-1", ObjType::Food, 100000)));

    //Arc<Mutex<Food>>

    let world = World {
        animals: vec![],
        food: Some(Arc::clone(&shared_food)),
    };

    let mut bird: Bird = Bird::new("bird-1", ObjType::Bird, "Birdie".to_owned(), 100, 10, 4);
    println!("\r\n{:?}\r\n", bird.clone());

    bird.set_given_name("Birdie Num".to_owned());
    bird.set_food_reserve(200);
    bird.set_maximum_speed(1000);
    bird.set_wing_span(10);
    println!("\r\n{:?}\r\n", bird.clone());

    bird.eat(Arc::clone(&shared_food));

    let food = shared_food.lock().unwrap();
    println!("\r\n{:?}\r\n{:?}\r\n", bird.clone(), food.clone());
    drop(food);

    let mut lizard: Lizard = Lizard::new(
        "lizard-1",
        ObjType::Lizard,
        "Lizzie".to_owned(),
        1000,
        24,
        "blue, red, green".to_owned(),
    );
    println!("\r\n{:?}\r\n", lizard.clone());
    lizard.set_given_name("Lizzy the Busy".to_owned());
    lizard.set_food_reserve(1000);
    lizard.set_number_of_claws(42);
    lizard.set_scale_colors("yellow blue".to_owned());
    println!("\r\n{:?}\r\n", lizard.clone());
    lizard.eat(Arc::clone(&shared_food));

    let food = shared_food.lock().unwrap();
    println!("\r\n{:?}\r\n{:?}\r\n", lizard.clone(), food.clone());
    drop(food);

    let mut dragon: Dragon = Dragon::new(
        "dragon-1",
        ObjType::Dragon,
        "Il Dragone".to_owned(),
        50,
        100,
        10,
        36,
        "green, red,".to_owned(),
        5,
    );
    println!("\r\n{:?}\r\n", dragon.clone());
    dragon.set_given_name("Il Dragone Gigante".to_owned());
    dragon.set_food_reserve(50);
    dragon.set_maximum_speed(20);
    dragon.set_wing_span(25);
    dragon.set_number_of_claws(72);
    dragon.set_scale_colors("white blue".to_owned());
    dragon.set_fire_capacity(5);
    println!("\r\n{:?}\r\n", dragon.clone());
    dragon.eat(Arc::clone(&shared_food));

    let food = shared_food.lock().unwrap();
    println!("\r\n{:?}\r\n{:?}\r\n", dragon.clone(), food.clone());
    drop(food);

    dragon.fire(Arc::clone(&shared_food));
    dragon.fire(Arc::clone(&shared_food));
    dragon.fire(Arc::clone(&shared_food));
    dragon.fire(Arc::clone(&shared_food));
    dragon.fire(Arc::clone(&shared_food));
    dragon.fire(Arc::clone(&shared_food));
    dragon.fire(Arc::clone(&shared_food));
    dragon.fire(Arc::clone(&shared_food));
    dragon.fire(Arc::clone(&shared_food));
    dragon.fire(Arc::clone(&shared_food));
    dragon.fire(Arc::clone(&shared_food));
    dragon.fire(Arc::clone(&shared_food));
    dragon.fire(Arc::clone(&shared_food));
    dragon.fire(Arc::clone(&shared_food));
    dragon.fire(Arc::clone(&shared_food));
    dragon.fire(Arc::clone(&shared_food));
    dragon.fire(Arc::clone(&shared_food));
    println!("\r\n{:?}\r\n", dragon.clone());
}
//https://medium.com/comsystoreply/28-days-of-rust-part-2-composition-over-inheritance-cab1b106534a#id_token=eyJhbGciOiJSUzI1NiIsImtpZCI6ImFjM2UzZTU1ODExMWM3YzdhNzVjNWI2NTEzNGQyMmY2M2VlMDA2ZDAiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMTYyOTYwMzU4MzQtazFrNnFlMDYwczJ0cDJhMmphbTRsamRjbXMwMHN0dGcuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMTYyOTYwMzU4MzQtazFrNnFlMDYwczJ0cDJhMmphbTRsamRjbXMwMHN0dGcuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMDg1MTI2MDQ3MjEyNzU4ODQzMjQiLCJlbWFpbCI6InRob21hcy53ZXN0ZXJnYXJkQGdtYWlsLmNvbSIsImVtYWlsX3ZlcmlmaWVkIjp0cnVlLCJuYmYiOjE3MTQzMzgzNTgsIm5hbWUiOiJUaG9tYXMgV2VzdGVyZ2FyZCIsInBpY3R1cmUiOiJodHRwczovL2xoMy5nb29nbGV1c2VyY29udGVudC5jb20vYS9BQ2c4b2NKMTF4amowR0JMc1BJb3dVSUEySkV4TW9hRHZXeGJ0VUFHRmNMS25XMmF1YU9wS0E9czk2LWMiLCJnaXZlbl9uYW1lIjoiVGhvbWFzIiwiZmFtaWx5X25hbWUiOiJXZXN0ZXJnYXJkIiwiaWF0IjoxNzE0MzM4NjU4LCJleHAiOjE3MTQzNDIyNTgsImp0aSI6ImMwY2IyZTllNDg5YWE0NzcxYjc0NzBhZDJkNGUzMjA2ZGIxM2IyMjkifQ.NtnmCLmOqm2aTywS2BpXwGiqhWMnJmQSgm6dew6e-ptmq2nU5t7IK85NKyPXULvU_E2IZKUhiGYxRaeE7wCn070Vsj4QtV_KU0uJ-pCZYj4D7NL86WOUwvnyeUwjBhj5bgoAos0iwmUWL2QHa2UnRvnYdaTyKtmbw9kSAw4N0iaNPwWfzyo1k2FRq_v0qOHDZWEoSZYmLdxeBZ5xbZrzCZm26t1_0M7BjZs03R174yUsxYlvc6ZfgpdL_qQ1X4HYaKq9GDL4v1GbOUBni0RtRfKahpn4RIX6161CYicb-WaYuVMKj4_dfJ4z4G_Ofvnz3Z10e3M4aSSNZ5XpPuPKYA
