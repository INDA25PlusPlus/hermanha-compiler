use crate::ast::{Program, Statement, Expression, Type, CompOp, ArithmeticOp, Condition, Block};

fn emit_type_rs(var_type: &Type) -> String {
    match var_type {
        Type::Number => "usize".to_string(),
        Type::String => "String".to_string(),
    }
}

fn emit_comp_op_rs(comp_op: &CompOp) -> String {
    match comp_op {
        CompOp::Equal => "==".to_string(),
        CompOp::LessThan => "<".to_string(),
        CompOp::GreaterThan => ">".to_string(),
    }
}

fn emit_arithmetic_op_rs(arithmetic_op: &ArithmeticOp) -> String {
    match arithmetic_op {
        ArithmeticOp::Add => "+".to_string(),
    }
}

fn emit_expression_rs(expression: &Expression) -> String {
    match expression {
        Expression::Number(n) => format!("{}", n),
        Expression::Identifier(id) => format!("{}", id),
        Expression::StringLiteral(s) => format!("{}", s),
        Expression::ArithmeticOp { left, op, right } => format!("({} {} {})", emit_expression_rs(left), emit_arithmetic_op_rs(op), emit_expression_rs(right)),
    }
}

fn emit_condition_rs(condition: &Condition) -> String {
    format!("({} {} {})", emit_expression_rs(&condition.left), emit_comp_op_rs(&condition.op), emit_expression_rs(&condition.right))
}

fn emit_block_rs(block: &Block) -> String {
    let mut out = String::new();
    for stmt in &block.statements {
        out.push_str(&emit_statement_rs(stmt));
    }
    out
}

fn emit_var_decl_rs(name: &str, var_type: &Type, value: &Expression) -> String {
    match (var_type, value) {
        (Type::Number, Expression::Number(n)) => format!("let mut {}: {} = {};
", name, emit_type_rs(var_type), n),
        (Type::String, Expression::StringLiteral(s)) => format!("let mut {}: {} = {};
", name, emit_type_rs(var_type), s),
        _ => panic!(),
    }
}

fn emit_assignment_rs(name: &str, value: &Expression) -> String {
    format!("{} = {};
", name, emit_expression_rs(value))
}

fn emit_if_rs(condition: &Condition, then_block: &Block, else_block: &Option<Block>) -> String {
    let mut out = String::new();
    out.push_str(&format!("if {} {{\n", emit_condition_rs(condition)));
    out.push_str(&emit_block_rs(then_block));
    match else_block {
        Some(block) => {
            out.push_str("} else {\n");
            out.push_str(&emit_block_rs(block));
            out.push_str("}\n");
        }
        None => {
            out.push_str("}\n");
        }
    }
    out
}

fn emit_while_rs(condition: &Condition, body: &Block) -> String {
    let mut out = String::new();
    out.push_str(&format!("while {} {{\n", emit_condition_rs(condition)));
    out.push_str(&emit_block_rs(body));
    out.push_str("}\n");
    out
}

fn emit_print_rs(value: &Expression) -> String {
    format!("println!(\"{{}}\", {});
", emit_expression_rs(value))
}

fn emit_statement_rs(stmt: &Statement) -> String {
    match stmt {
        Statement::VarDeclaration { name, var_type, value } => emit_var_decl_rs(name, var_type, value),
        Statement::Assignment { name, value } => emit_assignment_rs(name, value),
        Statement::If { condition, then_block, else_block } => emit_if_rs(condition, then_block, else_block),
        Statement::While { condition, body } => emit_while_rs(condition, body),
        Statement::Print { value } => emit_print_rs(value),
    }
}

pub fn generate_rust(program: &Program) -> String {
    let mut out = String::new();
    out.push_str("fn main() {\n");
    for stmt in &program.statements {
        out.push_str(&emit_statement_rs(stmt));
    }
    out.push_str("}\n");
    out
}