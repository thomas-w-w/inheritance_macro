use inheritance_macro::*;

use traitcast::{Traitcast, TraitcastFrom};

mod foons;

use foons::*;

trait IObj {
    fn id(&self) -> u64;
    fn obj_type(&self) -> ObjectType;
}

trait IFoo: IObj {
    fn foo_num(&mut self) -> &mut i32;
    fn foo_str(&mut self) -> &mut String;
    fn foo_bln(&mut self) -> &mut bool;
}

trait IBar: IObj {
    fn bar_num(&mut self) -> &mut i32;
    fn bar_str(&mut self) -> &mut String;
    fn bar_bln(&mut self) -> &mut bool;
}

trait IPii: IFoo + IBar {
    fn pii_num(&mut self) -> &mut i32;
    fn pii_str(&mut self) -> &mut String;
    fn pii_bln(&mut self) -> &mut bool;
}
#[derive(Clone, Debug)]
pub enum ObjectType {
    Object,
    Foo,
    Bar,
    Pii,
}

//#[derive(ObjObj)]
#[derive(Clone, Debug)]
struct Obj {
    id: u64,
    obj_type: ObjectType,
}
impl IObj for Obj {
    fn id(&self) -> u64 {
        self.id
    }
    fn obj_type(&self) -> ObjectType {
        self.obj_type.to_owned()
    }
}
//#[derive(ObjObj)]
#[derive(Clone, Debug)]
struct Foo {
    obj: Box<Obj>,
    foo_num: i32,
    foo_str: String,
    foo_bln: bool,
}

#[derive(Clone, Debug)]
struct Bar {
    obj: Box<Obj>,
    bar_num: i32,
    bar_str: String,
    bar_bln: bool,
}

#[derive(Clone, Debug, ObjObj)]
struct Pii {
    foo: Box<Foo>,
    bar: Box<Bar>,
    pii_num: i32,
    pii_str: String,
    pii_bln: bool,
}
impl IObj for Foo {
    fn id(&self) -> u64 {
        self.obj.id()
    }

    fn obj_type(&self) -> ObjectType {
        self.obj.obj_type()
    }
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

impl IObj for Bar {
    fn id(&self) -> u64 {
        self.obj.id()
    }

    fn obj_type(&self) -> ObjectType {
        self.obj.obj_type()
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

impl IObj for Pii {
    fn id(&self) -> u64 {
        self.foo.id()
    }

    fn obj_type(&self) -> ObjectType {
        self.foo.obj_type()
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

    let obj = Obj {
        id: 1,
        obj_type: ObjectType::Pii,
    };

    let foo = Foo {
        obj: Box::new(obj.clone()),
        foo_num,
        foo_str,
        foo_bln,
    };

    let bar = Bar {
        obj: Box::new(obj),
        bar_num,
        bar_str,
        bar_bln,
    };

    let mut pii = Pii {
        foo: Box::new(foo),
        bar: Box::new(bar),
        pii_num,
        pii_str,
        pii_bln,
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

    println!("{:?}", pii);
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
