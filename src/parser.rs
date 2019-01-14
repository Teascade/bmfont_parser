use std::str::FromStr;

#[derive(Clone)]
pub(crate) struct Parser {
    chars: Vec<char>,
    cursor: usize,
}

impl Parser {
    pub fn new(text: &str) -> Parser {
        Parser {
            chars: text.chars().collect(),
            cursor: 0,
        }
    }

    pub fn peek(&self, amount: usize) -> Option<char> {
        let idx = self.cursor + amount;
        if idx >= self.chars.len() {
            None
        } else {
            Some(self.chars[idx])
        }
    }

    pub fn expect<T: Into<String>>(&mut self, text: T) -> Expect {
        Expect::new().or(text, self)
    }

    pub fn expect_number<T: FromStr>(&mut self) -> Result<T, String> {
        let mut text = String::new();
        let mut got_number = Err("No number found".to_owned());
        while let Some(c) = self.peek(0) {
            text += &c.to_string();
            if c == '-' && text.len() == 1 {
                self.move_cursor(1);
                continue;
            }
            let number = text.parse::<T>();
            match number {
                Ok(num) => {
                    got_number = Ok(num);
                    self.move_cursor(1);
                }
                Err(_) => break,
            }
        }
        got_number
    }

    pub fn expect_ident(&mut self) -> Result<String, String> {
        let mut text = String::new();
        let mut got_ident = Err("No ident found".to_owned());

        let res = self.expect("\"").get();
        let within_quotations = res.is_ok();

        while let Some(c) = self.peek(0) {
            if (!within_quotations && !Parser::is_whitespace(c)) || (within_quotations && c != '\"')
            {
                text += &c.to_string();
                self.move_cursor(1);
                got_ident = Ok(text.clone());
            } else if within_quotations && c == '\"' {
                self.move_cursor(1);
                break;
            } else {
                break;
            }
        }
        got_ident
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek(0) {
            if Parser::is_whitespace(c) {
                self.move_cursor(1);
            } else {
                break;
            }
        }
    }

    pub fn is_finished(&self) -> bool {
        self.cursor == self.chars.len()
    }

    fn move_cursor(&mut self, amount: usize) {
        self.cursor += amount;
    }

    fn is_whitespace(c: char) -> bool {
        c == ' ' || c == '\t' || c == '\n' || c == '\r'
    }
}

pub(crate) struct Expect {
    previous_expects: Vec<String>,
    correct: Option<String>,
}

impl Expect {
    fn new() -> Expect {
        Expect {
            previous_expects: Vec::new(),
            correct: None,
        }
    }

    pub(crate) fn or<T: Into<String>>(mut self, expected: T, parser: &mut Parser) -> Expect {
        let expected = expected.into();
        if self.correct.is_some() {
            self
        } else {
            for (idx, character) in expected.chars().enumerate() {
                if let Some(c) = parser.peek(idx) {
                    if character != c {
                        self.previous_expects.push(expected);
                        return self;
                    }
                }
            }
            parser.move_cursor(expected.chars().count());
            self.correct = Some(expected);
            self
        }
    }

    pub(crate) fn get(&self) -> Result<String, String> {
        if let Some(correct) = &self.correct {
            Ok(correct.clone())
        } else {
            Err(format!(
                "Not-expected result, tried: {:?}",
                self.previous_expects
            ))
        }
    }
}
