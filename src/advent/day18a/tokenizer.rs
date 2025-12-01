pub(crate) enum Token {
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
                        number = number * 10 + next_c.to_digit(10).unwrap() as i32;
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(number));
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            ']' => {
                tokens.push(Token::CloseBracket);
                chars.next();
            }
            ' ' => {
                chars.next();
            }
            _ => {
                chars.next();
            }
        }
    }

    tokens
}