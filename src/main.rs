mod base_01;
mod base_02;
mod base_03;

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
}