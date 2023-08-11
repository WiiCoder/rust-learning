// match

/**
 * 通用形式如下，match也是一个表达式
 * match target {
 *     模式1 => 表达式1,
 *     模式2 => {
 *         语句1;
 *         语句2;
 *         表达式2
 *     },
 *     _ => 表达式3
 * }
 *
 * match 必须穷尽所有情况
 * 当我们只关心几个匹配值，其他不匹配时可以用 _ 通配符， _ 类似 switch 中的 default
 *
 * 如果穷举类型使用了 #[derive(Debug)] 的情况下，也可以用一个变量承载，如下 Direction 匹配中的 oter
 *
 */
#[derive(Debug)]
#[allow(unused)]
enum Direction {
    East,
    West,
    North,
    South,
}

#[derive(Debug)]
#[allow(unused)]
enum UsState {
    Alabama,
    Alaska,
}

#[allow(unused)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[allow(unused)]
enum IpAddr {
    IPv4,
    IPv6,
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky boy");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // 绑定一个值
        Coin::Quarter(state) => {
            print!("State quarter from {:?}", state);
            25
        }
    }
}

// match 类似 switch， _ 类似 switch 中的 default
pub fn match_example() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("this is East"),
        Direction::North | Direction::South => {
            println!("this is North or South");
        }
        _ => println!("West"),
    };

    println!("{:?}", value_in_cents(Coin::Penny));
    println!("{:?}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    let ip = IpAddr::IPv6;
    let ip_str = match ip {
        IpAddr::IPv4 => "127.0.0.1",
        _ => "::1",
    };
    println!("{}", ip_str);

    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("print from (0, 0) move to ({}, {})", x, y);
            }
            Action::ChangeColorRGB(r, g, b) => {
                println!("R:{}, G:{}, B:{}", r, g, b);
            }
        }
    }

    match dire {
        Direction::East => println!("East"),
        other => println!("other direction: {:?}", other),
    };
}

// if let 匹配, 只匹配一个条件时且忽略其他条件时用 if let,否则都用 match
pub fn if_let_example() {
    let v = Some(3u8);
    if let Some(3) = v {
        println!("three");
    }
}

// matches! 宏
#[derive(Debug)]
enum MyEnum {
    Foo,
    Bar,
}

pub fn matches_macro_example() {
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    // 过滤 Foo
    let v = v.iter().filter(|x| matches!(x, MyEnum::Foo));
    println!("{:?}", v);

    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x >2));
}

/**
 * 变量遮蔽 Variable masking,
 * 无论是 match 还是 if let，这里都是一个新的代码块，而且这里的绑定相当于新变量，如果你使用同名变量，会发生变量遮蔽
 * match中的变量遮蔽不容易看出来，所以最好不要使用同名变量，不利于理解
 */
pub fn masking_example() {
    let age = Some(30);
    println!("before age is {:?}", age);
    if let Some(age) = age {
        println!("maching age is {:?}", age);
    }
    println!("after age is {:?}", age);

    let age = Some(31);
    println!("在匹配前，age是{:?}", age);
    match age {
        Some(x) => println!("匹配出来的age是{}", x),
        _ => (),
    }
    println!("在匹配后，age是{:?}", age);
}

// Option 枚举
/**
 * enum Option<T> {
 *  Some(T),
 *  None,
 * }
 * 一个变量要么有值：Some(T), 要么为空：None
 * 因为 Some 和 None 都包含在 prelude 中，所以可以直接使用，不用 Option::Some
 *
 */

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
pub fn option_example() {
    let five = Some(5);
    let six = plus_one(five);
    println!("six {:?}", six);
    let none = plus_one(None);
    println!("none {:?}", none);
}

pub fn main() {
    println!("========== {} ==========", "base_06: pattern matching");
    match_example();
    if_let_example();
    matches_macro_example();
    masking_example();
    option_example();
    println!("========== {} ==========", "base_06: pattern matching");
}
