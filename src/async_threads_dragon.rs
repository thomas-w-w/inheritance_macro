use std::{
    fmt::Error,
    mem,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

// use obj_fr_baseclass::{Animal, Bird, Dragon, Food, IDragon, Lizard, Obj, ObjType};

// use crate::{obj_fr_baseclass, IAnimal, IBird, ILizard, IObj};
use crate::model::{
    components::{
        animal_component::AnimalComponent,
        bird_component::BirdComponent,
        dragon_component::DragonComponent,
        egglaying_animal_component::{EgglayingAnimalComponent, INIT_EGGS},
        food_component::FoodComponent,
        lizard_component::LizardComponent,
        obj_component::{ObjComponent, ObjType},
    },
    entities::{
        bird_entity::BirdEntity as Bird, dragon_entity::DragonEntity as Dragon,
        lizard_entity::LizardEntity as Lizard,
    },
    traits::{animal_trait::AnimalTrait, dragon_trait::DragonTrait},
};

fn create_bird(food_reserve: u32, food_resource: Arc<Mutex<FoodComponent>>) -> Arc<Mutex<Bird>> {
    let obj = ObjComponent {
        obj_id: ObjComponent::new_id(),
        parent_id: None,
        obj_type: ObjType::Bird,
    };

    let animal = AnimalComponent {
        calories: food_reserve,
    };

    let egg_laying_animal = EgglayingAnimalComponent { eggs: INIT_EGGS };

    let bird = BirdComponent {};

    Arc::new(Mutex::new(Bird::new(
        bird,
        animal,
        egg_laying_animal,
        obj,
        food_resource,
    )))
}

fn create_lizard(food_reserve: u32, food_resource: Arc<Mutex<FoodComponent>>) -> Lizard {
    let obj = ObjComponent {
        obj_id: ObjComponent::new_id(),
        parent_id: None,
        obj_type: ObjType::Lizard,
    };

    let animal = AnimalComponent {
        calories: food_reserve,
    };

    let egg_laying_animal = EgglayingAnimalComponent { eggs: INIT_EGGS };

    let lizard = LizardComponent {};

    Lizard::new(lizard, egg_laying_animal, animal, obj, food_resource)
}

fn create_dragon(
    food_reserve: u32,
    fire_capacity: u32,
    food_resource: Arc<Mutex<FoodComponent>>,
) -> Arc<Mutex<Dragon>> {
    let dragon = Dragon::new(
        DragonComponent {
            etanol_liters: fire_capacity,
        },
        BirdComponent {},
        LizardComponent {},
        EgglayingAnimalComponent { eggs: INIT_EGGS },
        AnimalComponent {
            calories: food_reserve,
        },
        ObjComponent {
            obj_id: ObjComponent::new_id(),
            parent_id: None,
            obj_type: ObjType::Dragon,
        },
        food_resource,
    );
    Arc::new(Mutex::new(dragon))
}

pub async fn main_dragon() {
    println!("# ##  ###   ####  M A I N I  #####    ####   ###  ## #");

    let food_component = FoodComponent {
        food_capacity: 1000,
    };

    let food_resource: Arc<Mutex<FoodComponent>> = Arc::new(Mutex::new(food_component));

    let mut fire_handle_and_dragon_vect: Vec<(
        JoinHandle<Result<String, Error>>,
        Arc<Mutex<Dragon>>,
    )> = vec![];

    let lizard_handles: Arc<Mutex<Vec<(JoinHandle<Result<String, Error>>, Arc<Mutex<Lizard>>)>>> =
        Arc::new(Mutex::new(vec![]));

    let mut fly_handle_and_bird_vect: Vec<(JoinHandle<Result<String, Error>>, Arc<Mutex<Bird>>)> =
        vec![];

    let food_reserve: u32 = 100;
    let fire_capacity: u32 = 0;

    println!("# ##  ###   ####  M A I N II dragons  #####    ####   ###  ## #");

    // for i in 2..50 {
    for i in 1..1 {
        let id = format!("{}-{}", ObjType::Dragon, i);

        let dragon_pointer = create_dragon(food_reserve, fire_capacity, Arc::clone(&food_resource));

        let dragon_pointer_clone = Arc::clone(&dragon_pointer);

        let fire_handle = thread::spawn(move || {
            loop {
                let mut dragon = dragon_pointer_clone.lock().unwrap();

                let s = format!("{:?}", dragon);

                let mut do_break = false;

                if dragon.fire() {
                    println!("loop: Dragon [{s}] FIRED.");
                } else {
                    println!("loop: Dragon [{s}] did NOT fire. BREAK.");
                    do_break = true;
                }

                drop(dragon);

                if do_break {
                    break;
                }
            }
            let str = format!("{}", id.clone()).as_str().to_owned();
            Ok(str)
        });

        fire_handle_and_dragon_vect.push((fire_handle, dragon_pointer));
    }

    println!("# ##  ###   ####  M A I N III lizards  #####    ####   ###  ## #");

    // for i in 50..90 {
    for _i in 1..2 {
        let lizard: Arc<Mutex<Lizard>> = Arc::new(Mutex::new(create_lizard(
            food_reserve,
            Arc::clone(&food_resource),
        )));

        let lizard_handle = run_lizard(Arc::clone(&lizard), Arc::clone(&lizard_handles));

        // https://stackoverflow.com/a/69153739/24129232
        // To exchange values between threads without waiting for the thread to complete, you can use channels.
        // Function std::sync::mpsc::channel
        // https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html

        lizard_handles
            .lock()
            .unwrap()
            .push((lizard_handle, Arc::clone(&lizard)));
    }

    println!("# ##  ###   ####  M A I N IV birds  #####    ####   ###  ## #");

    // for i in 90..130 {
    for i in 90..90 {
        let id = format!("{}-{}", ObjType::Bird, i);

        let bird_pointer = create_bird(food_reserve, Arc::clone(&food_resource));

        let bird_pointer_clone = Arc::clone(&bird_pointer);

        let fly_handle = thread::spawn(move || {
            loop {
                let mut bird = bird_pointer_clone.lock().unwrap();

                let s = format!("{:?}", bird);

                let mut do_break = false;

                if bird.eat(100) {
                    println!("loop: Bird {s} ATE.");
                } else {
                    println!("loop: Bird {s} did NOT eat. BREAK.");
                    do_break = true;
                }

                drop(bird);

                if do_break {
                    break;
                }
            }
            let str = format!("{}", id.clone()).as_str().to_owned();
            Ok(str)
        });

        fly_handle_and_bird_vect.push((fly_handle, bird_pointer));
    }

    let outer_handles: Arc<Mutex<Vec<JoinHandle<Result<String, Error>>>>> =
        Arc::new(Mutex::new(vec![]));

    println!("# ##  ###   ####  M A I N V join/collect dragons  #####    ####   ###  ## #");

    for handle_and_dragon in fire_handle_and_dragon_vect {
        let handle = handle_and_dragon.0;
        let dragon_pointer = handle_and_dragon.1;

        let dragon = dragon_pointer.lock().unwrap();

        println!("dragon: {:?}", dragon);

        drop(dragon);

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

            //let str = format!("{:?}", dragon_pointer).as_str().to_owned();
            Ok(s)
        });

        outer_handles.lock().unwrap().push(outer_handle);
    }

    println!("# ##  ###   ####  M A I N VI join/collect lizards  #####    ####   ###  ## #");

    let lizard_handles_clone: Arc<
        Mutex<Vec<(JoinHandle<Result<String, Error>>, Arc<Mutex<Lizard>>)>>,
    > = Arc::clone(&lizard_handles);

    let outer_handles_clone: Arc<Mutex<Vec<JoinHandle<Result<String, Error>>>>> =
        Arc::clone(&outer_handles);

    let lizards_with_repro_handle = thread::spawn(move || {
        //https://stackoverflow.com/a/72018692/24129232
        let mut lizard_handles_locked = lizard_handles_clone.lock().unwrap();

        //https://users.rust-lang.org/t/how-can-i-dereference-an-arc-mutex-vec-for-return/92060/2
        //https://doc.rust-lang.org/std/mem/fn.take.html
        let lizard_handles_vec: Vec<(JoinHandle<Result<String, Error>>, Arc<Mutex<Lizard>>)> =
            mem::take(&mut *lizard_handles_locked);

        drop(lizard_handles_locked);

        for handle_and_lizard in lizard_handles_vec {
            let item = handle_and_lizard;

            let handle = item.0;
            let lizard = item.1;

            println!("lizard: {:?}", lizard);

            drop(lizard);

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

                //let str = format!("{:?}", lizard_pointer).as_str().to_owned();
                Ok(s)
            });

            outer_handles_clone.lock().unwrap().push(outer_handle);
        }
    });

    let all_done = lizards_with_repro_handle.join();

    println!("# ##  ###   ####  M A I N VI join/collect birds  #####    ####   ###  ## #");

    for handle_and_bird in fly_handle_and_bird_vect {
        let handle = handle_and_bird.0;
        let bird_pointer = handle_and_bird.1;

        let bird = bird_pointer.lock().unwrap();

        println!("bird: {:?}", bird);

        drop(bird);

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

            //let str = format!("{:?}", bird_pointer).as_str().to_owned();
            Ok(s)
        });

        outer_handles.lock().unwrap().push(outer_handle);
    }

    println!("# ##  ###   ####  M A I N VI join/collect outer handles w dragons and lizards  #####    ####   ###  ## #");

    let mut outer_handles_locked = outer_handles.lock().unwrap();

    let outer_handles_copy = mem::take(&mut *outer_handles_locked);

    drop(outer_handles_locked);

    for outer_handle in outer_handles_copy {
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

fn run_lizard(
    lizard_mutex: Arc<Mutex<Lizard>>,
    lizard_handles_mutex: Arc<Mutex<Vec<(JoinHandle<Result<String, Error>>, Arc<Mutex<Lizard>>)>>>,
) -> JoinHandle<Result<String, Error>> {
    let lizard_handle = thread::spawn(move || {
        let mut internal_lizard_handles: Vec<(
            JoinHandle<Result<String, Error>>,
            Arc<Mutex<Lizard>>,
        )> = vec![];

        loop {
            let mut lizard = lizard_mutex.lock().unwrap();

            let s = format!("{:?}", lizard);

            let mut do_break = false;

            if lizard.eat(100) {
                println!("loop: Lizard {s} ATE.");

                if let Some(child_lizard) = lizard.try_reproduce() {
                    //not needed more
                    drop(lizard);

                    println!("\r\nloop: Lizard {s} REPRODUCED.\r\n");

                    let child_lizard_mutex: Arc<Mutex<Lizard>> = Arc::new(Mutex::new(child_lizard));

                    let child_lizard_handle = run_lizard(
                        Arc::clone(&child_lizard_mutex),
                        Arc::clone(&lizard_handles_mutex),
                    );

                    internal_lizard_handles
                        .push((child_lizard_handle, Arc::clone(&child_lizard_mutex)));
                } else {
                    println!("loop: Lizard {s} did NOT reproduce.");
                }
            } else {
                println!("loop: Lizard {s} did NOT eat. BREAK.");
                do_break = true;
            }

            if do_break {
                break;
            }
        }
        let str = format!("{:?}", *lizard_mutex.lock().unwrap());
        Ok(str)
    });

    lizard_handle
}
