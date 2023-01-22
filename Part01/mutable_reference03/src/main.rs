fn main() {
    let mut s = String::from("hello");
    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no
      // problems.
    let _r2 = &mut s;
}
