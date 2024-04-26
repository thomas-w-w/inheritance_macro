# inheritance_macro

https://stackoverflow.com/questions/59018413/when-to-use-self-self-mut-self-in-methods

Summary table
Short for Equivalent to Takes ownership? When to use? Example

self self: Self a: A Yes The code that calls this method should not be able to reuse the object on which you call the method. Invalidate an object because maybe you transform it into another. Drop an object. Immutable data structures?

&self self: &Self a: &A No - immutably borrows For reading self or its fields. For printing something or compute a value based on multiple values of other copyable fields.

&mut self self: &mut Self a: &mut A No - mutable borrows For reading or writing self and its fields. A function that updates the internal state of a data structure.

mut self mut self: Self mut a: A Yes If you want to change what self points to. Not useful because it takes ownership.

https://jsdw.me/posts/rust-fn-traits/
