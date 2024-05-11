use std::{
    fmt::Error,
    sync::{Arc, Mutex},
    thread,
};

use obj_fr_baseclass::{Animal, Bird, Dragon, Food, IDragon, Lizard, Obj, ObjType};

use crate::{obj_fr_baseclass, IAnimal, IBird, ILizard};

fn create_bird(
    id: String,
    given_name: String,
    food_reserve: i32,
    maximum_speed: i32,
    wing_span: i32,
    food_resource: Arc<Mutex<Food>>,
) -> Arc<Mutex<Bird>> {
    let obj = Arc::new(Mutex::new(Obj::new(id, ObjType::Bird)));

    let animal = Arc::new(Mutex::new(Animal::new(
        Arc::clone(&obj),
        given_name.clone(),
        food_reserve,
        food_resource,
    )));

    let bird = Arc::new(Mutex::new(Bird::new(
        Arc::clone(&animal),
        maximum_speed,
        wing_span,
    )));

    bird
}

fn create_lizard(
    id: String,
    given_name: String,
    food_reserve: i32,
    number_of_claws: i32,
    scale_colors: String,
    food_resource: Arc<Mutex<Food>>,
) -> Arc<Mutex<Lizard>> {
    let obj = Arc::new(Mutex::new(Obj::new(id, ObjType::Lizard)));

    let animal = Arc::new(Mutex::new(Animal::new(
        Arc::clone(&obj),
        given_name.clone(),
        food_reserve,
        food_resource,
    )));

    let lizard = Arc::new(Mutex::new(Lizard::new(
        Arc::clone(&animal),
        number_of_claws,
        scale_colors,
    )));

    lizard
}

fn create_dragon(
    id: String,
    given_name: String,
    food_reserve: i32,
    maximum_speed: i32,
    wing_span: i32,
    number_of_claws: i32,
    scale_colors: String,
    fire_capacity: i32,
    food_resource: Arc<Mutex<Food>>,
) -> Arc<Mutex<Dragon>> {
    let obj = Arc::new(Mutex::new(Obj::new(id, ObjType::Dragon)));

    let animal = Arc::new(Mutex::new(Animal::new(
        Arc::clone(&obj),
        given_name.clone(),
        food_reserve,
        food_resource,
    )));

    let bird = Arc::new(Mutex::new(Bird::new(
        Arc::clone(&animal),
        maximum_speed,
        wing_span,
    )));

    let lizard = Arc::new(Mutex::new(Lizard::new(
        Arc::clone(&animal),
        number_of_claws,
        scale_colors,
    )));

    let dragon = Arc::new(Mutex::new(Dragon::new(
        Arc::clone(&bird),
        Arc::clone(&lizard),
        fire_capacity,
    )));
    dragon
}

pub async fn main_dragon() {
    println!("# ##  ###   ####  M A I N I  #####    ####   ###  ## #");

    let food_resource: Arc<Mutex<Food>> = Arc::new(Mutex::new(Food::new(
        "Food-1".to_string(),
        ObjType::Food,
        100000,
    )));

    let mut fire_handle_and_dragon_vect: Vec<(
        std::thread::JoinHandle<Result<String, Error>>,
        Arc<Mutex<Dragon>>,
    )> = vec![];

    let mut crawl_handle_and_lizard_vect: Vec<(
        std::thread::JoinHandle<Result<String, Error>>,
        Arc<Mutex<Lizard>>,
    )> = vec![];

    let mut fly_handle_and_bird_vect: Vec<(
        std::thread::JoinHandle<Result<String, Error>>,
        Arc<Mutex<Bird>>,
    )> = vec![];

    let food_reserve: i32 = 100;
    let maximum_speed: i32 = 10;
    let wing_span: i32 = 34;
    let number_of_claws: i32 = 4;
    let scale_colors: String = "red green yellow".to_string();
    let fire_capacity: i32 = 0;

    println!("# ##  ###   ####  M A I N II dragons  #####    ####   ###  ## #");

    for i in 2..50 {
        let id = format!("{}-{}", ObjType::Dragon, i);

        let dragon = create_dragon(
            id.clone(),
            format!("Red Dragon [{}]", id.clone()),
            food_reserve,
            maximum_speed,
            wing_span,
            number_of_claws,
            scale_colors.clone(),
            fire_capacity,
            Arc::clone(&food_resource),
        );

        let dragon_clone = Arc::clone(&dragon);

        let fire_handle = thread::spawn(move || {
            loop {
                let mut dragon_lock = dragon_clone.lock().unwrap();

                let given_name = dragon_lock.get_given_name();

                let mut do_break = false;

                if dragon_lock.fire() {
                    println!("loop: Dragon {given_name} FIRED.");
                } else {
                    println!("loop: Dragon {given_name} did NOT fire. BREAK.");
                    do_break = true;
                }

                drop(dragon_lock);

                if do_break {
                    break;
                }
            }
            let str = format!("{i}").as_str().to_owned();
            Ok(str)
        });

        fire_handle_and_dragon_vect.push((fire_handle, dragon));
    }

    println!("# ##  ###   ####  M A I N III lizards  #####    ####   ###  ## #");

    for i in 50..90 {
        let id = format!("{}-{}", ObjType::Lizard, i);
        let lizard = create_lizard(
            id.clone(),
            format!("Green Lizard [{}]", id.clone()),
            food_reserve,
            number_of_claws,
            scale_colors.clone(),
            Arc::clone(&food_resource),
        );

        let lizard_clone = Arc::clone(&lizard);

        let fire_handle = thread::spawn(move || {
            loop {
                let mut lizard_lock = lizard_clone.lock().unwrap();

                let given_name = lizard_lock.get_given_name();
                let number_of_claws = lizard_lock.get_number_of_claws();
                let scale_colors = lizard_lock.get_scale_colors();

                let mut do_break = false;

                if lizard_lock.eat() {
                    println!("loop: Lizard {given_name} w {number_of_claws} claws and colors: {scale_colors} ATE.");
                } else {
                    println!("loop: Lizard {given_name} w {number_of_claws} claws and colors: {scale_colors} did NOT eat. BREAK.");
                    do_break = true;
                }

                drop(lizard_lock);

                if do_break {
                    break;
                }
            }
            let str = format!("{i}").as_str().to_owned();
            Ok(str)
        });

        crawl_handle_and_lizard_vect.push((fire_handle, lizard));
    }

    println!("# ##  ###   ####  M A I N IV birds  #####    ####   ###  ## #");

    for i in 90..130 {
        let id = format!("{}-{}", ObjType::Bird, i);
        let bird = create_bird(
            id.clone(),
            format!("Blue Bird [{}]", id.clone()),
            food_reserve,
            maximum_speed,
            wing_span,
            Arc::clone(&food_resource),
        );

        let bird_clone = Arc::clone(&bird);

        let fly_handle = thread::spawn(move || {
            loop {
                let mut bird_lock = bird_clone.lock().unwrap();

                let given_name = bird_lock.get_given_name();
                let wing_span = bird_lock.get_wing_span();
                let maximum_speed = bird_lock.get_maximum_speed();

                let mut do_break = false;

                if bird_lock.eat() {
                    println!("loop: Bird {given_name} w wing span: {wing_span} and maximum speed: {maximum_speed} ATE.");
                } else {
                    println!("loop: Bird {given_name} w wing span: {wing_span} and maximum speed: {maximum_speed} did NOT eat. BREAK.");
                    do_break = true;
                }

                drop(bird_lock);

                if do_break {
                    break;
                }
            }
            let str = format!("{i}").as_str().to_owned();
            Ok(str)
        });

        fly_handle_and_bird_vect.push((fly_handle, bird));
    }

    let mut outer_handles: Vec<std::thread::JoinHandle<Result<String, Error>>> = vec![];

    println!("# ##  ###   ####  M A I N V join/collect dragons  #####    ####   ###  ## #");

    for handle_and_dragon in fire_handle_and_dragon_vect {
        let handle = handle_and_dragon.0;
        let dragon_res = handle_and_dragon.1;

        let dragon_lock = dragon_res.lock().unwrap();

        println!("dragon_lock: {:?}", dragon_lock);

        drop(dragon_lock);

        let outer_handle = thread::spawn(move || {
            let result = handle.join();

            let s = match &result {
                Ok(s) => {
                    format!("Join: Ok: {:?}", s)
                }
                Err(msg) => {
                    format!("Join: Err: {:?}", msg)
                }
            };

            let str = format!("{:?}", dragon_res).as_str().to_owned();
            Ok(str)
        });

        outer_handles.push(outer_handle);
    }

    println!("# ##  ###   ####  M A I N VI join/collect lizards  #####    ####   ###  ## #");

    for handle_and_lizard in crawl_handle_and_lizard_vect {
        let handle = handle_and_lizard.0;
        let lizard_res = handle_and_lizard.1;

        let lizard_lock = lizard_res.lock().unwrap();

        println!("lizard_lock: {:?}", lizard_lock);

        drop(lizard_lock);

        let outer_handle = thread::spawn(move || {
            let result = handle.join();

            let s = match &result {
                Ok(s) => {
                    format!("Join: Ok: {:?}", s)
                }
                Err(msg) => {
                    format!("Join: Err: {:?}", msg)
                }
            };

            let str = format!("{:?}", lizard_res).as_str().to_owned();
            Ok(str)
        });

        outer_handles.push(outer_handle);
    }

    println!("# ##  ###   ####  M A I N VI join/collect birds  #####    ####   ###  ## #");

    for handle_and_bird in fly_handle_and_bird_vect {
        let handle = handle_and_bird.0;
        let bird_res = handle_and_bird.1;

        let bird_lock = bird_res.lock().unwrap();

        println!("bird_lock: {:?}", bird_lock);

        drop(bird_lock);

        let outer_handle = thread::spawn(move || {
            let result = handle.join();

            let s = match &result {
                Ok(s) => {
                    format!("Join: Ok: {:?}", s)
                }
                Err(msg) => {
                    format!("Join: Err: {:?}", msg)
                }
            };

            let str = format!("{:?}", bird_res).as_str().to_owned();
            Ok(str)
        });

        outer_handles.push(outer_handle);
    }

    println!("# ##  ###   ####  M A I N VI join/collect outer handles w dragons and lizards  #####    ####   ###  ## #");

    for outer_handle in outer_handles {
        let result = outer_handle.join();
        match result {
            Ok(s) => {
                println!("Join outer: Ok: {:?}", s);
            }
            Err(msg) => {
                println!("Join outer: Err: {:?}", msg);
            }
        }
    }
}
