extern crate three_body_lang;
extern crate rustyline;
extern crate rustyline_derive;

use three_body_lang::evaluator::builtins::new_builtins;
use three_body_lang::evaluator::env::Env;
use three_body_lang::evaluator::Evaluator;
use three_body_lang::lexer::{is_whitespace, Lexer};
use three_body_lang::parser::{ParseError, Parser};
use three_body_lang::token::Token;
use std::borrow::Cow::{self, Borrowed, Owned};
use std::cell::RefCell;
use std::rc::Rc;
use std::io::prelude::*;
use std::fs::File;
use std::env;

use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{self, Validator};
use rustyline::KeyEvent;
use rustyline::{Cmd, CompletionType, Config, Context, EditMode, Editor};
use rustyline_derive::Helper;

#[derive(Helper)]
struct MonkeyHelper {
    env: Rc<RefCell<Env>>,
    highlighter: MatchingBracketHighlighter,
    hinter: HistoryHinter,
    colored_prompt: String,
}

impl Completer for MonkeyHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let (start, word) = extract_word(line, pos);
        let mut matches: Vec<Pair> = Vec::new();
        for key in self.env.borrow().store.keys() {
            if key.starts_with(word) {
                matches.push(Pair {
                    display: key.to_string(),
                    replacement: key.to_string(),
                });
            }
        }

        Ok((start, matches))
    }
}

impl Hinter for MonkeyHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for MonkeyHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl Validator for MonkeyHelper {
    fn validate(
        &self,
        ctx: &mut validate::ValidationContext,
    ) -> rustyline::Result<validate::ValidationResult> {
        let mut parser = Parser::new(Lexer::new(ctx.input()));
        let _ = parser.parse();
        let errors = parser.get_errors();

        Ok(match errors.len() {
            0 => validate::ValidationResult::Valid(None),
            _ => match &errors[0] {
                ParseError::UnexpectedToken {
                    want: _,
                    got: Token::Eof,
                } => validate::ValidationResult::Incomplete,
                x => validate::ValidationResult::Invalid(Some(format!("{}", x))),
            },
        })
    }

    fn validate_while_typing(&self) -> bool {
        false
    }
}

// ---- Completer ----

/// Given a `line` and a cursor `pos`ition,
/// try to find backward the start of a word.
/// Return (0, `line[..pos]`) if no break char has been found.
/// Return the word and its start position (idx, `line[idx..pos]`) otherwise.
pub fn extract_word<'l>(line: &'l str, pos: usize) -> (usize, &'l str) {
    let line = &line[..pos];
    if line.is_empty() {
        return (0, line);
    }
    let mut start = None;
    for (i, c) in line.char_indices().rev() {
        if is_whitespace(c) {
            start = Some(i + c.len_utf8());
        }
    }

    match start {
        Some(start) => (start, &line[start..]),
        None => (0, line),
    }
}

// ---- Main ----
fn main() {
    let env = Env::from(new_builtins());
    let mut evaluator = Evaluator::new(Rc::new(RefCell::new(env)));

    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build();
    let h = MonkeyHelper {
        env: evaluator.env.clone(),
        highlighter: MatchingBracketHighlighter::new(),
        hinter: HistoryHinter {},
        colored_prompt: "\x1b[32m>>\x1b[0m ".to_owned(),
    };
    let mut rl = Editor::with_config(config);
    rl.set_helper(Some(h));
    rl.bind_sequence(KeyEvent::alt('n'), Cmd::HistorySearchForward);
    rl.bind_sequence(KeyEvent::alt('p'), Cmd::HistorySearchBackward);

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let file_path = &args[1];
        let file = file_path;
        let mut file = File::open(file).expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");
        let input = contents;
        let mut parser = Parser::new(Lexer::new(&input));
        let program = parser.parse();
        evaluator.eval(&program);
        return;
    }

    println!("Feel free to type in commands\n");

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(&line);

                let mut parser = Parser::new(Lexer::new(&line));
                let program = parser.parse();
                let errors = parser.get_errors();

                if errors.len() > 0 {
                    for err in errors {
                        println!("{}", err);
                    }
                    continue;
                }

                if let Some(evaluated) = evaluator.eval(&program) {
                    println!("{}\n", evaluated);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("\n文明的种子仍在，它将重新启动，再次开始在三体世界中命运莫测的进化，欢迎您再次登录");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
}
