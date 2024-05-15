# inheritance_macro

https://stackoverflow.com/questions/59018413/when-to-use-self-self-mut-self-in-methods

Summary table
Short for Equivalent to Takes ownership? When to use? Example

self self: Self a: A Yes The code that calls this method should not be able to reuse the object on which you call the method. Invalidate an object because maybe you transform it into another. Drop an object. Immutable data structures?

&self self: &Self a: &A No - immutably borrows For reading self or its fields. For printing something or compute a value based on multiple values of other copyable fields.

&mut self self: &mut Self a: &mut A No - mutable borrows For reading or writing self and its fields. A function that updates the internal state of a data structure.

mut self mut self: Self mut a: A Yes If you want to change what self points to. Not useful because it takes ownership.

https://jsdw.me/posts/rust-fn-traits/

https://stackoverflow.com/questions/73195638/multiple-inheritance-for-oop-like-rust




/// https://en.wikipedia.org/wiki/Modern_C%2B%2B_Design#Policy-based_design
/// https://stackoverflow.com/a/31178549/24129232
/// https://softwareengineering.stackexchange.com/questions/100993/multiple-inheritance-use-cases
/// In C++ a good example of multiple inheritance used to composite orthogonal features is when you use CRTP to, for example, setup a component system for a game.
/// https://drdobbs.com/cpp/multiple-inheritance-considered-useful/184402074


Traits are similar to interfaces:

Traits are Rust’s sole notion of interface.

An interface is meant to document available methods, to have an interface with private methods makes no sense. Correspondingly, in Rust you can't have different levels of visibility in one trait. If you can see the trait, you can always see all of it. However, Rust traits are subtly different from interfaces: they combine declarations and implementations. I see how it would be intuitive to have a trait with some private functions.

For some time it was possible to split a trait into a public and private part. You would have two traits, one containing your public interface, the other with your private functionality, but this is being removed in newer versions of Rust.

The current workaround is still splitting the trait, but the private part must now be represented by a public trait within a private module. To explain this, here is some sample code:

https://stackoverflow.com/a/53207767/24129232

// this module contains a public trait Inc, to increment a value
// and it implements it by using a private trait Add
mod my_math {
    pub struct Val {
        pub val: i32,
    }

    // this is necessary to encapsulate the private trait
    // the module is private, so the trait is not exported
    mod private_parts {
        pub trait Add {
            fn add(&mut self, i32);
        }
    }

    // in the following code, we have to use adequate namespacing
    impl private_parts::Add for Val {
        fn add(&mut self, other: i32) {
            self.val += other;
        }
    }

    pub trait Inc: private_parts::Add {
        fn inc(&mut self);
    }

    impl Inc for Val {
        fn inc(&mut self) {
            use my_math::private_parts::Add;
            self.add(1)
        }
    }
}

fn main() {
    use my_math::Inc;
    let mut b = my_math::Val { val: 3 };
    println!("value: {}", b.val);
    b.inc();
    println!("value: {}", b.val);
}
//?
/// maybe delete/keep
//// DO DELETE
///// DO KEEP


ChronosWS — Idag 17:16
A couple items:
https://en.wikipedia.org/wiki/Entity_component_system
https://t-machine.org/index.php/2007/11/11/entity-systems-are-the-future-of-mmog-development-part-2/
Entity component system
Entity component system (ECS) is a software architectural pattern mostly used in video game development for the representation of game world objects. An ECS comprises entities composed from components of data, with systems which operate on the components.
ECS follows the principle of composition over inheritance, meaning that every entity is def...
Entity component system
The Bevy book also talks about it: https://bevy-cheatbook.github.io/programming/ecs-intro.html
Intro to ECS - Unofficial Bevy Cheat Book
All of these are game-oriented.
So I would look at them for ideas about how to separate data and logic, not necessarily that you should or must use this particular paragidm.  It'll give you ideas though
You can also look at Data-oriented Design, or Data-oriented Programming.