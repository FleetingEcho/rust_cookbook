// mod examples;
mod std_types;
// use examples::test_print;


fn main() {
    // test_print::test();
    // examples::print_debug::test();
    // examples::print_display::test();

    // examples::primitives::test();
    // examples::structs::test();
    // examples::variable_binding::test();
    // examples::conversion::test();
    // examples::expression::test();
    // examples::flow_control::test();
    // examples::match_test::test();
    // examples::test_function::test();
    // examples::closures::test();
    // examples::test_mod::test();
    // examples::struct_visit::test();

    //rustc ./src/main.rs --extern rary=library.rlib && ./main
    //使用刚写好的lib rary.rs
    // rary::public_function();

    // 错误！`private_function` 是私有的
    //rary::private_function();

    // rary::indirect_access();

    std_types::my_box::test();
    std_types::my_vectors::test();

}
