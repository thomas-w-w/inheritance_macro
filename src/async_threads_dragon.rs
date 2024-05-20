use std::{
    fmt::{format, Error},
    mem,
    ops::DerefMut,
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

fn create_bird(index: u32, food_reserve: u32, food_resource: Arc<Mutex<FoodComponent>>) -> Bird {
    let obj = ObjComponent {
        obj_id: ObjComponent::new_id(),
        parent_id: None,
        obj_type: ObjType::Bird,
    };

    let animal = AnimalComponent {
        calories: food_reserve,
        given_name: format!("{} #{}", ObjType::Bird, index),
    };

    let egg_laying_animal = EgglayingAnimalComponent { eggs: INIT_EGGS };

    let bird = BirdComponent {};

    Bird::new(bird, animal, egg_laying_animal, obj, food_resource)
}

fn create_lizard(
    index: u32,
    food_reserve: u32,
    food_resource: Arc<Mutex<FoodComponent>>,
) -> Lizard {
    let obj = ObjComponent {
        obj_id: ObjComponent::new_id(),
        parent_id: None,
        obj_type: ObjType::Lizard,
    };

    let animal = AnimalComponent {
        calories: food_reserve,
        given_name: format!("{} #{}", ObjType::Lizard, index),
    };

    let egg_laying_animal = EgglayingAnimalComponent { eggs: INIT_EGGS };

    let lizard = LizardComponent {};

    Lizard::new(lizard, egg_laying_animal, animal, obj, food_resource)
}

fn create_dragon(
    index: u32,
    food_reserve: u32,
    fire_capacity: u32,
    food_resource: Arc<Mutex<FoodComponent>>,
) -> Dragon {
    let dragon = Dragon::new(
        DragonComponent {
            etanol_liters: fire_capacity,
        },
        BirdComponent {},
        LizardComponent {},
        EgglayingAnimalComponent { eggs: INIT_EGGS },
        AnimalComponent {
            calories: food_reserve,
            given_name: format!("{} #{}", ObjType::Dragon, index),
        },
        ObjComponent {
            obj_id: ObjComponent::new_id(),
            parent_id: None,
            obj_type: ObjType::Dragon,
        },
        food_resource,
    );
    dragon
}

pub async fn main_dragon() {
    let food_component = FoodComponent {
        food_capacity: 4000,
    };

    let food_resource: Arc<Mutex<FoodComponent>> = Arc::new(Mutex::new(food_component));

    let mut handles: Vec<JoinHandle<Result<String, Error>>> = vec![];

    let food_reserve: u32 = 10;

    for index in 1..4 {
        let bird: Arc<Mutex<Bird>> = Arc::new(Mutex::new(create_bird(
            index,
            food_reserve,
            Arc::clone(&food_resource),
        )));

        let handle = run_bird(Arc::clone(&bird));

        handles.push(handle);
    }

    for index in 1..4 {
        let lizard: Arc<Mutex<Lizard>> = Arc::new(Mutex::new(create_lizard(
            index,
            food_reserve,
            Arc::clone(&food_resource),
        )));

        let handle = run_lizard(Arc::clone(&lizard));

        handles.push(handle);
    }

    let fire_capacity: u32 = 0;

    for index in 1..4 {
        let dragon: Arc<Mutex<Dragon>> = Arc::new(Mutex::new(create_dragon(
            index,
            food_reserve,
            fire_capacity,
            Arc::clone(&food_resource),
        )));

        let handle = run_dragon(Arc::clone(&dragon));

        handles.push(handle);
    }

    join_handles(handles);
}

fn join_handles(handles: Vec<JoinHandle<Result<String, Error>>>) {
    for lizard_handle in handles {
        let result = lizard_handle.join();
        match result {
            Ok(s) => {
                //println!("\r\n\r\nJoin outer: Ok: {:?}", s);
            }
            Err(msg) => {
                println!("Join outer: Err: {:?}", msg);
            }
        }
    }
}

fn run_lizard(lizard_mutex: Arc<Mutex<Lizard>>) -> JoinHandle<Result<String, Error>> {
    let tag = "RUN_LIZARD";
    let handle = thread::spawn(move || {
        let mut child_handles: Vec<JoinHandle<Result<String, Error>>> = vec![];

        let mut child_index = 1;

        loop {
            let mut lizard_locked = lizard_mutex.lock().unwrap();

            let given_name = format!("{:?}", lizard_locked.get_given_name());

            let mut do_break = false;

            if lizard_locked.eat(100) {
                println!("{tag}: {given_name} ATE.\r\n");

                if let Some(mut child) = lizard_locked.try_reproduce() {
                    //not needed more
                    drop(lizard_locked);

                    let new_given_name = format!("{} #{}", child.get_given_name(), child_index);

                    child.set_given_name(new_given_name);

                    println!(
                        "{tag}: {given_name} REPRODUCED: {}.\r\n",
                        child.get_given_name()
                    );

                    let child_mutex: Arc<Mutex<Lizard>> = Arc::new(Mutex::new(child));

                    let child_handle = run_lizard(Arc::clone(&child_mutex));

                    child_handles.push(child_handle);

                    child_index += 1;
                } else {
                    println!("{tag}: {given_name} did NOT reproduce..\r\n");
                }
            } else {
                println!("{tag}: {given_name} did NOT eat. BREAK..\r\n");
                do_break = true;
            }

            if do_break {
                break;
            }
        }

        join_handles(child_handles);

        let lizard_locked = lizard_mutex.lock().unwrap();

        let given_name = lizard_locked.get_given_name();

        let result = format!(
            "{tag}: {} END THREAD\r\n {:?}\r\n",
            given_name, lizard_locked
        );

        println!("{}", result.clone());

        Ok(result)
    });

    handle
}

fn run_bird(bird_mutex: Arc<Mutex<Bird>>) -> JoinHandle<Result<String, Error>> {
    let tag = "RUN_BIRD";
    let handle = thread::spawn(move || {
        let mut child_handles: Vec<JoinHandle<Result<String, Error>>> = vec![];

        let mut child_index = 1;

        loop {
            let mut bird_locked = bird_mutex.lock().unwrap();

            let given_name = format!("{:?}", bird_locked.get_given_name());

            let mut do_break = false;

            if bird_locked.eat(100) {
                println!("{tag}: {given_name} ATE.\r\n");

                if let Some(mut child) = bird_locked.try_reproduce() {
                    //not needed more
                    drop(bird_locked);

                    let new_given_name = format!("{} #{}", child.get_given_name(), child_index);

                    child.set_given_name(new_given_name);

                    println!(
                        "{tag}: {given_name} REPRODUCED: {}.\r\n",
                        child.get_given_name()
                    );

                    let child_mutex: Arc<Mutex<Bird>> = Arc::new(Mutex::new(child));

                    let child_handle = run_bird(Arc::clone(&child_mutex));

                    child_handles.push(child_handle);

                    child_index += 1;
                } else {
                    println!("{tag}: {given_name} did NOT reproduce..\r\n");
                }
            } else {
                println!("{tag}: {given_name} did NOT eat. BREAK..\r\n");
                do_break = true;
            }

            if do_break {
                break;
            }
        }

        join_handles(child_handles);

        let bird_locked = bird_mutex.lock().unwrap();

        let given_name = bird_locked.get_given_name();

        let result = format!("{tag}: {} END THREAD\r\n {:?}\r\n", given_name, bird_locked);

        println!("{}", result.clone());

        Ok(result)
    });

    handle
}

fn run_dragon(dragon_mutex: Arc<Mutex<Dragon>>) -> JoinHandle<Result<String, Error>> {
    let tag = "RUN_DRAGON";
    let handle = thread::spawn(move || {
        let mut child_handles: Vec<JoinHandle<Result<String, Error>>> = vec![];

        let mut child_index = 1;

        loop {
            let mut dragon_locked = dragon_mutex.lock().unwrap();

            let given_name = format!("{:?}", dragon_locked.get_given_name());

            let mut do_break = false;

            if dragon_locked.fire() {
                println!("{tag}: {given_name} FIRED..\r\n");

                if let Some(mut child) = dragon_locked.try_reproduce() {
                    //not needed more
                    drop(dragon_locked);

                    let new_given_name = format!("{} #{}", child.get_given_name(), child_index);

                    child.set_given_name(new_given_name);

                    println!(
                        "{tag}: {given_name} REPRODUCED: {}.\r\n",
                        child.get_given_name()
                    );

                    let child_mutex: Arc<Mutex<Dragon>> = Arc::new(Mutex::new(child));

                    let child_handle = run_dragon(Arc::clone(&child_mutex));

                    child_handles.push(child_handle);

                    child_index += 1;
                } else {
                    println!("{tag}: {given_name} did NOT reproduce.\r\n");
                }
            } else {
                println!("{tag}: {given_name} did NOT fire. BREAK.\r\n");
                do_break = true;
            }

            if do_break {
                break;
            }
        }

        join_handles(child_handles);

        let dragon_locked = dragon_mutex.lock().unwrap();

        let given_name = dragon_locked.get_given_name();

        let result = format!(
            "{tag}: {} END THREAD\r\n {:?}.\r\n",
            given_name, dragon_locked
        );

        println!("{}", result.clone());

        Ok(result)
    });

    handle
}
