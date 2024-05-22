use std::{
    fmt::Error,
    mem,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

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

fn collect_birds(
    handles: Vec<JoinHandle<Result<Arc<Mutex<Bird>>, Error>>>,
    collection: Arc<Mutex<Vec<Arc<Mutex<Bird>>>>>,
) {
    for handle in handles {
        let join_result = handle.join();
        match join_result {
            Ok(result) => {
                collection.lock().unwrap().push(result.unwrap());
            }
            Err(msg) => {
                println!("Join outer: Err: {:?}\r\n", msg);
            }
        }
    }
}

fn collect_lizards(
    handles: Vec<JoinHandle<Result<Arc<Mutex<Lizard>>, Error>>>,
    collection: Arc<Mutex<Vec<Arc<Mutex<Lizard>>>>>,
) {
    for handle in handles {
        let join_result = handle.join();
        match join_result {
            Ok(result) => {
                collection.lock().unwrap().push(result.unwrap());
            }
            Err(msg) => {
                println!("Join outer: Err: {:?}\r\n", msg);
            }
        }
    }
}
fn collect_dragons(
    handles: Vec<JoinHandle<Result<Arc<Mutex<Dragon>>, Error>>>,
    collection: Arc<Mutex<Vec<Arc<Mutex<Dragon>>>>>,
) {
    for handle in handles {
        let join_result = handle.join();
        match join_result {
            Ok(result) => {
                collection.lock().unwrap().push(result.unwrap());
            }
            Err(msg) => {
                println!("Join outer: Err: {:?}\r\n", msg);
            }
        }
    }
}

fn run_bird(
    bird_mutex: Arc<Mutex<Bird>>,
    collection: Arc<Mutex<Vec<Arc<Mutex<Bird>>>>>,
) -> JoinHandle<Result<Arc<Mutex<Bird>>, Error>> {
    let bird_mutex_locked = bird_mutex.lock().unwrap();
    let given_name = bird_mutex_locked.get_given_name();
    drop(bird_mutex_locked);

    let tag = format!("RUN_BIRD[{given_name}]");

    let handle = thread::spawn(move || {
        let mut child_handles: Vec<JoinHandle<Result<Arc<Mutex<Bird>>, Error>>> = vec![];

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

                    let child_handle = run_bird(Arc::clone(&child_mutex), Arc::clone(&collection));

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

        collect_birds(child_handles, collection);

        Ok(Arc::clone(&bird_mutex))
    });

    handle
}

fn run_lizard(
    lizard_mutex: Arc<Mutex<Lizard>>,
    collection: Arc<Mutex<Vec<Arc<Mutex<Lizard>>>>>,
) -> JoinHandle<Result<Arc<Mutex<Lizard>>, Error>> {
    let lizard_mutex_locked = lizard_mutex.lock().unwrap();
    let given_name = lizard_mutex_locked.get_given_name();
    drop(lizard_mutex_locked);

    let tag = format!("RUN_LIZARD[{given_name}]");

    let handle = thread::spawn(move || {
        let mut child_handles: Vec<JoinHandle<Result<Arc<Mutex<Lizard>>, Error>>> = vec![];

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

                    let child_handle =
                        run_lizard(Arc::clone(&child_mutex), Arc::clone(&collection));

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

        collect_lizards(child_handles, collection);

        Ok(Arc::clone(&lizard_mutex))
    });

    handle
}

fn run_dragon(
    dragon_mutex: Arc<Mutex<Dragon>>,
    collection: Arc<Mutex<Vec<Arc<Mutex<Dragon>>>>>,
) -> JoinHandle<Result<Arc<Mutex<Dragon>>, Error>> {
    let dragon_mutex_locked = dragon_mutex.lock().unwrap();
    let given_name = dragon_mutex_locked.get_given_name();
    drop(dragon_mutex_locked);

    let tag = format!("RUN_DRAGON[{given_name}]");

    let handle = thread::spawn(move || {
        let mut child_handles: Vec<JoinHandle<Result<Arc<Mutex<Dragon>>, Error>>> = vec![];

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

                    let child_handle =
                        run_dragon(Arc::clone(&child_mutex), Arc::clone(&collection));

                    child_handles.push(child_handle);

                    child_index += 1;
                } else {
                    println!("{tag}: {given_name} did NOT reproduce..\r\n");
                }
            } else {
                println!("{tag}: {given_name} did NOT fire. BREAK.\r\n");
                do_break = true;
            }

            if do_break {
                break;
            }
        }

        collect_dragons(child_handles, collection);

        Ok(Arc::clone(&dragon_mutex))
    });

    handle
}

pub async fn main_dragon() {
    let food_component = FoodComponent {
        food_capacity: 4000,
    };

    let bird_collection: Arc<Mutex<Vec<Arc<Mutex<Bird>>>>> = Arc::new(Mutex::new(vec![]));
    let lizard_collection: Arc<Mutex<Vec<Arc<Mutex<Lizard>>>>> = Arc::new(Mutex::new(vec![]));
    let dragon_collection: Arc<Mutex<Vec<Arc<Mutex<Dragon>>>>> = Arc::new(Mutex::new(vec![]));

    let food_resource: Arc<Mutex<FoodComponent>> = Arc::new(Mutex::new(food_component));

    let mut bird_handles: Vec<JoinHandle<Result<Arc<Mutex<Bird>>, Error>>> = vec![];
    let mut lizard_handles: Vec<JoinHandle<Result<Arc<Mutex<Lizard>>, Error>>> = vec![];
    let mut dragon_handles: Vec<JoinHandle<Result<Arc<Mutex<Dragon>>, Error>>> = vec![];

    let food_reserve: u32 = 100;

    for index in 1..2 {
        let bird: Arc<Mutex<Bird>> = Arc::new(Mutex::new(create_bird(
            index,
            food_reserve,
            Arc::clone(&food_resource),
        )));

        let handle = run_bird(Arc::clone(&bird), Arc::clone(&bird_collection));

        bird_handles.push(handle);
    }

    for index in 1..2 {
        let lizard: Arc<Mutex<Lizard>> = Arc::new(Mutex::new(create_lizard(
            index,
            food_reserve,
            Arc::clone(&food_resource),
        )));

        let handle = run_lizard(Arc::clone(&lizard), Arc::clone(&lizard_collection));

        lizard_handles.push(handle);
    }

    let fire_capacity: u32 = 100;

    for index in 1..2 {
        let dragon: Arc<Mutex<Dragon>> = Arc::new(Mutex::new(create_dragon(
            index,
            food_reserve,
            fire_capacity,
            Arc::clone(&food_resource),
        )));

        let handle = run_dragon(Arc::clone(&dragon), Arc::clone(&dragon_collection));

        dragon_handles.push(handle);
    }

    collect_birds(bird_handles, Arc::clone(&bird_collection));
    collect_lizards(lizard_handles, Arc::clone(&lizard_collection));
    collect_dragons(dragon_handles, Arc::clone(&dragon_collection));

    //copy vect of lizards
    let mut arc_birds_vec_locked = bird_collection.lock().unwrap();

    let arc_birds_vec_copy = mem::take(&mut *arc_birds_vec_locked);

    drop(arc_birds_vec_locked);

    for bird_mutex in arc_birds_vec_copy {
        let bird = bird_mutex.lock().unwrap();
        println!("\r\nSAVE: {}\r\n{:?}", bird.get_given_name(), *bird);
    }

    let mut arc_lizards_vec_locked = lizard_collection.lock().unwrap();

    let arc_lizards_vec_copy = mem::take(&mut *arc_lizards_vec_locked);

    drop(arc_lizards_vec_locked);

    for lizard_mutex in arc_lizards_vec_copy {
        let lizard = lizard_mutex.lock().unwrap();
        println!("\r\nSAVE: {}\r\n{:?}", lizard.get_given_name(), *lizard);
    }

    let mut arc_dragons_vec_locked = dragon_collection.lock().unwrap();

    let arc_dragons_vec_copy = mem::take(&mut *arc_dragons_vec_locked);

    drop(arc_dragons_vec_locked);

    for dragon_mutex in arc_dragons_vec_copy {
        let dragon = dragon_mutex.lock().unwrap();
        println!("\r\nSAVE: {}\r\n{:?}", dragon.get_given_name(), *dragon)
    }
}
