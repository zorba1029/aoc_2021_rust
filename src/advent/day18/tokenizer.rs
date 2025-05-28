#[derive(Debug)]
pub enum Token {
    Number(i32),
    OpenBracket,
    CloseBracket,
    Comma,
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
            ']' => {
                tokens.push(Token::CloseBracket);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
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
                tokens.push(Token::Number(number));
            }
            ' ' => {
                chars.next(); // 공백 무시
            }
            _ => {
                chars.next(); // 예상치 못한 문자 무시
            }
        }
    }

    tokens
}
