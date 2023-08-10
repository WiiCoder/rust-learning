// 集合类型 collection

use std::collections::HashMap;

pub fn vector_example() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);

    // 没有显示声明类型，当第一次 push 后，会自动推断类型，如果没有添加只定义了变量，会编译不通过
    let mut v = Vec::new();
    v.push(1);

    // 如果预先知道要存储室元素个数，使用如下方式，可以避免频繁的内存分配和拷贝
    let _: Vec<f64> = Vec::with_capacity(10);

    // 使用宏创建数组，它会根据初始化的值自动推导类型
    let _ = vec![1, 2, 3];

    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("第三个元素是:{}", third);

    match v.get(2) {
        Some(third) => println!("第三个元素是 {third}"),
        None => println!("根本没有"),
    }

    // 如果不可变借用 first 在可变借用 v.push 后使用，无法通过编译
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    println!("The first element is: {first}");

    v.push(6);

    for ele in v {
        println!("{ele}");
    }

    let mut c = vec![1, 2, 3, 4, 5];
    for ele in &mut c {
        *ele += 10
    }
    for ele in c {
        println!("{ele}");
    }

    // 存储不同类型的元素， 可以通过枚举和特征对象来实现不同类型元素的存储
    // 实际应用中，特征对象数组要比枚举数组常见的多，特征对象非常灵活，
    // 编译器对枚举的限制较多，且无法动态增加类型
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let v = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];
    for ip in v {
        println!("{:?}", ip);
    }

    // 通过特征对象
    trait IpAddrTr {
        fn display(&self);
    }
    struct V4(String);
    impl IpAddrTr for V4 {
        fn display(&self) {
            println!("ipv4: {:?}", self.0);
        }
    }
    struct V6(String);
    impl IpAddrTr for V6 {
        fn display(&self) {
            println!("ipv6: {:?}", self.0);
        }
    }

    let v: Vec<Box<dyn IpAddrTr>> = vec![
        Box::new(V4("127..0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];
    for ip in v {
        ip.display();
    }

    // 排序, 稳定和非稳定指的是对相等元素的处理方式
    // 非稳定排序的算法速度优于稳定算法，稳定算法排序会额外分配原数组一半的空间
    // 稳定排序 sort 和 sort_by
    // 非稳定排序 sort_unstable 和 sort_unstable_by
    let mut v = vec![1, 3, 4, 12, 10];
    v.sort_unstable();
    println!("{:?}", v);

    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];
    // 因为浮点类型并没有实现全数值比较 0rd 的特性，只实现了部分可比较 PartialOrd.
    // 所以直接排序会有问题，可以使用 partial_cmp 作为大小比较的根据
    // vec.sort_unstable();
    vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", vec);

    // 结构体排序
    #[derive(Debug)]
    #[allow(unused)]
    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        fn new(name: String, age: u32) -> Person {
            Person { name, age }
        }
    }

    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    people.sort_unstable_by(|a, b| a.age.cmp(&b.age));
    println!("people: {:?}", people);
}

pub fn hash_map_example() {
    // 所有集合类型都是动态的，意味着没有固定的内存大小，因此底层的数据都存储在内存堆上
    let mut my_gems = HashMap::new();
    my_gems.insert("k1", 1);
    my_gems.insert("k2", 2);

    // 如果预先知道 KV 对个数
    let mut my_gems = HashMap::with_capacity(10);
    my_gems.insert("k", "v");

    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];
    let mut teams_map = HashMap::new();
    for team in &teams_list {
        teams_map.insert(&team.0, &team.1);
    }

    println!("{:?}", teams_map);

    // 可以通过 into_iter 方法将列表转为迭代器，再通过 collect 方法收集
    // collect 方法内部支持生成多种类型的目标集合，所以需要 显示声明 类型
    // 如 HashMap<_, _>，即收集 HashMap 类型，KV 类型让编译器推导
    let teams_map: HashMap<_, _> = teams_list.into_iter().collect();
    println!("{:?}", teams_map);

    // 所有权转移
    // 类型实现 Copy 特征， 会被复制进 HashMap，无所谓所有权
    // 没有实现，所有权会被转移给 HashMap 中
    let name = String::from("Sunface");
    let age = 18;

    let mut handsome_boys = HashMap::new();
    handsome_boys.insert(name, age);

    // 因为 name 的所有权转移给了 handsome_boys， 所以会下面这行会有问题
    // println!("因为过于无耻，{}已经被从帅气男孩名单中除名", name);
    // 因为 i32 实现了 Copy 特征， 所以仍然可以直接使用 ages
    println!("还有，他的真实年龄远远不止{}岁", age);

    // get 返回一个 Option<&_> 类型，如果查不到，会返回一个 None， 查询到了就是 Some<&_>
    // 返回的是对 HashMap 中值的借用，如果不使用借用，可能会发生所有权的转移
    let score = teams_map.get(&("中国队".to_string()));
    println!("score: {:?}", score);
    // copied 复制一份，unwrap 返回 i32 类型，如果前面复制的是 None，则使用 0 当默认值
    let score = teams_map.get(&("中国队".to_string())).copied().unwrap_or(0);
    println!("score: {:?}", score);

    for (k, v) in &teams_map {
        println!("k:{}, v:{}", k, v);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // 判断 key 是否存在， 不存在则插入
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
