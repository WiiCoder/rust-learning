mod base_01;
mod base_02;
mod base_03;
mod base_04;
mod base_05;
mod base_06;
mod base_07;
mod base_08;
mod base_09;
mod base_10;

fn main() {
    // base_01: variable
    println!("========== {} ==========", "base_01: variable start");
    base_01::variables();
    base_01::variable_unused_warn();
    base_01::variable_destruction();
    base_01::variable_destruction_assign();
    base_01::variable_constant_discrepancy();
    base_01::variable_shadowing();
    println!("========== {} ==========", "base_01: variable end");
    println!();

    // base_02: base type
    println!("========== {} ==========", "base_02: base type start");
    base_02::int_type();
    base_02::int_overflow();
    base_02::float_type();
    base_02::float_non();
    base_02::number_operation();
    base_02::bit_operation();
    base_02::for_range();
    base_02::rational_complex();
    base_02::char_type();
    base_02::bool_type();
    base_02::statement_expression();
    base_02::fn_report(5);
    base_02::fn_clear(&mut "a".to_string());
    // base_02::fn_dead_end();
    // base_02::fn_forever();
    println!("========== {} ==========", "base_02: base type end");
    println!();

    // base_03: Ownership and borrowing
    println!(
        "========== {} ==========",
        "base_03: Ownership and borrowing start"
    );
    base_03::example();
    base_03::pass_return();
    base_03::quote();
    base_03::immutable_quote();
    base_03::mutable_borrow();
    base_03::repeat_mutable_borrow();
    println!(
        "========== {} ==========",
        "base_03: Ownership and borrowing end"
    );

    // base_04: quote type
    println!("========== {} ==========", "base_04: quote type start");
    base_04::string_slice();
    base_04::string_str_translate();
    base_04::string_operation();
    base_04::string_transfer();
    base_04::string_utf8_operation();

    base_04::tuple_example();
    base_04::struct_example();
    base_04::tuple_struct();

    base_04::enum_example();

    base_04::array_example();
    println!("========== {} ==========", "base_04: quote type end");

    // base_05: flow control
    println!("========== {} ==========", "base_05: flow control");
    base_05::ifelse_example();
    base_05::for_example();
    base_05::while_example();
    base_05::loop_example();
    println!("========== {} ==========", "base_05: flow control");

    // base_06: pattern matching
    println!("========== {} ==========", "base_06: pattern matching");
    base_06::match_example();
    base_06::if_let_example();
    base_06::matches_macro_example();
    base_06::masking_example();

    base_06::option_example();
    println!("========== {} ==========", "base_06: pattern matching");

    // base_07: method
    println!("========== {} ==========", "base_07: method");
    base_07::method_example();
    println!("========== {} ==========", "base_07: method");

    // base_08: 泛型和特征 generics and trait
    println!("========== {} ==========", "base_08: generics and trait");
    base_08::generics_example();

    base_08::trait_example();

    base_08::add_example();
    base_08::display_example();
    println!("========== {} ==========", "base_08: generics and trait");

    // base_09: 集合类型 collection
    println!("========== {} ==========", "base_09: collection");
    base_09::vector_example();
    base_09::hash_map_example();
    println!("========== {} ==========", "base_09: collection");

    // base_10: 生命周期 life cycle
    println!("========== {} ==========", "base_10: life cycle");
    base_10::life_cycle_example();
    println!("========== {} ==========", "base_10: life cycle");

    // 生命周期 'static 意味着能和程序活得一样久，例如字符串字面量和特征对象
    // 实在遇到解决不了的生命周期标注问题，可以尝试 T: 'static，有时候它会给你奇迹
    let _: &'static str = "我没啥优点，就是活得久，嘿嘿";
}
