#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompOp {
    Equal,
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArithmeticOp {
    Add,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(i64),
    Identifier(String),
    StringLiteral(String),
    ArithmeticOp {
        left: Box<Expression>,
        op: ArithmeticOp,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    pub left: Expression,
    pub op: CompOp,
    pub right: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VarDeclaration {
        name: String,
        var_type: Type,
        value: Expression,
    },
    Assignment {
        name: String,
        value: Expression,
    },
    If {
        condition: Condition,
        then_block: Block,
        else_block: Option<Block>,
    },
    While {
        condition: Condition,
        body: Block,
    },
    Print {
        value: Expression,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}
