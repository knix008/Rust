use cons03::List::{ Cons, Nil };
fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("The list : {:?}.", list);
}
