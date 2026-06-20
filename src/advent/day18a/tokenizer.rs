#[derive(Debug)]
pub(crate) enum Token {
    #[allow(dead_code)] // tree_handler 모듈에서 사용됨 (현재 주석 처리됨)
    Number(i32),
    OpenBracket,
    Comma,
    CloseBracket,
}

pub(crate) fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '[' => {
                tokens.push(Token::OpenBracket);
                chars.next();
            }
            '0'..='9' => {
                let mut number = 0;
                while let Some(&next_c) = chars.peek() {
                    if next_c.is_digit(10) {
                        chars.next();
                        number = number * 10 + next_c.to_digit(10).unwrap() as i32;
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(number as i32));
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            ']' => {
                tokens.push(Token::CloseBracket);
                chars.next();
            }
            _ => {
                // Skip unrecognized characters (like commas or whitespace)
                chars.next();
            }
        }
    }

    tokens
}
