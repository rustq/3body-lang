use std::cell::RefCell;
use std::rc::Rc;

pub mod builtins;
pub mod env;
pub mod object;
use crate::ast;

#[derive(PartialEq, Clone, Debug)]
pub struct Evaluator {
    pub env: Rc<RefCell<env::Env>>,
}

///
// Evaluator Basic Implement
///
impl Evaluator {
    pub fn new(env: Rc<RefCell<env::Env>>) -> Self {
        Evaluator { env }
    }

    pub fn eval(&mut self, program: &ast::Program) -> Option<object::Object> {
        let mut result = None;

        for stmt in program {
            if *stmt == ast::Stmt::Blank {
                continue;
            }
            match self.eval_stmt(stmt) {
                Some(object::Object::ReturnValue(value)) => return Some(*value),
                Some(object::Object::Error(msg)) => return Some(object::Object::Error(msg)),
                obj => result = obj,
            }
        }
        result
    }

    fn eval_stmt(&mut self, stmt: &ast::Stmt) -> Option<object::Object> {
        match stmt {
            ast::Stmt::Let(ident, expr) => {
                let value = match self.eval_expr(expr) {
                    Some(value) => value,
                    None => return None,
                };
                if Self::is_error(&value) {
                    Some(value)
                } else {
                    let ast::Ident(name) = ident;
                    let mut env_borrow_mut = self.env.borrow_mut();
                    match env_borrow_mut.check_inner(name.clone()) {
                        env::CheckInnerInfo::ConstantExist => Some(object::Object::Error(format!(
                            "{} {}!",
                            "Can not redeclare constant variable", name
                        ))),
                        env::CheckInnerInfo::VariableExist => Some(object::Object::Error(format!(
                            "{} {}!",
                            "Can not redeclare variable", name
                        ))),
                        env::CheckInnerInfo::NoIdentifier => {
                            env_borrow_mut.set(name.clone(), value);
                            None
                        },
                    }
                }
            }
            ast::Stmt::Const(ident, expr) => {
                let value = match self.eval_expr(expr) {
                    Some(value) => value,
                    None => return None,
                };
                if Self::is_error(&value) {
                    Some(value)
                } else {
                    let ast::Ident(name) = ident;
                    let mut env_borrow_mut = self.env.borrow_mut();
                    match env_borrow_mut.check_inner(name.clone()) {
                        env::CheckInnerInfo::ConstantExist => Some(object::Object::Error(format!(
                            "{} {}!",
                            "Can not redeclare constant variable", name
                        ))),
                        env::CheckInnerInfo::VariableExist => Some(object::Object::Error(format!(
                            "{} {}!",
                            "Can not redeclare variable", name
                        ))),
                        env::CheckInnerInfo::NoIdentifier => {
                            env_borrow_mut.set(name.clone(), value);
                            env_borrow_mut.constant(name.clone());
                            None
                        },
                    }
                }
            }
            ast::Stmt::Break => Some(object::Object::BreakStatement),
            ast::Stmt::Continue => Some(object::Object::ContinueStatement),
            ast::Stmt::Return(expr) => {
                let value = match self.eval_expr(expr) {
                    Some(value) => value,
                    None => return None,
                };
                Some(object::Object::ReturnValue(Box::new(value)))
            }
            ast::Stmt::Expr(expr) => self.eval_expr(expr),
            ast::Stmt::ReAssign(ident, expr) => {
                let value = match self.eval_expr(expr) {
                    Some(value) => value,
                    None => return None,
                };
                if Self::is_error(&value) {
                    Some(value)
                } else {
                    let ast::Ident(name) = ident;
                    match self.env.borrow_mut().update(name.clone(), value) {
                        env::UpdateInfo::ConstantForbidden => Some(object::Object::Error(format!(
                            "{} {}!",
                            "Can not assign to constant variable", name
                        ))),
                        env::UpdateInfo::NoIdentifier => Some(object::Object::Error(format!(
                            "{} {}!",
                            "No identifier", name
                        ))),
                        env::UpdateInfo::Succeed => None,
                    }
                }
            }
            _ => todo!(),
        }
    }
}

///
// truthy + Error Eval Implement
///
impl Evaluator {
    fn error(msg: String) -> object::Object {
        object::Object::Error(msg)
    }

    fn is_truthy(obj: object::Object) -> bool {
        match obj {
            object::Object::Null | object::Object::Bool(false) => false,
            _ => true,
        }
    }

    fn is_error(obj: &object::Object) -> bool {
        match obj {
            object::Object::Error(_) => true,
            _ => false,
        }
    }
}

///
// Condition Eval Implement
///
impl Evaluator {
    // if
    fn eval_if_expr(
        &mut self,
        cond: &ast::Expr,
        consequence: &ast::BlockStmt,
        alternative: &Option<ast::BlockStmt>,
    ) -> Option<object::Object> {
        let cond = match self.eval_expr(cond) {
            Some(cond) => cond,
            None => return None,
        };

        if Self::is_truthy(cond) {
            self.eval_block_stmt(consequence)
        } else if let Some(alt) = alternative {
            self.eval_block_stmt(alt)
        } else {
            None
        }
    }

    // while
    fn eval_while_expr(
        &mut self,
        cond: &ast::Expr,
        consequence: &ast::BlockStmt,
    ) -> Option<object::Object> {
        let mut result: Option<object::Object> = None;
        loop {
            let cond_result = match self.eval_expr(cond) {
                Some(cond) => cond,
                None => break,
            };
            if !Self::is_truthy(cond_result.clone()) {
                break;
            }
            match self.eval_block_stmt_with_continue_and_break_statement(consequence) {
                Some(object::Object::BreakStatement) => {
                    result = Some(object::Object::Null);
                    break;
                }
                Some(object::Object::ContinueStatement) => {
                    result = Some(object::Object::Null);
                    continue;
                }
                Some(object::Object::ReturnValue(value)) => {
                    return Some(object::Object::ReturnValue(value))
                }
                _ => {}
            }
        }
        result
    }

    // continue + break
    fn eval_block_stmt_with_continue_and_break_statement(
        &mut self,
        stmts: &ast::BlockStmt,
    ) -> Option<object::Object> {
        let mut result = None;

        for stmt in stmts {
            if *stmt == ast::Stmt::Blank {
                continue;
            }

            match self.eval_stmt(stmt) {
                Some(object::Object::ReturnValue(value)) => {
                    return Some(object::Object::ReturnValue(value))
                }
                Some(object::Object::BreakStatement) => {
                    return Some(object::Object::BreakStatement)
                }
                Some(object::Object::ContinueStatement) => {
                    return Some(object::Object::ContinueStatement)
                }
                Some(object::Object::Error(msg)) => return Some(object::Object::Error(msg)),
                obj => result = obj,
                _ => todo!(),
            }
        }

        result
    }

    // block
    fn eval_block_stmt(&mut self, stmts: &ast::BlockStmt) -> Option<object::Object> {
        let mut result = None;

        for stmt in stmts {
            if *stmt == ast::Stmt::Blank {
                continue;
            }

            match self.eval_stmt(stmt) {
                Some(object::Object::ReturnValue(value)) => {
                    return Some(object::Object::ReturnValue(value))
                }
                Some(object::Object::Error(msg)) => return Some(object::Object::Error(msg)),
                obj => result = obj,
                _ => todo!(),
            }
        }

        result
    }
}

/// Expr Eval Implement
impl Evaluator {
    fn eval_expr(&mut self, expr: &ast::Expr) -> Option<object::Object> {
        match expr {
            ast::Expr::Ident(ident) => Some(self.eval_ident(ident)),
            ast::Expr::Literal(literal) => Some(self.eval_literal(literal)),
            ast::Expr::Prefix(prefix, right_expr) => self
                .eval_expr(&*right_expr)
                .map(|right| self.eval_prefix_expr(prefix, right)),
            ast::Expr::Infix(infix, left_expr, right_expr) => {
                let left = self.eval_expr(&*left_expr);
                let right = self.eval_expr(&*right_expr);
                if left.is_some() && right.is_some() {
                    Some(self.eval_infix_expr(infix, left.unwrap(), right.unwrap()))
                } else {
                    None
                }
            }
            ast::Expr::Index(left_expr, index_expr) => {
                let left = self.eval_expr(&*left_expr);
                let index = self.eval_expr(&*index_expr);
                if left.is_some() && index.is_some() {
                    Some(self.eval_index_expr(left.unwrap(), index.unwrap()))
                } else {
                    None
                }
            }
            ast::Expr::While { cond, consequence } => self.eval_while_expr(&*cond, consequence),
            ast::Expr::If {
                cond,
                consequence,
                alternative,
            } => self.eval_if_expr(&*cond, consequence, alternative),
            ast::Expr::Function { params, body } => Some(object::Object::Function(
                params.clone(),
                body.clone(),
                Rc::clone(&self.env),
            )),
            ast::Expr::Call { func, args } => Some(self.eval_call_expr(func, args)),
            _ => None,
        }
    }

    fn eval_prefix_expr(&mut self, prefix: &ast::Prefix, right: object::Object) -> object::Object {
        match prefix {
            ast::Prefix::Not => self.eval_not_op_expr(right),
            ast::Prefix::Minus => self.eval_minus_prefix_op_expr(right),
            ast::Prefix::Plus => self.eval_plus_prefix_op_expr(right),
        }
    }

    fn eval_not_op_expr(&mut self, right: object::Object) -> object::Object {
        match right {
            object::Object::Bool(true) => object::Object::Bool(false),
            object::Object::Bool(false) => object::Object::Bool(true),
            object::Object::Null => object::Object::Bool(true),
            _ => object::Object::Bool(false),
        }
    }

    fn eval_minus_prefix_op_expr(&mut self, right: object::Object) -> object::Object {
        match right {
            object::Object::Int(value) => object::Object::Int(-value),
            _ => Self::error(format!("unknown operator: -{}", right)),
        }
    }

    fn eval_plus_prefix_op_expr(&mut self, right: object::Object) -> object::Object {
        match right {
            object::Object::Int(value) => object::Object::Int(value),
            _ => Self::error(format!("unknown operator: {}", right)),
        }
    }

    fn eval_infix_expr(
        &mut self,
        infix: &ast::Infix,
        left: object::Object,
        right: object::Object,
    ) -> object::Object {
        match left {
            object::Object::Int(left_value) => {
                if let object::Object::Int(right_value) = right {
                    self.eval_infix_int_expr(infix, left_value, right_value)
                } else {
                    Self::error(format!("type mismatch: {} {} {}", left, infix, right))
                }
            }
            object::Object::String(left_value) => {
                if let object::Object::String(right_value) = right {
                    self.eval_infix_string_expr(infix, left_value, right_value)
                } else {
                    Self::error(format!("type mismatch: {} {} {}", left_value, infix, right))
                }
            }
            _ => Self::error(format!("unknown operator: {} {} {}", left, infix, right)),
        }
    }

    fn eval_infix_int_expr(&mut self, infix: &ast::Infix, left: i64, right: i64) -> object::Object {
        match infix {
            ast::Infix::Plus => object::Object::Int(left + right),
            ast::Infix::Minus => object::Object::Int(left - right),
            ast::Infix::Multiply => object::Object::Int(left * right),
            ast::Infix::Divide => object::Object::Int(left / right),
            ast::Infix::LT => object::Object::Bool(left < right),
            ast::Infix::LTEQ => object::Object::Bool(left <= right),
            ast::Infix::GT => object::Object::Bool(left > right),
            ast::Infix::GTEQ => object::Object::Bool(left >= right),
            ast::Infix::Equal => object::Object::Bool(left == right),
            ast::Infix::NotEqual => object::Object::Bool(left != right),
            _ => todo!(),
        }
    }

    fn eval_infix_string_expr(
        &mut self,
        infix: &ast::Infix,
        left: String,
        right: String,
    ) -> object::Object {
        match infix {
            ast::Infix::Plus => object::Object::String(format!("{}{}", left, right)),
            _ => object::Object::Error(format!("unknown operator: {} {} {}", left, infix, right)),
        }
    }
}

///
// Function Eval Implement
// (put args ident list into scoped env and the eval block stmts with the scoped env)
///
impl Evaluator {
    fn eval_call_expr(&mut self, func: &Box<ast::Expr>, args: &Vec<ast::Expr>) -> object::Object {
        let args = args
            .iter()
            .map(|e| self.eval_expr(e).unwrap_or(object::Object::Null))
            .collect::<Vec<_>>();

        let (params, body, env) = match self.eval_expr(&*func) {
            Some(object::Object::Function(params, body, env)) => (params, body, env),
            Some(object::Object::Builtin(expect_param_num, f)) => {
                if expect_param_num < 0 || expect_param_num == args.len() as i32 {
                    return f(args);
                } else {
                    return Self::error(format!(
                        "wrong number of arguments. got={}, want={}",
                        args.len(),
                        expect_param_num,
                    ));
                }
            }
            Some(o) => return Self::error(format!("{} is not valid function", o)),
            None => return object::Object::Null,
        };

        if params.len() != args.len() {
            return Self::error(format!(
                "wrong number of arguments: {} expected but {} given",
                params.len(),
                args.len()
            ));
        }

        let current_env = Rc::clone(&self.env);
        let mut scoped_env = env::Env::new_with_outer(Rc::clone(&env));
        let list = params.iter().zip(args.iter());
        for (_, (ident, o)) in list.enumerate() {
            let ast::Ident(name) = ident.clone();
            scoped_env.set(name, o.clone());
        }

        self.env = Rc::new(RefCell::new(scoped_env));

        let object = self.eval_block_stmt(&body);

        self.env = current_env;

        match object {
            Some(object::Object::ReturnValue(o)) => *o,
            Some(o) => o,
            None => object::Object::Null,
        }
    }
}

///
// ...
///
impl Evaluator {
    fn eval_ident(&mut self, ident: &ast::Ident) -> object::Object {
        let ast::Ident(name) = ident;

        match self.env.borrow_mut().get(name.clone()) {
            Some(value) => value,
            None => object::Object::Error(format!("identifier not found: {}", name)),
        }
    }

    fn eval_literal(&mut self, literal: &ast::Literal) -> object::Object {
        match literal {
            ast::Literal::Int(value) => object::Object::Int(*value),
            ast::Literal::String(value) => object::Object::String(value.clone()),
            ast::Literal::Bool(value) => object::Object::Bool(*value),
            ast::Literal::Array(objects) => self.eval_array_literal(objects),
            ast::Literal::Hash(pairs) => self.eval_hash_literal(pairs),
            _ => panic!(),
        }
    }
}

///
// array, hash, index ...
///
impl Evaluator {
    fn eval_array_literal(&mut self, objects: &Vec<ast::Expr>) -> object::Object {
        object::Object::Array(
            objects
                .iter()
                .map(|e| self.eval_expr(&e.clone()).unwrap_or(object::Object::Null))
                .collect::<Vec<_>>(),
        )
    }

    fn eval_index_expr(&mut self, left: object::Object, index: object::Object) -> object::Object {
        match left {
            object::Object::Array(ref array) => {
                if let object::Object::Int(i) = index {
                    self.eval_array_index_expr(array.clone(), i)
                } else {
                    Self::error(format!("index operator not supported: {}", left))
                }
            }
            object::Object::Hash(ref hash) => match index {
                object::Object::Int(_) | object::Object::Bool(_) | object::Object::String(_) => {
                    match hash.get(&index) {
                        Some(o) => o.clone(),
                        None => object::Object::Null,
                    }
                }
                object::Object::Error(_) => index,
                _ => Self::error(format!("unusable as hash key: {}", index)),
            },
            _ => Self::error(format!("uknown operator: {} {}", left, index)),
        }
    }

    fn eval_array_index_expr(&mut self, array: Vec<object::Object>, index: i64) -> object::Object {
        let max = array.len() as i64;

        if index < 0 || index > max {
            return object::Object::Null;
        }

        match array.get(index as usize) {
            Some(o) => o.clone(),
            None => object::Object::Null,
        }
    }

    fn eval_hash_literal(&mut self, pairs: &Vec<(ast::Expr, ast::Expr)>) -> object::Object {
        let mut hash = std::collections::HashMap::new();

        for (key_expr, value_expr) in pairs {
            let key = self.eval_expr(key_expr).unwrap_or(object::Object::Null);
            if Self::is_error(&key) {
                return key;
            }

            let value = self.eval_expr(value_expr).unwrap_or(object::Object::Null);
            if Self::is_error(&value) {
                return value;
            }

            hash.insert(key, value);
        }

        object::Object::Hash(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::env;
    use super::object;
    use super::Evaluator;
    use crate::ast;
    use crate::evaluator::builtins::new_builtins;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn eval(input: &str) -> Option<object::Object> {
        Evaluator {
            env: Rc::new(RefCell::new(env::Env::from(new_builtins()))),
        }
        .eval(&Parser::new(Lexer::new(input)).parse())
    }

    /// cases in edition 2015

    #[test]
    fn test_int_expr() {
        let tests = vec![
            ("5", Some(object::Object::Int(5))),
            ("10", Some(object::Object::Int(10))),
            ("-5", Some(object::Object::Int(-5))),
            ("-10", Some(object::Object::Int(-10))),
            ("+5", Some(object::Object::Int(5))),
            ("+10", Some(object::Object::Int(10))),
            ("+(-5)", Some(object::Object::Int(-5))),
            ("+(-10)", Some(object::Object::Int(-10))),
            ("5 + 5 + 5 + 5 - 10", Some(object::Object::Int(10))),
            ("2 * 2 * 2 * 2 * 2", Some(object::Object::Int(32))),
            ("-50 + 100 + -50", Some(object::Object::Int(0))),
            ("5 * 2 + 10", Some(object::Object::Int(20))),
            ("5 + 2 * 10", Some(object::Object::Int(25))),
            ("20 + 2 * -10", Some(object::Object::Int(0))),
            ("50 / 2 * 2 + 10", Some(object::Object::Int(60))),
            ("2 * (5 + 10)", Some(object::Object::Int(30))),
            ("3 * 3 * 3 + 10", Some(object::Object::Int(37))),
            ("3 * (3 * 3) + 10", Some(object::Object::Int(37))),
            (
                "(5 + 10 * 2 + 15 / 3) * 2 + -10",
                Some(object::Object::Int(50)),
            ),
        ];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

    #[test]
    fn test_string_expr() {
        let input = "\"Hello World!\"";

        assert_eq!(
            Some(object::Object::String(String::from("Hello World!"))),
            eval(input)
        );
    }

    #[test]
    fn test_string_concatenation() {
        let input = "\"Hello\" + \" \" + \"World!\"";

        assert_eq!(
            Some(object::Object::String(String::from("Hello World!"))),
            eval(input)
        );
    }

    #[test]
    fn test_boolean_expr() {
        let tests = vec![
            ("true", Some(object::Object::Bool(true))),
            ("false", Some(object::Object::Bool(false))),
            ("1 < 2", Some(object::Object::Bool(true))),
            ("1 > 2", Some(object::Object::Bool(false))),
            ("1 < 1", Some(object::Object::Bool(false))),
            ("1 > 1", Some(object::Object::Bool(false))),
            ("1 >= 1", Some(object::Object::Bool(true))),
            ("1 <= 1", Some(object::Object::Bool(true))),
            ("1 >= 2", Some(object::Object::Bool(false))),
            ("1 <= 1", Some(object::Object::Bool(true))),
            ("2 <= 1", Some(object::Object::Bool(false))),
            ("1 == 1", Some(object::Object::Bool(true))),
            ("1 != 1", Some(object::Object::Bool(false))),
            ("1 == 2", Some(object::Object::Bool(false))),
            ("1 != 2", Some(object::Object::Bool(true))),
        ];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

    #[test]
    fn test_array_literal() {
        let input = "[1, 2 * 2, 3 + 3]";

        assert_eq!(
            Some(object::Object::Array(vec![
                object::Object::Int(1),
                object::Object::Int(4),
                object::Object::Int(6),
            ])),
            eval(input),
        );
    }

    #[test]
    fn test_array_index_expr() {
        let tests = vec![
            ("[1, 2, 3][0]", Some(object::Object::Int(1))),
            ("[1, 2, 3][1]", Some(object::Object::Int(2))),
            ("let i = 0; [1][i]", Some(object::Object::Int(1))),
            ("[1, 2, 3][1 + 1];", Some(object::Object::Int(3))),
            (
                "let myArray = [1, 2, 3]; myArray[2];",
                Some(object::Object::Int(3)),
            ),
            (
                "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];",
                Some(object::Object::Int(6)),
            ),
            (
                "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i];",
                Some(object::Object::Int(2)),
            ),
            ("[1, 2, 3][3]", Some(object::Object::Null)),
            ("[1, 2, 3][-1]", Some(object::Object::Null)),
        ];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

    #[test]
    fn test_hash_literal() {
        let input = r#"
let two = "two";
{
  "one": 10 - 9,
  two: 1 + 1,
  "thr" + "ee": 6 / 2,
  4: 4,
  true: 5,
  false: 6
}
"#;

        let mut hash = std::collections::HashMap::new();
        hash.insert(
            object::Object::String(String::from("one")),
            object::Object::Int(1),
        );
        hash.insert(
            object::Object::String(String::from("two")),
            object::Object::Int(2),
        );
        hash.insert(
            object::Object::String(String::from("three")),
            object::Object::Int(3),
        );
        hash.insert(object::Object::Int(4), object::Object::Int(4));
        hash.insert(object::Object::Bool(true), object::Object::Int(5));
        hash.insert(object::Object::Bool(false), object::Object::Int(6));

        assert_eq!(Some(object::Object::Hash(hash)), eval(input),);
    }

    #[test]
    fn test_hash_index_expr() {
        let tests = vec![
            ("{\"foo\": 5}[\"foo\"]", Some(object::Object::Int(5))),
            ("{\"foo\": 5}[\"bar\"]", Some(object::Object::Null)),
            (
                "let key = \"foo\"; {\"foo\": 5}[key]",
                Some(object::Object::Int(5)),
            ),
            ("{}[\"foo\"]", Some(object::Object::Null)),
            ("{5: 5}[5]", Some(object::Object::Int(5))),
            ("{true: 5}[true]", Some(object::Object::Int(5))),
            ("{false: 5}[false]", Some(object::Object::Int(5))),
        ];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

    #[test]
    fn test_not_operator() {
        let tests = vec![
            ("!true", Some(object::Object::Bool(false))),
            ("!false", Some(object::Object::Bool(true))),
            ("!!true", Some(object::Object::Bool(true))),
            ("!!false", Some(object::Object::Bool(false))),
            ("!!5", Some(object::Object::Bool(true))),
        ];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

    #[test]
    fn test_if_else_expr() {
        let tests = vec![
            ("if (true) { 10 }", Some(object::Object::Int(10))),
            ("if (false) { 10 }", None),
            ("if (1) { 10 }", Some(object::Object::Int(10))),
            ("if (1 < 2) { 10 }", Some(object::Object::Int(10))),
            ("if (1 > 2) { 10 }", None),
            (
                "if (1 > 2) { 10 } else { 20 }",
                Some(object::Object::Int(20)),
            ),
            (
                "if (1 < 2) { 10 } else { 20 }",
                Some(object::Object::Int(10)),
            ),
            ("if (1 <= 2) { 10 }", Some(object::Object::Int(10))),
            ("if (1 >= 2) { 10 }", None),
            (
                "if (1 >= 2) { 10 } else { 20 }",
                Some(object::Object::Int(20)),
            ),
            (
                "if (1 <= 2) { 10 } else { 20 }",
                Some(object::Object::Int(10)),
            ),
        ];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

    #[test]
    fn test_return_stmt() {
        let tests = vec![
            ("return 10;", Some(object::Object::Int(10))),
            ("return 10; 9;", Some(object::Object::Int(10))),
            ("return 2 * 5; 8;", Some(object::Object::Int(10))),
            ("9; return 2 * 5; 9;", Some(object::Object::Int(10))),
            (
                r#"
if (10 > 1) {
  if (10 > 1) {
    return 10;
  }
  return 1;
}"#,
                Some(object::Object::Int(10)),
            ),
        ];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

    #[test]
    fn test_let_stmt() {
        let tests = vec![
            ("let a = 5; a;", Some(object::Object::Int(5))),
            ("let a = 5 * 5; a;", Some(object::Object::Int(25))),
            ("let a = 5; let b = a; b;", Some(object::Object::Int(5))),
            (
                "let a = 5; let b = a; let c = a + b + 5; c;",
                Some(object::Object::Int(15)),
            ),
            ("let a = 5; let a = 1;", Some(object::Object::Error(
                "Can not redeclare variable a!".to_owned(),
            ))),
        ];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

    #[test]
    fn test_const_stmt() {
        let tests = vec![(
            "const a = 5; a = 3;",
            Some(object::Object::Error(
                "Can not assign to constant variable a!".to_owned(),
            )),
        ), (
            "const a = 5; let a = 3;",
            Some(object::Object::Error(
                "Can not redeclare constant variable a!".to_owned(),
            )),
        ), (
            "const a = 5; const a = 3;",
            Some(object::Object::Error(
                "Can not redeclare constant variable a!".to_owned(),
            )),
        )];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

    #[test]
    fn test_assign_stmt() {
        let tests = vec![("let a = 5; a = 3; a", Some(object::Object::Int(3)))];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

    #[test]
    fn test_blank_stmt() {
        let tests = vec![
            (
                r#"5;


"#,
                Some(object::Object::Int(5)),
            ),
            (
                r#"let identity = fn (x) {
  x;

}

identity(100);

"#,
                Some(object::Object::Int(100)),
            ),
        ];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

    #[test]
    fn test_fn_object() {
        let input = "fn(x) { x + 2; };";

        assert_eq!(
            Some(object::Object::Function(
                vec![ast::Ident(String::from("x"))],
                vec![ast::Stmt::Expr(ast::Expr::Infix(
                    ast::Infix::Plus,
                    Box::new(ast::Expr::Ident(ast::Ident(String::from("x")))),
                    Box::new(ast::Expr::Literal(ast::Literal::Int(2))),
                ))],
                Rc::new(RefCell::new(env::Env::from(new_builtins()))),
            )),
            eval(input),
        );
    }

    #[test]
    fn test_fn_application() {
        let tests = vec![
            (
                "let identity = fn(x) { x; }; identity(5);",
                Some(object::Object::Int(5)),
            ),
            (
                "let identity = fn(x) { return x; }; identity(5);",
                Some(object::Object::Int(5)),
            ),
            (
                "let double = fn(x) { x * 2; }; double(5);",
                Some(object::Object::Int(10)),
            ),
            (
                "let add = fn(x, y) { x + y; }; add(5, 5);",
                Some(object::Object::Int(10)),
            ),
            (
                "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));",
                Some(object::Object::Int(20)),
            ),
            ("fn(x) { x; }(5)", Some(object::Object::Int(5))),
            (
                "fn(a) { let f = fn(b) { a + b }; f(a); }(5);",
                Some(object::Object::Int(10)),
            ),
        ];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

    #[test]
    fn test_closures() {
        let input = r#"
let newAdder = fn(x) {
  fn(y) { x + y };
}

let addTwo = newAdder(2);
addTwo(2);
        "#;

        assert_eq!(Some(object::Object::Int(4)), eval(input));
    }

    #[test]
    fn test_builtin_functions() {
        let tests = vec![
            // len
            ("len(\"\")", Some(object::Object::Int(0))),
            ("len(\"four\")", Some(object::Object::Int(4))),
            ("len(\"hello world\")", Some(object::Object::Int(11))),
            ("len([1, 2, 3])", Some(object::Object::Int(3))),
            (
                "len(1)",
                Some(object::Object::Error(String::from(
                    "argument to `len` not supported, got 1",
                ))),
            ),
            (
                "len(\"one\", \"two\")",
                Some(object::Object::Error(String::from(
                    "wrong number of arguments. got=2, want=1",
                ))),
            ),
            // first
            ("first([1, 2, 3])", Some(object::Object::Int(1))),
            ("first([])", Some(object::Object::Null)),
            (
                "first([], [])",
                Some(object::Object::Error(String::from(
                    "wrong number of arguments. got=2, want=1",
                ))),
            ),
            (
                "first(\"string\")",
                Some(object::Object::Error(String::from(
                    "argument to `first` must be array. got \"string\"",
                ))),
            ),
            (
                "first(1)",
                Some(object::Object::Error(String::from(
                    "argument to `first` must be array. got 1",
                ))),
            ),
            // last
            ("last([1, 2, 3])", Some(object::Object::Int(3))),
            ("last([])", Some(object::Object::Null)),
            (
                "last([], [])",
                Some(object::Object::Error(String::from(
                    "wrong number of arguments. got=2, want=1",
                ))),
            ),
            (
                "last(\"string\")",
                Some(object::Object::Error(String::from(
                    "argument to `last` must be array. got \"string\"",
                ))),
            ),
            (
                "last(1)",
                Some(object::Object::Error(String::from(
                    "argument to `last` must be array. got 1",
                ))),
            ),
            // rest
            (
                "rest([1, 2, 3, 4])",
                Some(object::Object::Array(vec![
                    object::Object::Int(2),
                    object::Object::Int(3),
                    object::Object::Int(4),
                ])),
            ),
            (
                "rest([2, 3, 4])",
                Some(object::Object::Array(vec![
                    object::Object::Int(3),
                    object::Object::Int(4),
                ])),
            ),
            ("rest([4])", Some(object::Object::Array(vec![]))),
            ("rest([])", Some(object::Object::Null)),
            (
                "rest([], [])",
                Some(object::Object::Error(String::from(
                    "wrong number of arguments. got=2, want=1",
                ))),
            ),
            (
                "rest(\"string\")",
                Some(object::Object::Error(String::from(
                    "argument to `rest` must be array. got \"string\"",
                ))),
            ),
            (
                "rest(1)",
                Some(object::Object::Error(String::from(
                    "argument to `rest` must be array. got 1",
                ))),
            ),
            // push
            (
                "push([1, 2, 3], 4)",
                Some(object::Object::Array(vec![
                    object::Object::Int(1),
                    object::Object::Int(2),
                    object::Object::Int(3),
                    object::Object::Int(4),
                ])),
            ),
            (
                "push([], 1)",
                Some(object::Object::Array(vec![object::Object::Int(1)])),
            ),
            (
                "let a = [1]; push(a, 2); a", // 不改变原数组
                Some(object::Object::Array(vec![object::Object::Int(1)])),
            ),
            (
                "push([], [], [])",
                Some(object::Object::Error(String::from(
                    "wrong number of arguments. got=3, want=2",
                ))),
            ),
            (
                "push(\"string\", 1)",
                Some(object::Object::Error(String::from(
                    "argument to `push` must be array. got \"string\"",
                ))),
            ),
            (
                "push(1, 1)",
                Some(object::Object::Error(String::from(
                    "argument to `push` must be array. got 1",
                ))),
            ),
        ];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

    #[test]
    fn test_error_handling() {
        let tests = vec![
            (
                "5 + true",
                Some(object::Object::Error(String::from(
                    "type mismatch: 5 + true",
                ))),
            ),
            (
                "5 + true; 5;",
                Some(object::Object::Error(String::from(
                    "type mismatch: 5 + true",
                ))),
            ),
            (
                "-true",
                Some(object::Object::Error(String::from(
                    "unknown operator: -true",
                ))),
            ),
            (
                "5; true + false; 5;",
                Some(object::Object::Error(String::from(
                    "unknown operator: true + false",
                ))),
            ),
            (
                "if (10 > 1) { true + false; }",
                Some(object::Object::Error(String::from(
                    "unknown operator: true + false",
                ))),
            ),
            (
                "\"Hello\" - \"World\"",
                Some(object::Object::Error(String::from(
                    "unknown operator: Hello - World",
                ))),
            ),
            // todo cases
        ];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

    #[test]
    fn test_z_combinator() {
        let input = r#"
let z = fn(f) {
    return fn(x) {
        return f(fn(y) {
            return x(x)(y);
        });
        }(fn(x) {
        return f(fn(y) {
            return x(x)(y);
        });
    });
};

return z(fn(f) {
    return fn(n) {
    if (n == 0) {
        1
    } else {
        n * f(n - 1)
    }
    };
})(5);
        "#;

        assert_eq!(Some(object::Object::Int(120)), eval(input));
    }

    /// self cases

    #[test]
    fn test_reassign_evaluator() {
        let tests = vec![
            ("let five = 5; five = 6; five", Some(object::Object::Int(6))),
            (
                "let y = 5; let x = 3; x = y; x",
                Some(object::Object::Int(5)),
            ),
            (
                "let y = 5; let x = 3; x = x + y * y / 5 - 1; x",
                Some(object::Object::Int(7)),
            ),
        ];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

    #[test]
    fn test_while_break_continue_evaluator() {
        let tests = vec![
            ("let cond = true; let a = 5; while (cond) { a = 3; cond = false; } a;", Some(object::Object::Int(3))),
            ("let cond = true; let a = 5; while (cond) { a = a - 1; if (a == 2) { break; } } a", Some(object::Object::Int(2))),
            ("let cond = true; let a = 5; while (cond) { a = a - 1; if (a >= 2) { continue; } return a; }", Some(object::Object::Int(1))),
        ];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

    #[test]
    fn test_closure_adder() {
        let input = r#"
let adder = fn() {
  let x = 1;
  return fn() { x = x + 1; x };
}

let f = adder();
f();
f();
f();
f()
        "#;

        assert_eq!(Some(object::Object::Int(5)), eval(input));
    }

    #[test]
    fn test_comment() {
        let tests = vec![
            (
                "let identity = fn(x) { // function defination here
x; // just x
};
identity(5); // run with param 5",
                Some(object::Object::Int(5)),
            ),
        ];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }
}
