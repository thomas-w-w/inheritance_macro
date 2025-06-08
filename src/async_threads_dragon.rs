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

fn collect<T>(
    handles: Vec<JoinHandle<Result<Arc<Mutex<T>>, Error>>>,
    collection: Arc<Mutex<Vec<Arc<Mutex<T>>>>>,
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
                        "{tag}: {given_name} REPRO#####--------#######---####DUCED: {}.\r\n",
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

        collect::<Bird>(child_handles, collection);

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

        collect::<Lizard>(child_handles, collection);

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

        collect::<Dragon>(child_handles, collection);

        Ok(Arc::clone(&dragon_mutex))
    });

    handle
}

fn unwrap_vect<T>(collection: Arc<Mutex<Vec<Arc<Mutex<T>>>>>) -> Vec<Arc<Mutex<T>>> {
    let mut collection_locked = collection.lock().unwrap();

    let collection_copy: Vec<Arc<Mutex<T>>> = mem::take(&mut *collection_locked);

    drop(collection_locked);

    collection_copy
}

pub(crate) const MAIN_FOOD_CAPACITY: u32 = 1000;//500
pub(crate) const MAIN_FOOD_RESERVE: u32 = 1000;//500
pub(crate) const MAIN_FIRE_CAPACITY: u32 = 100;
pub(crate) const MAIN_NO_BIRDS: u32 = 4;//2
pub(crate) const MAIN_NO_LIZARDS: u32 = 0;//2
pub(crate) const MAIN_NO_DRAGONS: u32 = 0;//2


pub async fn main_dragon() {
    let food_component = FoodComponent { food_capacity: MAIN_FOOD_CAPACITY };

    let food_resource: Arc<Mutex<FoodComponent>> = Arc::new(Mutex::new(food_component));

    let bird_collection: Arc<Mutex<Vec<Arc<Mutex<Bird>>>>> = Arc::new(Mutex::new(vec![]));
    let lizard_collection: Arc<Mutex<Vec<Arc<Mutex<Lizard>>>>> = Arc::new(Mutex::new(vec![]));
    let dragon_collection: Arc<Mutex<Vec<Arc<Mutex<Dragon>>>>> = Arc::new(Mutex::new(vec![]));

    let mut bird_handles: Vec<JoinHandle<Result<Arc<Mutex<Bird>>, Error>>> = vec![];
    let mut lizard_handles: Vec<JoinHandle<Result<Arc<Mutex<Lizard>>, Error>>> = vec![];
    let mut dragon_handles: Vec<JoinHandle<Result<Arc<Mutex<Dragon>>, Error>>> = vec![];

    let food_reserve: u32 = MAIN_FOOD_RESERVE;

    for index in 1..MAIN_NO_BIRDS {
        let bird: Arc<Mutex<Bird>> = Arc::new(Mutex::new(create_bird(
            index,
            food_reserve,
            Arc::clone(&food_resource),
        )));

        let handle = run_bird(Arc::clone(&bird), Arc::clone(&bird_collection));

        bird_handles.push(handle);
    }

    for index in 1..MAIN_NO_LIZARDS {
        let lizard: Arc<Mutex<Lizard>> = Arc::new(Mutex::new(create_lizard(
            index,
            food_reserve,
            Arc::clone(&food_resource),
        )));

        let handle = run_lizard(Arc::clone(&lizard), Arc::clone(&lizard_collection));

        lizard_handles.push(handle);
    }

    let fire_capacity: u32 = MAIN_FIRE_CAPACITY;

    for index in 1..MAIN_NO_DRAGONS {
        let dragon: Arc<Mutex<Dragon>> = Arc::new(Mutex::new(create_dragon(
            index,
            food_reserve,
            fire_capacity,
            Arc::clone(&food_resource),
        )));

        let handle = run_dragon(Arc::clone(&dragon), Arc::clone(&dragon_collection));

        dragon_handles.push(handle);
    }

    collect::<Bird>(bird_handles, Arc::clone(&bird_collection));
    collect::<Lizard>(lizard_handles, Arc::clone(&lizard_collection));
    collect::<Dragon>(dragon_handles, Arc::clone(&dragon_collection));

    let bird_collection_copy = unwrap_vect::<Bird>(Arc::clone(&bird_collection));
    let lizard_collection_copy = unwrap_vect::<Lizard>(Arc::clone(&lizard_collection));
    let dragon_collection_copy = unwrap_vect::<Dragon>(Arc::clone(&dragon_collection));

    for bird_mutex in bird_collection_copy {
        let bird = bird_mutex.lock().unwrap();
        println!("\r\nSAVE: {}\r\n{:?}", bird.get_given_name(), *bird);
    }

    for lizard_mutex in lizard_collection_copy {
        let lizard = lizard_mutex.lock().unwrap();
        println!("\r\nSAVE: {}\r\n{:?}", lizard.get_given_name(), *lizard);
    }

    for dragon_mutex in dragon_collection_copy {
        let dragon = dragon_mutex.lock().unwrap();
        println!("\r\nSAVE: {}\r\n{:?}", dragon.get_given_name(), *dragon)
    }
}
