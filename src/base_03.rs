// 所有权和借用 Ownership and borrowing
pub fn example() {
    // 基础类型只是借用，在栈中存储的数据类型且存在Copy特性，在变量被赋给新变量时，仍然可用
    // 任何基本类型的组合可以 Copy ，不需要分配内存或某种形式资源的类型是可以 Copy 的
    let x = 1;
    let y = x;
    println!("{}, {}", x, y);

    // 复杂类型是所有权的转移
    let s1 = String::from("value");
    let s2 = s1;
    // println!("s1: {}, s2: {}", s1, s2); // s1 的所有权已经转移给了 s2，所以再次使用 s1 将出现异常
    println!("s2: {}", s2);
}

// 传值和返回 Pass and return values
pub fn pass_return() {
    let s = String::from("value");

    takes_ownership(s);
    // println!("再次使用 s:{}", s); // 出现异常，s 在takes_ownership已经被drop

    let x = 5;

    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}

// 引用与解引用
pub fn quote() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// 不可变引用
pub fn immutable_quote() {
    let s1 = String::from("value");
    let len = calculate_length(&s1);
    println!("{} length: {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// 可变引用
pub fn mutable_borrow() {
    let mut s = String::from("value");
    println!("change before:{}", s);
    change_borrow_value(&mut s);
    println!("change after:{}", s);
}

fn change_borrow_value(some_string: &mut String) {
    some_string.push('a');
}

pub fn repeat_mutable_borrow() {
    let mut s = String::from("value");

    let r1 = &mut s;
    println!("{}", r1);

    let r2 = &mut s;
    println!("{}", r2);

    // 一个可变引用在被移出作用域之前，无法创建新的可变引用
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{},{}", r1, r2);

    // 解决方式：1. 通过代码块的方式, 即 {} 大括号来限定作用域

    // 可变引用与不可变引用不能同时存在
    let mut s = String::from("value");

    // 如下， 当r1 和 r2作用未结束，则无法创建r3，r3是可变引用，将改变r1，r2原本的值
    let r1 = &s;
    let r2 = &s;
    println!("{},{}", r1, r2);
    let r3 = &mut s;

    println!("{}", r3);
}

pub fn main() {
    println!("========== {} ==========", "base_03: Ownership borrowing");
    example();
    pass_return();
    quote();
    immutable_quote();
    mutable_borrow();
    repeat_mutable_borrow();
    println!("========== {} ==========", "base_03: Ownership borrowing");
}
