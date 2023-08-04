// if else 无处不在
pub fn ifelse_example() {
    let condition = true;
    // if else 内的返回值需要同类型
    let number = if condition { 5 } else { 6 };

    println!("number: {}", number);

    let n = 6;
    if n % 4 == 0 {
        println!("divisible by 4");
    } else if n % 3 == 0 {
        println!("divisible by 3");
    } else {
        println!("divisible by other");
    }
}

// for
pub fn for_example() {
    // 从 1 到 5
    for i in 1..=5 {
        println!("i: {}", i);
    }
    // 需要注意所有权的转移，复合类型中数组实现了copy特性，所以可以正常使用，否则都要使用 & 引用，修改用 &mut
    // for item in collection 等价 for item in IntoIterator::into_iter(collection) 所有权转移
    // for item in &collection 等价 for item in collection.iter()	不可变借用
    // for item in &mut collection 等价	for item in collection.iter_mut()	可变借用
    let a = [4, 3, 2, 1];
    for (i, v) in a.iter().enumerate() {
        println!("i:{},v:{}", i + 1, v);
    }

    // 第一种, 通过索引访问，需要进行越界检查，有性能损耗，且访问是非连续的，存在访问时数组变化的风险
    let collection = [1, 2, 3, 4, 5];
    for i in 0..collection.len() {
        let item = collection[i];
        println!("item:{}", item);
    }

    // 第二种，在编译时就分析并证明访问是安全的，且访问是连续的
    for item in collection {
        println!("item:{}", item);
    }

    // continue 跳过当次循环，开始下次循环
    for i in 1..4 {
        if i == 2 {
            continue;
        }
        println!("continue {}", i);
    }

    // break 跳出整个循环，注意：只挑出当前for循环，存在嵌套时不影响上级的循环
    for j in 1..4 {
        for i in 1..4 {
            if i == 2 {
                break;
            }
            println!("break i {}", i);
        }
        println!("break j {}", j);
    }
}

// while
pub fn while_example() {
    let mut n = 0;

    while n <= 5 {
        println!("while n:{}", n);
        n += 1;
    }
    println!("结束 while n");
}

// loop loop 是表达式，可以返回一个值， break可以带一个返回值，类似 return
pub fn loop_example() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("loop result:{}", result);
}
