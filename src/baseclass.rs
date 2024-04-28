struct BaseClass {
    name: String,
}
impl BaseClass {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
trait BaseClassInterface {
    fn as_base(&self) -> &BaseClass;
    fn get_name(&self) -> &str {
        &self.as_base().name
    }
}
struct MiddleClass<T> {
    base: BaseClass,
    value: T,
}
impl<T> MiddleClass<T> {
    fn new(name: &str, value: T) -> Self {
        Self {
            base: BaseClass::new(name),
            value,
        }
    }
}
trait MiddleClassInterface {
    type Type;
    fn as_middle(&self) -> &MiddleClass<Self::Type>;
    fn get_value(&self) -> &Self::Type {
        &self.as_middle().value
    }
}
struct MyIntClass {
    middle: MiddleClass<i32>,
}
impl MyIntClass {
    fn new(name: &str, value: i32) -> Self {
        Self {
            middle: MiddleClass::new(name, value),
        }
    }
}
impl MiddleClassInterface for MyIntClass {
    type Type = i32;
    fn as_middle(&self) -> &MiddleClass<Self::Type> {
        &self.middle
    }
}

//############ NON-DYNAMIC PROGRAMMING #/*
impl<T> BaseClassInterface for T
where
    T: MiddleClassInterface,
{
    fn as_base(&self) -> &BaseClass {
        &self.as_middle().base
    }
}
fn print_my_class(my_class: MyIntClass) {
    println!(
        "name: {}, value: {}",
        my_class.get_name(),
        my_class.get_value()
    );
}
// */
//############ DYNAMIC PROGRAMMING #
/*
impl<T> BaseClassInterface for dyn MiddleClassInterface<Type = T> {
    fn as_base(&self) -> &BaseClass {
        &self.as_middle().base
    }
}
fn print_my_class(my_class: MyIntClass) {
    let my_dyn_class = &my_class as &dyn MiddleClassInterface<Type = i32>;
    println!(
        "name: {}, value: {}",
        my_dyn_class.get_name(),
        my_dyn_class.get_value()
    );
}
// */
pub fn baseclass_main() {
    let my_class: MyIntClass = MyIntClass::new("my_class", 1);

    print_my_class(my_class);
}
