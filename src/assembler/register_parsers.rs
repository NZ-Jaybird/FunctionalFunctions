use nom::types::CompleteStr;
use nom::digit;

use crate::assembler::Token;

named!(pub register <CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("$") >>
            reg_num: digit >>
            (
                Token::Register{
                  reg_num: reg_num.parse::<u8>().unwrap()
                }
            )
        )
    )
);

#[test]
fn test_parse_register() {
    println!("testing register");

    let result = register(CompleteStr("$0"));
    assert_eq!(result.is_ok(), true);
    let result = register(CompleteStr("0"));
    assert_eq!(result.is_ok(), false);
    let result = register(CompleteStr("$a"));
    assert_eq!(result.is_ok(), false);
}
