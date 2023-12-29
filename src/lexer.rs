pub struct Lexer {}

impl Lexer {
    pub fn scan(input: &str) -> Vec<TokenType> {
        let chars: Vec<char> = input.chars().collect();

        let mut tokenvec: Vec<TokenType> = vec![];

        let mut i = 0;
        while i < input.len() {
            let c = chars[i];

            match c {
                '{' => tokenvec.push(TokenType::LeftParen),
                '}' => tokenvec.push(TokenType::RightParen),
                '(' => tokenvec.push(TokenType::LeftBrace),
                ')' => tokenvec.push(TokenType::RightBrace),
                ',' => tokenvec.push(TokenType::Comma),
                '.' => tokenvec.push(TokenType::Dot),
                '+' => tokenvec.push(TokenType::Plus),
                '-' => tokenvec.push(TokenType::Minus),
                ';' => tokenvec.push(TokenType::Semicolon),
                '*' => tokenvec.push(TokenType::Star),
                '!' => {
                    if i < (input.len() - 1) && chars[i + 1] == '=' {
                        tokenvec.push(TokenType::BangEqual);
                        i += 1;
                    } else {
                        tokenvec.push(TokenType::Bang);
                    }
                }
                '=' => {
                    if i < (input.len() - 1) && chars[i + 1] == '=' {
                        tokenvec.push(TokenType::EqualEqual);
                        i += 1;
                    } else {
                        tokenvec.push(TokenType::Equal);
                    }
                }
                '<' => {
                    if i < (input.len() - 1) && chars[i + 1] == '=' {
                        tokenvec.push(TokenType::LessEqual);
                        i += 1;
                    } else {
                        tokenvec.push(TokenType::Less);
                    }
                }
                '>' => {
                    if i < (input.len() - 1) && chars[i + 1] == '=' {
                        tokenvec.push(TokenType::GreaterEqual);
                        i += 1;
                    } else {
                        tokenvec.push(TokenType::Greater);
                    }
                }
                '/' => {
                    if i < (input.len() - 1) && chars[i + 1] == '/' {
                        return tokenvec;
                    } else {
                        tokenvec.push(TokenType::Slash);
                    }
                }
                ' ' => (),
                '"' => {
                    let mut string = String::new();

                    i += 1;
                    let mut c2;
                    loop {
                        c2 = chars[i];
                        if c2 == '"' {
                            break;
                        }

                        string.push(c2);

                        i += 1;
                        if i >= input.len() {
                            println!("String not closed: line: {}", input);
                            return vec![];
                        }
                    }
                    tokenvec.push(TokenType::String(string));
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    let mut num = String::new();

                    let mut c2;
                    let mut dotted = false;

                    loop {
                        c2 = chars[i];

                        if !(c2.is_ascii_digit() || c2 == '.' && !dotted) {
                            i -= 1;
                            break;
                        }
                        if c2 == '.' {
                            dotted = true;
                        }

                        num.push(c2);

                        i += 1;
                        if i >= input.len() {
                            break;
                        }
                    }
                    if dotted {
                        tokenvec.push(TokenType::Float(num.parse::<f32>().unwrap()));
                    } else {
                        tokenvec.push(TokenType::Intiger(num.parse::<i32>().unwrap()));
                    }
                }
                _ => {
                    let keywoards = [
                        ("and", TokenType::And),
                        ("class", TokenType::Class),
                        ("else", TokenType::Else),
                        ("false", TokenType::False),
                        ("fn", TokenType::Fun),
                        ("for", TokenType::For),
                        ("if", TokenType::If),
                        ("nil", TokenType::Nil),
                        ("or", TokenType::Or),
                        ("print", TokenType::Print),
                        ("return", TokenType::Return),
                        ("super", TokenType::Super),
                        ("this", TokenType::This),
                        ("true", TokenType::True),
                        ("let", TokenType::Var),
                        ("while", TokenType::While),
                    ];
                    let mut found_keywoard = false;
                    for keywoard in keywoards {
                        if chars.len() < keywoard.0.len() + i {
                            continue;
                        }

                        if chars[i..(i + keywoard.0.len())].iter().collect::<String>() == keywoard.0
                        {
                            i += keywoard.0.len() - 1;
                            tokenvec.push(keywoard.1);

                            if keywoard.1 == TokenType::Var {
                                // remove identifier
                                tokenvec.push(TokenType::Identifier)
                            }

                            found_keywoard = true;
                            break;
                        }
                    }
                    if !found_keywoard {
                        println!("Unknown char: {} \nline: {}", c, input);
                        return vec![];
                    }
                }
            }

            i += 1;
        }

        tokenvec
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String(String),
    Intiger(i32),
    Float(f32),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}
