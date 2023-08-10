// 生命周期 Life cycle

/**
 * 生命周期标注语法
 * 生命周期标注并不会改变任何引用的实际作用域 —— 鲁迅
 * 语法，以 ' 开头，名称往往是一个单独的小写字母，大多数人都用 'a 来作为生命周期的名称
 * &i32         一个引用
 * &‘a i32      具有显示生命周期的引用
 * &'a mut i32  具有显示生命周期的可变引用
 *
 * 一个生命周期标注本身不具有什么意义， 只是告诉编译器多个引用之间的关系
 * 它只能说明使用同一个标注的参数活的和 ‘a 一样久， 但是不知道到底活多久或哪个参数活更久
 *
 */
#[allow(unused)]
fn useless<'a>(first: &'a i32, second: &'a i32) {}

fn longest<'a>(x: &'a str, _: &str) -> &'a str {
    x
}

pub fn life_cycle_example() {
    useless(&10, &10);

    let c = longest(&"a", &"b ");
    println!("c: {}", c);

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
