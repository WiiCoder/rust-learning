// 字符串和切片 String and slice
pub fn string_slice() {
    let s = String::from("value");
    let va = &s[0..2];
    let lue = &s[2..5];
    println!("s:{}, value: {}{}",s, va, lue);

    let _ = &s[..];
    let _ = &s[1..];
}
