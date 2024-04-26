use inheritance_macro::*;

mod foons;

use foons::*;

trait IFoo {
    fn foo_num(&mut self) -> &mut i32;
    fn foo_str(&mut self) -> &mut String;
    fn foo_bln(&mut self) -> &mut bool;
}

trait IBar {
    fn bar_num(&mut self) -> &mut i32;
    fn bar_str(&mut self) -> &mut String;
    fn bar_bln(&mut self) -> &mut bool;
}

trait IPii: IFoo + IBar {
    fn pii_num(&mut self) -> &mut i32;
    fn pii_str(&mut self) -> &mut String;
    fn pii_bln(&mut self) -> &mut bool;
}

struct Foo {
    foo_num: i32,
    foo_str: String,
    foo_bln: bool,
}

struct Bar {
    bar_num: i32,
    bar_str: String,
    bar_bln: bool,
}

struct Pii {
    foo: Box<Foo>,
    bar: Box<Bar>,
    pii_num: i32,
    pii_str: String,
    pii_bln: bool,
}

impl IFoo for Foo {
    fn foo_num(&mut self) -> &mut i32 {
        &mut self.foo_num
    }

    fn foo_str(&mut self) -> &mut String {
        &mut self.foo_str
    }

    fn foo_bln(&mut self) -> &mut bool {
        &mut self.foo_bln
    }
}

impl IBar for Bar {
    fn bar_num(&mut self) -> &mut i32 {
        &mut self.bar_num
    }

    fn bar_str(&mut self) -> &mut String {
        &mut self.bar_str
    }

    fn bar_bln(&mut self) -> &mut bool {
        &mut self.bar_bln
    }
}

impl IFoo for Pii {
    fn foo_num(&mut self) -> &mut i32 {
        &mut self.foo.foo_num
    }

    fn foo_str(&mut self) -> &mut String {
        &mut self.foo.foo_str
    }

    fn foo_bln(&mut self) -> &mut bool {
        &mut self.foo.foo_bln
    }
}
impl IBar for Pii {
    fn bar_num(&mut self) -> &mut i32 {
        &mut self.bar.bar_num
    }

    fn bar_str(&mut self) -> &mut String {
        &mut self.bar.bar_str
    }

    fn bar_bln(&mut self) -> &mut bool {
        &mut self.bar.bar_bln
    }
}
impl IPii for Pii {
    fn pii_num(&mut self) -> &mut i32 {
        &mut self.pii_num
    }

    fn pii_str(&mut self) -> &mut String {
        &mut self.pii_str
    }

    fn pii_bln(&mut self) -> &mut bool {
        &mut self.pii_bln
    }
}

fn print_foo(
    foo_num: i32,
    foo_str: String,
    foo_bln: bool,
    bar_num: i32,
    bar_str: String,
    bar_bln: bool,
    pii_num: i32,
    pii_str: String,
    pii_bln: bool,
) {
    println!(
        "
        foo_num: {}, foo_str: {}, foo_bln: {}, 
        bar_num: {}, bar_str: {}, bar_bln: {}, 
        pii_num: {}, pii_str: {}, pii_bln: {}",
        foo_num, foo_str, foo_bln, bar_num, bar_str, bar_bln, pii_num, pii_str, pii_bln
    );
}
fn test_foo() {
    let foo_num: i32 = 10;
    let foo_str: String = "foo i".to_string();
    let foo_bln: bool = false;
    let bar_num: i32 = 10;
    let bar_str: String = "foo i".to_string();
    let bar_bln: bool = false;
    let pii_num: i32 = 10;
    let pii_str: String = "foo i".to_string();
    let pii_bln: bool = false;

    let mut pii = Pii {
        foo: Box::new(Foo {
            foo_num: foo_num,
            foo_str: foo_str,
            foo_bln: foo_bln,
        }),
        bar: Box::new(Bar {
            bar_num: bar_num,
            bar_str: bar_str,
            bar_bln: bar_bln,
        }),
        pii_num: pii_num,
        pii_str: pii_str,
        pii_bln: pii_bln,
    };

    let foo_num = pii.foo_num().clone();
    let foo_str = pii.foo_str().clone();
    let foo_bln = pii.foo_bln().clone();
    let bar_num = pii.bar_num().clone();
    let bar_str = pii.bar_str().clone();
    let bar_bln = pii.bar_bln().clone();
    let pii_num = pii.pii_num().clone();
    let pii_str = pii.pii_str().clone();
    let pii_bln = pii.pii_bln().clone();

    print_foo(
        foo_num,
        foo_str.clone(),
        foo_bln,
        bar_num,
        bar_str.clone(),
        bar_bln,
        pii_num,
        pii_str.clone(),
        pii_bln,
    );

    *pii.bar_bln() = match pii.bar_bln() {
        true => false,
        false => true,
    };

    *pii.foo_bln() = match pii.foo_bln() {
        true => false,
        false => true,
    };

    *pii.pii_bln() = match pii.pii_bln() {
        true => false,
        false => true,
    };

    *pii.foo_num() = 1000;
    *pii.bar_num() = 1000;
    *pii.pii_num() = 1000;

    *pii.foo_str() = "foo ii".to_string();
    *pii.bar_str() = "bar ii".to_string();
    *pii.pii_str() = "pii ii".to_string();

    let foo_num = pii.foo_num().clone();
    let foo_str = pii.foo_str().clone();
    let foo_bln = pii.foo_bln().clone();
    let bar_num = pii.bar_num().clone();
    let bar_str = pii.bar_str().clone();
    let bar_bln = pii.bar_bln().clone();
    let pii_num = pii.pii_num().clone();
    let pii_str = pii.pii_str().clone();
    let pii_bln = pii.pii_bln().clone();

    print_foo(
        foo_num, foo_str, foo_bln, bar_num, bar_str, bar_bln, pii_num, pii_str, pii_bln,
    );
}

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
    test_foo();
    test_dragon();
}

//fn do_work_w_dragon(dragon: Dragon) {}
