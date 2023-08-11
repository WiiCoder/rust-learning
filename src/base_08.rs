// 泛型和特征 generics and trait

use core::fmt;
use std::{
    fmt::{Debug, Display},
    ops::Add,
};

/**
 * 结构体中的泛型，使用同一种泛型参数的字段需要时同一类型
 * 可以存在不同的泛型参数
 */
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Pointu<T, U> {
    x: T,
    y: U,
}
/**
 * 枚举中的泛型
 */
#[allow(unused)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

/**
 * 结构体方法需要提前申明 imple<T>, 这样Rust才能识别Point<T>中的T时泛型，而不是具体类型
 */
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

/**
 * 定义额外的泛型参数
 * T, U 是结构体上的泛型参数
 * V，W 是方法上的泛型
 */
impl<T, U> Pointu<T, U> {
    fn mixup<V, W>(self, other: Pointu<V, W>) -> Pointu<T, W> {
        Pointu {
            x: self.x,
            y: other.y,
        }
    }
}

/**
 * 为具体的泛型类型实现方法，只有 f32 类型的 Point 可以使用
 */
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

/**
 * 数组泛型
 */
fn display_array<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr);
}

/**
 * const 泛型 此处用于改变数组泛型无法设置长度的问题
 *
 * const 泛型表达式： const N: unsize, 表示const泛型N，它基于的值类型是 unsize
 */
fn display_array2<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

pub fn generics_example() {
    let integer = Point { x: 5, y: 5 };
    // let float = Point { x: 1.0, y: 2.0 };

    println!("{}", integer.x());

    let flex = Pointu { x: 5, y: 1.0 };
    let flex2 = Pointu { x: 5, y: 1.0 };
    let flex = flex.mixup(flex2);

    println!("flex:{:?}", flex);

    let f32t = Point {
        x: 1.0f32,
        y: 2.0f32,
    };
    let v = f32t.distance_from_origin();
    println!("f32 T: {}", v);

    let arr = [1, 2, 3];
    let arr2 = [1, 2];

    display_array(&arr);
    display_array(&arr2);

    display_array2(arr);
    display_array2(arr2);
}

// 特征 Trait
/**
 * 特征定义一组可以被共享的行为，只要实现了特征，就能使用这组行为
 * 类似Java中的接口
 *
 * 特征 是孤儿规则，即定义和实现必须有一个是在当前作用域中定义的
 * 在当前作用域中 Summary 可以被Post实现，
 * 在当前作用域中 Post 也可以实现标准库中的 Display 特征，
 * 在当前作用域中 String 可以实现 Summary 特征
 * 在当前作用域中 String 无法实现 Display 特征，因为两者都不在当前作用域中定义！！！！
 */
trait Summary {
    // 没有默认实现
    fn summarize(&self) -> String;

    // 有默认实现， 类型实现了就覆盖，没实现就用默认的
    fn summarize_author(&self) -> String {
        String::from("Read more...")
    }
}

#[allow(unused)]
struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

// 为类型实现特征
// Post 实现 Summary 特征
impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章:{}, 作者:{}", self.title, self.author)
    }

    fn summarize_author(&self) -> String {
        format!("作者:{}", self.author)
    }
}

/**
 * 使用特征作为函数参数
 */
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/**
 * 上面的 notify 参数写法是语法糖，实际完整形式如下， T: Summary 被称为特征约束
 */
#[allow(unused)]
fn notify_1<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/**
 * 多重约束
 * 必须实现 Summary 和 Display 特征
 */
#[allow(unused)]
fn notify_2(item: &(impl Summary + Display)) {}

/**
 * Where 约束
 *
 * 当约束变多时，函数签名就回变得复杂，可以通过 where 改进
 */
#[allow(unused)]
fn smoe_funcation<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}

#[allow(unused)]
fn where_funcation<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

/**
 * 使用特征约束，有田间地实现方法嚯特征
 */
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x: x, y: y }
    }
}

/**
 * 限制 T 实现了 Display 和 PartialOrd 才能拥有此方法
 */
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("largest is x = {}", self.x);
        } else {
            println!("largest is y = {}", self.y);
        }
    }
}

/**
 * 函数返回中的 impl Trait
 * 它的限制是只能翻译一个具体的类型，比如在
 *
 * Post 实现了 Summary 特征，所以可以作为返回值 Pair 也实现了 Summary 特征，
 * 但是在如下函数中，不能通过判断之类的方式 进行返回两种类型
 * 如 if true {
 *  Post {}
 * } else {
 *  Pair {}
 * }
 * 无法通过编译
 */
#[allow(unused)]
fn returns_summarizable() -> impl Summary {
    Post {
        title: "title".to_string(),
        author: "author".to_string(),
        content: "content".to_string(),
    }
}

/**
 * 让 T 拥有 Copy 特征
 *
 * 如果不希望限制 只能用于实现了 Copy 特征的类型，可以指定 Clone 特征
 * 使用 clone 函数意味着类似 String 这种拥有堆上数据的类型会潜在地分配更多堆上空间，
 * 堆的分配在设计大量数据时可能相当缓慢
 */
#[allow(unused)]
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/**
 * 返回引用，可以不实现 clone 或 copy 特征
 */
#[allow(unused)]
fn largest_t<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }

    largest
}
/**
 * derive 派生特征
 * 如： #[drive(Debug)]
 *
 * 被 derive 标记的对象会自动实现对应的默认特征代码，继承相应功能
 *
 * 如 Debug 特征，标记后，就可以使用 println!("{:?}",s) 的形式打印结构体的对象
 * 如 Copy 特征，可以调用 copy 方法，进行自我复制
 */

pub fn trait_example() {
    let post = Post {
        title: "呐喊彷徨".to_string(),
        author: "鲁迅".to_string(),
        content: "".to_string(),
    };
    println!("{}", post.summarize());
    println!("{}", post.summarize_author());

    notify(&post);

    let pair = Pair::new(1, 2);
    pair.cmp_display();

    let a: i32 = 10;
    let b: u16 = 100;
    let b_ = b.try_into().unwrap();
    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}

// 综合例子
#[derive(Debug)] // 为 PointAdd 结构体派生 Debug 特征，用于格式化输出
struct PointAdd<T: Add<T, Output = T>> {
    // 限制类型 T 必须实现了 Add 特征，否则无法进行 + 操作
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for PointAdd<T> {
    type Output = PointAdd<T>;

    fn add(self, p: PointAdd<T>) -> PointAdd<T> {
        PointAdd {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

pub fn add_example() {
    let p1 = PointAdd {
        x: 1.1f32,
        y: 1.1f32,
    };
    let p2 = PointAdd {
        x: 2.2f32,
        y: 2.2f32,
    };

    //println!("{:?}", add(p1, p2));
    let p1 = p1.add(p2);
    println!("{:?}", p1);

    let p3 = PointAdd { x: 1i32, y: 1i32 };
    let p4 = PointAdd { x: 2i32, y: 2i32 };

    println!("34: {:?}", add(p3, p4));
}

// 格式化输出案例
#[derive(Debug, PartialEq)]
#[allow(unused)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
#[allow(unused)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "Close"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

pub fn display_example() {
    let f6 = File::new("f6.txt");
    //...
    println!("{:?}", f6);
    println!("{}", f6);
}

pub fn main() {
    println!("========== {} ==========", "base_08: generics and trait");
    generics_example();

    trait_example();

    add_example();
    display_example();
    println!("========== {} ==========", "base_08: generics and trait");
}
