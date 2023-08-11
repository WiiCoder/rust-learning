// method
/**
 * Rust 的方法往往跟结构体、枚举、特征一起使用
 *
 * 允许方法名和结构体字段名相同
 */

// method define
#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    // new 是 Circle 的关联函数， 因为第一个参数不是 self， 且 new 并不是关键字
    // 这种方法往往用于初始化当前结构体的实例
    // 返回 Self， 等价返回 Circle
    // 定义在 impl 中且没有 self 的函数被称为关联函数
    // Rust 约定俗成使用 new 作为构造器名称，出于设计上的考虑 Rust 没有特地用 new 作为关键字
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }

    // Circle的方法， &self表示借用当前的 Circle 结构体, &self 是 self: &Self 的简写
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    // 带多个参数的方法
    pub fn can_hold(&self, other: &Circle) -> bool {
        self.x > other.x && self.y > other.y && self.radius > other.radius
    }
}

/**
 * 允许对同一结构体重复定义 impl
 */
#[allow(unused)]
impl Circle {
    pub fn empty(&self) {}
}

// 为枚举实现方法
#[derive(Debug)]
#[allow(unused)]
enum Message {
    Quit,
    Move {x: i32,y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

pub fn method_example() {
    let circle = Circle::new(10.0, 10.0, 5.0);
    println!(
        "{:?}, x:{}, y:{}, radius:{}",
        circle, circle.x, circle.y, circle.radius
    );
    let area = circle.area();
    println!("area:{}", area);

    let other = Circle::new(5.0, 5.0, 2.5);
    println!("can hold:{}", circle.can_hold(&other));

    let msg = Message::Write(String::from("value"));
    msg.call();
}

pub fn main(){
    println!("========== {} ==========", "base_07: method");
    method_example();
    println!("========== {} ==========", "base_07: method");
}