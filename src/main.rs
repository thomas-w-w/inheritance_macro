use inheritance_macro::*;

fn main() {
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

//fn do_work_w_dragon(dragon: Dragon) {}
