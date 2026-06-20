use tiny_with_lib::compile;

#[test]
fn compile_end_to_end() {
    assert_eq!(compile("(add 2 (subtract 4 2))"), "add(2, subtract(4, 2));");
}
