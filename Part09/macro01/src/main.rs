#[macro_export]
macro_rules! vec {
    ($( $x:expr ),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let x = vec![0, 1, 2, 3, 4, 5];
    println!("The vector : {:?}.", x);
}
