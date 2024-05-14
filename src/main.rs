mod async_threads_dragon;
mod baseclass;
mod dragon;
mod foo_bar_obj;
mod foons;
mod obj_fr_baseclass;

use inheritance_macro::*;
use traitcast::{Traitcast, TraitcastFrom};

use async_threads_dragon::*;
use baseclass::*;
use foo_bar_obj::*;
use foons::*;
use obj_fr_baseclass::*;

use futures::executor::block_on;

use crate::dragon::bird::*;
use crate::dragon::lizard::*;

use dragon::*;

fn test_dragon() {
    // make_foo!(Animal, IAnimal, (given_name: String));
    // make_foo!(Bird, IBird, (wing_span: i32));
    // make_foo!(Lizard, ILizard, (number_claws: i32));
    // make_foo!(Dragon, IDragon, (fire_capacity: i32));

    // make_foo!(Animal, (given_name: String));
    // make_foo!(Bird, (wing_span: i32));
    // make_foo!(Lizard, (number_claws: i32));
    make_foo!
    (
        Dragon,
        IDragon,
        (
            (
                fire_capacity: i32,
                get_fire_capacity: set_fire_capacity
            ),
            (
                wing_span: i32,
                get_wing_span: set_wing_span
            )
        )
    );

    impl Dragon {
        fn fire(&mut self) {
            self.set_fire_capacity(self.get_fire_capacity() - 10);
        }
    }

    let mut dragon = Dragon {
        id: 1,
        fire_capacity: 100,
        wing_span: 10,
    };

    dragon.fire();

    println!("Dragon fire capacity: {}", dragon.get_fire_capacity());
}
fn main() {
    // // test_foo();
    // // test_dragon();
    // // baseclass_main();
    // //obj_main();
    // //
    // // block_on(main_dragon());
    bird_main();
    lizard_main();
    dragon_main();
}

//fn do_work_w_dragon(dragon: Dragon) {}
