#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

#[test]
fn test() {
    let input = "name: string";

    grammar::Field1Parser::new().parse(input).unwrap();
}
