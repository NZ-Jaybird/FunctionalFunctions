use nom::types::CompleteStr;
use nom::digit;

use crate::assembler::Token;
use crate::assembler::register_parsers::register;
use crate::assembler::label_parsers::label_usage;

// Parser for integer numbers, which we preface with `#` in our assembly language:
// #100
named!(pub integer_operand<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            reg_num: digit >>
            (
                Token::Number{value: reg_num.parse::<i32>().unwrap()}
            )
        )
    )
);

named!(pub operand<CompleteStr, Token>,
    alt!(
        integer_operand |
        register |
        label_usage |
        irstring
    )
);

named!(irstring<CompleteStr, Token>,
    do_parse!(
        tag!("'") >>
        content: take_until!("'") >>
        tag!("'") >>
        (
            Token::IrString{ literal: content.to_string() }
        )
    )
);

#[test]
fn test_parse_integer_operand() {
    println!("testing integer");

    // Test a valid integer operand
    let result = integer_operand(CompleteStr("#10"));
    assert_eq!(result.is_ok(), true);
    let (rest, value) = result.unwrap();
    assert_eq!(rest, CompleteStr(""));
    assert_eq!(value, Token::Number{value: 10});

    // Test an invalid one (missing the #)
    let result = integer_operand(CompleteStr("10"));
    assert_eq!(result.is_ok(), false);
}

#[test]
fn test_parse_string_operand() {
    let result = irstring(CompleteStr("'This is a test'"));
    assert_eq!(result.is_ok(), true);
}
