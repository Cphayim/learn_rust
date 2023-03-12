fn main() {
    println!("Hello, world!");
    another_function(5 as f32);
    println!("Five: {}", five());
}

fn another_function(x: f32) {
    println!("Another function: {x}");
}

fn five() -> i32 {
    print!("");
    5
}
