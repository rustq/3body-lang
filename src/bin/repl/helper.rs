#[cfg(feature = "repl")]
extern crate rustyline;
#[cfg(feature = "repl")]
extern crate rustyline_derive;

use three_body_interpreter::lexer::Lexer;
use three_body_interpreter::parser::ParseError;
use three_body_interpreter::parser::Parser;
use three_body_interpreter::token::Token;

#[derive(rustyline_derive::Helper, rustyline_derive::Hinter, rustyline_derive::Highlighter, rustyline_derive::Completer)]
pub struct Helper {
}

impl Helper {
    pub fn new() -> Self {
        Self {}
    }
}

impl rustyline::validate::Validator for Helper {
    fn validate(
        &self,
        ctx: &mut rustyline::validate::ValidationContext,
    ) -> rustyline::Result<rustyline::validate::ValidationResult> {
        let mut parser = Parser::new(Lexer::new(ctx.input()));
        let _ = parser.parse();
        let errors = parser.get_errors();

        Ok(match errors.len() {
            0 => rustyline::validate::ValidationResult::Valid(None),
            _ => match &errors[0] {
                ParseError::UnexpectedToken {
                    want: _,
                    got: Token::Eof,
                } => rustyline::validate::ValidationResult::Incomplete,
                x => rustyline::validate::ValidationResult::Invalid(Some(format!("{:?}???", x))),
            },
        })
    }

    fn validate_while_typing(&self) -> bool {
        false
    }
}
