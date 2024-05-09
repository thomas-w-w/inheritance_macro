use std::{
    borrow::BorrowMut,
    cell::RefCell,
    fmt::Error,
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

use obj_fr_baseclass::{
    Animal, Bird, Dragon, Food, IAnimal, IBird, IDragon, ILizard, Lizard, Obj, ObjType,
};

use crate::obj_fr_baseclass;

pub async fn main_dragon() {
    let food_resource = Arc::new(Mutex::new(Food::new(
        "Food-1".to_string(),
        ObjType::Food,
        500,
    )));

    let mut fire_handle_and_dragon_vect: Vec<(
        std::thread::JoinHandle<Result<String, Error>>,
        Arc<Mutex<Dragon>>,
    )> = vec![];

    for i in 2..3 {
        let shared_food_resource: Arc<Mutex<Food>> = Arc::clone(&food_resource);

        let id = format!("{}-{}", ObjType::Dragon, i);
        let given_name = format!("Il Dragone [id: {id}]").to_string();
        let food_reserve = 50;
        let maximum_speed = 10;
        let wing_span = 34;
        let number_of_claws = 4;
        let scale_colors = "red green yellow".to_string();
        let fire_capacity = 100;

        let obj = Arc::new(Mutex::new(Obj::new(id, ObjType::Dragon)));

        let animal = Arc::new(Mutex::new(Animal::new(
            Arc::clone(&obj),
            given_name.clone(),
            food_reserve,
            Arc::clone(&food_resource),
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

        let id_2 = format!("{}-{}", ObjType::Bird, i);

        let obj_2 = Arc::new(Mutex::new(Obj::new(id_2, ObjType::Dragon)));

        let animal_2 = Arc::new(Mutex::new(Animal::new(
            Arc::clone(&obj_2),
            given_name.clone(),
            food_reserve,
            Arc::clone(&food_resource),
        )));

        let bird_2 = Arc::new(Mutex::new(Bird::new(
            Arc::clone(&animal_2),
            maximum_speed,
            wing_span,
        )));

        let dragon = Arc::new(Mutex::new(Dragon::new(
            Arc::clone(&bird),
            Arc::clone(&lizard),
            fire_capacity,
        )));

        let dragon_clone = Arc::clone(&dragon);

        let bird_2_clone = Arc::clone(&bird_2);

        let bird_2_handle = thread::spawn(move || {
            println!("loop BIRD: start");

            let mut bird_2_lock = bird_2_clone.lock().unwrap();

            let bird_2_given_name = bird_2_lock.get_given_name().clone();

            println!("loop BIRD: bird_2_given_name: {}.", bird_2_given_name);

            let bird_2_wing_span = bird_2_lock.get_wing_span().clone();

            println!("loop BIRD: bird_2_wing_span: {}.", bird_2_wing_span);

            println!(
                "///////////////////////////
/// loop BIRD: bird_2_lock, before eat: {:?}.
//////////////////////////////",
                bird_2_lock
            );
            bird_2_lock.eat();
            println!(
                "\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
\\\\\\\\\\ loop BIRD: bird_2_lock, AFTER eat: {:?}.
\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\",
                bird_2_lock
            );
        });

        if i == 2 {
            let bird_3_clone = Arc::clone(&bird);

            let bird_3_handle = thread::spawn(move || loop {
                println!("loop BIRD: start");

                let mut bird_3_lock = bird_3_clone.lock().unwrap();

                let bird_3_given_name = bird_3_lock.get_given_name().clone();

                println!("loop BIRD: bird_3_given_name: {}.", bird_3_given_name);

                println!(
                    "///////////////////////////
    /// loop BIRD: bird_3_lock, before eat: {:?}.
    //////////////////////////////",
                    bird_3_lock
                );
                bird_3_lock.eat();
                println!(
                    "\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
    \\\\\\\\\\ loop BIRD: bird_3_lock, AFTER eat: {:?}.
    \\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\",
                    bird_3_lock
                );
            });
        }

        let bird_2_clone = Arc::clone(&bird_2);

        let fire_handle = thread::spawn(move || {
            loop {
                println!("loop: start");

                let mut dragon_lock = dragon_clone.lock().unwrap();

                let mut bird_2_lock = bird_2_clone.lock().unwrap();

                let bird_2_given_name = bird_2_lock.get_given_name().clone();

                println!("loop: bird_2_given_name: {}.", bird_2_given_name);

                println!(
                    "///////////////////////////
/// loop: bird_2_lock, before eat: {:?}.
//////////////////////////////",
                    bird_2_lock
                );
                bird_2_lock.eat();
                println!(
                    "\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
\\\\\\\\\\ loop: bird_2_lock, AFTER eat: {:?}.
\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\",
                    bird_2_lock
                );

                let given_name = dragon_lock.get_given_name().clone();

                println!(
                    "loop: after get_given_name(): {}, before shared food lock",
                    given_name
                );

                let shared_food_resource_lock = shared_food_resource.lock().unwrap();

                drop(shared_food_resource_lock);

                let mut do_break = false;

                if dragon_lock.fire() {
                    println!("loop: Dragon {given_name} FIRED.");
                } else {
                    println!("loop: Dragon {given_name} did NOT fire. BREAK.");
                    do_break = true;
                }

                // // let dragon_as_bird = dragon_lock.as_bird().lock().unwrap();
                // // let t = dragon_as_bird.get_wing_span();

                let wing_span = dragon_lock.get_wing_span();

                println!("loop: wing_span: {wing_span}");

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

    let mut outer_handles: Vec<std::thread::JoinHandle<Result<String, Error>>> = vec![];

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
