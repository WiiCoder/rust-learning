use std::fmt::Debug;

use num::complex::Complex;

// Base type 基础类型
pub fn int_type() {
    let a: i8 = 1;
    let b: u8 = 2;
    println!("i8 a:{:?}, u8 b:{:?}", a, b);

    let a: i16 = 1;
    let b: u16 = 2;
    println!("i16 a:{:?}, u16 b:{:?}", a, b);

    let a: i32 = 1;
    let b: u32 = 2;
    println!("i32 a:{:?}, u32 b:{:?}", a, b);

    let a: i64 = 1;
    let b: u64 = 2;
    println!("i64 a:{:?}, u64 b:{:?}", a, b);

    let a: i128 = 1;
    let b: u128 = 2;
    println!("i128 a:{:?}, u128 b:{:?}", a, b);

    let a: isize = 1;
    let b: usize = 2;
    println!("isize a:{:?}, usize b:{:?}", a, b);
}

// 整型溢出
pub fn int_overflow() {
    let a: u8 = 255;
    let b = a.wrapping_add(20);
    println!("b:{}", b);
}

// 浮点类型
pub fn float_type() {
    // 默认 f64, f32 单精度、 f64 双精度
    let _x = 2.0;
    let _y: f32 = 3.0;
    // assert!(0.1 + 0.2 == 0.3); // 实际的二进制精度问题导致 panicked

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("  0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("        0.3: {:x}", (abc.2).to_bits());
    println!("xyz (f64)");
    println!("  0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("        0.3: {:x}", (xyz.2).to_bits());
    assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2); // f64精度更高，所以出现 panicked
}

// NaN
pub fn float_non() {
    // 所有和 NaN 交互的操作，都会返回一个 NaN，且不能比较
    let x = (-32.0_f32).sqrt();
    // assert_eq!(x, x); // panicked

    if x.is_nan() {
        println!("异常数学行为");
    }
}

// 数学运算
pub fn number_operation() {
    let sum = 2 + 1;
    let difference = 5 - 1;
    let product = 4 * 2;
    let quotient = 25.1 / 2.4;
    let remainder = 43 % 5;

    println!(
        "sum:{}, difference:{}, product:{}, quotient:{}, remainder:{}",
        sum, difference, product, quotient, remainder
    );

    let addition = sum + difference;
    println!("addition:{}", addition);

    let one_million: i64 = 1_000_000;
    println!("one_million:{}", one_million.pow(2));
}

// 位运算
pub fn bit_operation() {
    let a: i32 = 2;
    let b = 3i32;

    println!("(a & b) = {}", a & b);
    println!("(a | b) = {}", a | b);
    println!("(a ^ b) = {}", a ^ b);
    println!("(!b) = {}", !b);
    println!("(a << b) = {}", a << b);
    println!("(a >> b) = {}", a >> b);

    let mut a = a;
    a <<= b;
    println!("(a << b) = {}", a);
}

// 序列 range
pub fn for_range() {
    for i in 1..=5 {
        println!("number range: {}", i);
    }

    for i in 'a'..='z' {
        println!("string range: {}", i);
    }
}

// 有理数和复数 Rational numbers and complex numbers
pub fn rational_complex() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);
}

// 字符类型 Rust 字符使用了 ASCII 和 Unicode, 所以字符类型占用4字节, 字符用‘’, 字符串用 ""
pub fn char_type() {
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
    println!("{}, {}, {}, {}", c, z, g, heart_eyed_cat);

    println!("字符 'c' 占用内存: {}", std::mem::size_of_val(&c));
}

// 布尔类型
pub fn bool_type() {
    let _t = true;

    let f: bool = false; // 使用类型标注,显式指定f的类型

    if f {
        println!("这是段毫无意义的代码");
    }
}

// 单元类型 - (), main 函数的返回值也是 (), 并不是无返回值， 可用于占位，不关注值，不占用任何内存

// 语句和表达式 Statements and expressions
fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句， 执行操作，但不会返回一个值
    let y = y + 5; // 语句， 执行操作，但不会返回一个值
    x + y // 表达式, 总要返回值，不能包含分号
}

pub fn statement_expression() {
    let sum = add_with_extra(2, 4);
    println!("sum: {}", sum);

    // 语句
    let _a = 9;
    let _b: Vec<f64> = Vec::new();
    let (_a, _c) = ("hi", false);
    // let b = (let a = 0); // let 是语句，无法赋值给其他变量

    // 用语句块表达式赋值给变量
    let _y = {
        let x = 3; // 语句
        x + 1 // 表达式
    };

    // if 语句块也是一个表达式，也可以用于赋值
    let _y = if sum % 2 == 1 { "odd" } else { "even" };
}

// 函数, 函数名和变量名使用 蛇形命名法
fn fu_add(i: i32, j: i32) -> i32 {
    i + j
}

// 无返回值()
pub fn fn_report<T: Debug>(item: T) {
    fu_add(1,2);
    println!("{:?}", item);
}

// 显示返回 ()
pub fn fn_clear(text: &mut String) -> (){
    *text = String::from("");
}

// 没有返回值的函数，发散函数 '!'
pub fn fn_dead_end() -> ! {
    panic!("😡崩溃吧！");
}

// 无法跳出循环，也是永不返回
pub fn fn_forever() -> ! {
    loop {
        
    }
}
