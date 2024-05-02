use std::{
    borrow::BorrowMut,
    fmt::Error,
    sync::{Arc, Mutex},
    thread,
};

use obj_fr_baseclass::{Dragon, Food, IAnimal, IBird, IDragon, ILizard, ObjType};

use crate::obj_fr_baseclass;

pub async fn main_dragon() {
    let food_resource = Arc::new(Mutex::new(Food::new("Food-1", ObjType::Food, 10000)));

    let mut dragon = Dragon::new(
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

    let mut handle_and_dragon_vect: Vec<(
        std::thread::JoinHandle<Result<String, Error>>,
        Arc<Mutex<Dragon>>,
    )> = vec![];

    for i in 2..10 {
        let shared_food_resource: Arc<Mutex<Food>> = Arc::clone(&food_resource);

        let id = format!("{}-{}", ObjType::Dragon, i);
        let given_name = format!("Il Dragone [id: {id}]").to_string();
        let food_reserve = 50;
        let maximum_speed = 10;
        let wing_span = 34;
        let number_of_claws = 4;
        let scale_colors = "red green yellow".to_string();
        let fire_capacity = 100;

        let dragon = Dragon::new(
            &id,
            ObjType::Dragon,
            given_name,
            food_reserve,
            maximum_speed,
            wing_span,
            number_of_claws,
            scale_colors,
            fire_capacity,
        );

        let dragon_mutex = Mutex::new(dragon);

        let dragon_arc = Arc::new(dragon_mutex);

        let dragon_clone = Arc::clone(&dragon_arc);

        let handle = thread::spawn(move || {
            loop {
                let mut dragon_lock = dragon_clone.lock().unwrap();
                let given_name = dragon_lock.get_given_name().clone();

                let shared_food_resource_lock = shared_food_resource.lock().unwrap();
                let food_capacity = shared_food_resource_lock.food_capacity.clone();

                println!("Global food capacity: {food_capacity}, reported by Dragon: {given_name}");
                drop(shared_food_resource_lock);

                let ate: bool = dragon_lock.eat(shared_food_resource.clone()).clone();

                let mut do_break = false;

                if dragon_lock.fire(shared_food_resource.clone()) {
                    println!("Dragon {given_name} FIRED.");
                } else {
                    println!("Dragon {given_name} did NOT fire. BREAK.");
                    do_break = true;
                }

                drop(dragon_lock);

                if do_break {
                    break;
                }

                // println!(
                //     "inside lopp, given name: {}, global food capacity: {}, ate: {}",
                //     given_name, food_capacity, ate
                // );
            }
            let str = format!("{i}").as_str().to_owned();
            Ok(str)
        });

        handle_and_dragon_vect.push((handle, dragon_arc));
    }

    let mut outer_handles: Vec<std::thread::JoinHandle<Result<String, Error>>> = vec![];

    for handle_and_dragon in handle_and_dragon_vect {
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

            let dragon_lock = dragon_res.lock().unwrap();

            println!("inner joined: result: {:?}, s: {}, dragon-arc-mutex: {:?}, dragon_lock given name: {}", result, s.to_string(), dragon_res, dragon_lock.get_given_name());

            drop(dragon_lock);

            let str = format!("{:?}", dragon_res).as_str().to_owned();
            Ok(str)
        });

        outer_handles.push(outer_handle);
    }

    println!("# ##  ###   ####    #####    ####   ###  ## #");
    println!("# ##  ###   ####    #####    ####   ###  ## #");
    println!("# ##  ###   ####    #####    ####   ###  ## #");

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
