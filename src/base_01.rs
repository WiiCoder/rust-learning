// 变量
pub fn variables() {
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6; // cannot mutate immutable variable `x`
    // println!("The value of x is: {}", x);

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

// 未使用常量警告，用下划线开头忽略
pub fn variable_unused_warn() {
    let _x = 5;
    // let y = 10; //  `#[warn(unused_variables)]` on by default
}

// 常量解构
pub fn variable_destruction() {
    let (a, mut b): (bool, bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);
}

pub struct Struct {
    e: i32,
}

// 常量解构式赋值
pub fn variable_destruction_assign() {
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    // 复制 5 位， c = 1, 省略中间，d = 4, _ 用于占位
    [c, .., d, _] = [1, 2, 3, 4, 5];

    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}

// 变量与常量的区别
pub fn variable_constant_discrepancy() {
    // 自始至终不可变，声明时必须标注类型。
    // 常量命名全大写，单词间下划线分隔
    // 数字字面量加入下划线提高可读性，即 100,000
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);
}

// 变量遮蔽 允许神明同名变量，后声明覆盖前面的
pub fn variable_shadowing() {
    let x = 5;
    // 使用第一个 x + 1 赋值新的 x
    let x = x + 1;
    {
        // 只在当前作用域生效
        let x = x * 2;
        println!("block x :{}", x);
    }
    println!("x: {}", x);
}

pub fn main() {
    println!("========== {} ==========", "base_01: variable start");
    variables();
    variable_unused_warn();
    variable_destruction();
    variable_destruction_assign();
    variable_constant_discrepancy();
    variable_shadowing();
    println!("========== {} ==========", "base_01: variable end");
}
