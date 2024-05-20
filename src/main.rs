mod async_threads_dragon;

use async_threads_dragon::*;

use futures::executor::block_on;

mod model;

fn main() {
    block_on(main_dragon());
    // // bird_main();
    // // lizard_main();
    // // dragon_main();
    // // dragon_entity_main();
}
