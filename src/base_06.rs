// match
enum Direction {
    East,
    West,
    North,
    South,
}

// match 类似 switch， _ 类似 switch 中的 default
pub fn match_example() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("this is East"),
        Direction::North | Direction::South => {
            println!("this is North or South");
        },
        _ => println!("West"),
    };
}
