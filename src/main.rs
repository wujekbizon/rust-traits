mod basket;
mod container;
mod stack;
use basket::Basket;
use container::Container;
use stack::Stack;

fn add_string<T: Container<String>>(c: &mut T, s: String) {
    c.put(s);
}

fn main() {
    let mut b1 = Basket::new(String::from("hi there"));

    add_string(&mut b1, String::from("String"));

    let b2 = Basket::new(10);
    let b3 = Basket::new(b2);
    let b4 = Basket::new(String::from("stack item"));

    let s1 = Stack::new(vec![b1, b4]);
}
