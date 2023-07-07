#![allow(dead_code)]
pub fn serve_order() {
    println!("Calling server_order()!!!");
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
}
