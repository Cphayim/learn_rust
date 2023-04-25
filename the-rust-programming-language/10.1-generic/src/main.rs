use crate::{largest::largest, point::Point};

pub mod largest;
pub mod point;

fn main() {
    let number_list = [34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let p = Point::new(5, 10);

    println!("p.x = {}", p.x());
}
