fn add_one(x: i32) -> i32 {
    x + 1
}

fn add_two(x: i32) -> i32 {
    x + 2
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer1 = do_twice(add_one, 5);
    println!("The answer is: {}", answer1);

    let answer2 = do_twice(add_two, 5);
    println!("The answer is: {}", answer2);
}
