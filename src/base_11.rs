// 生命周期 Life cycle

use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
    net::IpAddr,
};

/**
 * 传播错误， 但是这种写法太长了
 */
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("path.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

/**
 * 等效上一种写法
 * ？ 是一个宏，等效 match
 */
fn simple() -> Result<String, io::Error> {
    let mut f = File::open("path.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s)
}

/**
 * 等效上两种种写法，更短了
 * ？还可以链式调用
 */
fn simple2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("path.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn simple3() -> Result<String, io::Error> {
    fs::read_to_string("path.txt")
}

pub fn main() {
    // panic!("??????")
    // let v = vec![1, 2, 3];

    // v[99];

    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("{:?}", home);

    let a = read_username_from_file();
    match a {
        Ok(str) => println!("file content: {}", str),
        Err(e) => println!("file open err {:?}", e),
    }

    let f = File::open("path.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("path.txt") {
                Ok(fc) => fc,
                Err(fce) => panic!("Problem creating the file {:?}", fce),
            },
            oter_error => panic!("Problem opening the file {:?}", oter_error),
        },
    };

    println!("file: {:?}", f);

    println!("simple :{:?}", simple());
    println!("simple2 :{:?}", simple2());
    println!("simple3 :{:?}", simple3());
}
