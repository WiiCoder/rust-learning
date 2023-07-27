// 字符串和切片 String and slice
pub fn string_slice() {
    let s = String::from("value");
    let va = &s[0..2];
    let lue = &s[2..5];
    println!("s:{}, value: {}{}", s, va, lue);

    let _ = &s[..];
    let _ = &s[1..];

    // let mut sa: String = String::from("value");
    // let sp = &mut sa[..2];
    // println!("sp 1:{}", sp);
    // sa = String::from("aaaaa");
    // println!("sp 2:{}, sa:{}", sp, sa);
}

// 字符串和字符串字面量转换 Translate string and &str
pub fn string_str_translate() {
    let s = String::from("value");
    println!("&str: {}", &s);
    println!("$str[..2]: {}", &s[..2]);
    println!("String.as_str: {}", s.as_str());
}

// 字符串操作 String operation
pub fn string_operation() {
    let mut s = String::from("value ");
    s.push_str("string");
    println!("追加字符串: push_str() -> {} ", s);

    s.push('!');
    println!("追加字符: push() -> {} ", s);

    s.insert(5, ',');
    println!("插入字符: insert() -> {} ", s);

    s.insert_str(6, " insert");
    println!("插入字符串: insert_str() -> {} ", s);

    let mut s = s.replace("value", "VALUE");
    println!("替换字符串: replace() -> {} ", s);
    // dbg!(s);

    s.replace_range(7..8, "I");
    // dbg!(s);
    println!("替换字符串范围: replace_range() -> {} ", s);

    let pop1 = s.pop();
    let pop2 = s.pop();
    dbg!(pop1);
    dbg!(pop2);
    println!("删除最后的字符: pop() -> {} ", s);

    s.remove(0);
    println!("删除索引位置: remove() -> {} ", s);

    s.truncate(4);
    println!("从索引开始删除到结尾: remove() -> {} ", s);

    s.clear();
    dbg!(s);

    let concatenate_01 = String::from("value");
    let concatenate_02 = String::from(" is string");
    let result = concatenate_01 + &concatenate_02;
    let mut result = result + "!";
    result += "!!!";

    println!("使用 + 或 += 拼接：{}", result);

    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}", s1, s2);
    println!("通过格式化拼接字符串 {}", s);
}

// 字符串转义 String transfer
pub fn string_transfer() {
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE_STRUCK CAPITAL R\"";
    println!(
        "Unicode charactre {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行也保持格式、可以用 \ 忽略换行符
    let long_string = "String literals
    can span multiple lines.
    The linebreak and indentation here ->\
    <- can be escaped too!";
    println!("long_string: {}", long_string);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("no escape: {}", quotes);
}

// 操作 UTF-8 字符串 UTF-8 operation
pub fn string_utf8_operation() {
    for c in "中国移动".chars() {
        println!("char: {}", c);
    }

    for b in "中国移动".bytes() {
        println!("byte: {}", b);
    }
}

// 元组 tuple, 由多种类型组合，长度固定，顺序固定
pub fn tuple_example() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tuple x:{}, y:{}, z:{}", x, y, z);

    let one = tup.0;
    let two = tup.1;
    let three = tup.2;
    println!("tuple one:{}, two:{}, three:{}", one, two, three);

    let s1 = String::from("value");
    let (s2, len) = calculate_length(s1);
    println!("{} length: {}", s2, len);
}

// 通过元组的形式返回多个值
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// 结构体 struct
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

// 可以类似 JS 简写同名参数
fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

pub fn struct_example() {
    let mut user = User {
        active: true,
        username: String::from("Lisa"),
        email: String::from("value@qq.com"),
        sign_in_count: 10,
    };
    println!(
        "username: {}, email: {}, active: {}, signCount: {}",
        user.username, user.email, user.active, user.sign_in_count
    );

    user.username = String::from("Mike");
    println!("username: {}", user.username);

    let user1 = build_user(String::from("Tik"), String::from("abc@gmail.com"));
    let user2 = User {
        email: String::from("kk@163.com"),
        ..user1 // 类似 TS 的扩展运算符， 但是必须在尾部
    };
    println!("{:?}", user2);
    // 会报错，因为 user1 中的 String 类型的参数所有权已经转移，user1 已经无法使用
    // println!("{:?}", user1);

    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();
    println!("{:#?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}

// 元组结构体 Tuple Struct, 结构体字段没有名称，这种结构体叫 元组结构体
pub fn tuple_struct() {
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    println!("{:?}", black);

    let origin = Point(0, 0, 0);
    println!("{:#?}", origin);
}

// 单元结构体 没有任何字段和属性
// struct AlwaysEqual;

// let subject = AlwaysEqual;

// // 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
// impl SomeTrait for AlwaysEqual {

// }

// 枚举 enum
#[derive(Debug)]
enum PokerSuit {
    // Clubs(u8),
    // Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn enum_example() {
    let heart = PokerSuit::Hearts(1);
    let diamond = PokerSuit::Diamonds(2);
    println!("{:?}, {:?}", heart, diamond);

    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 2 };
    let m3 = Message::Write(String::from("value"));
    let m4 = Message::ChangeColor(99, 99, 99);
    println!("{:?}, {:?}, {:?}, {:?}", m1, m2, m3, m4);

    let some = Some(5);
    let absent: Option<i32> = None;
    println!("{:?}, {:?}", some, absent);
}

