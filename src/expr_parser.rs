use pest::{Parser};
use pest::pratt_parser::{PrattParser};
use pest::iterators::Pairs;


#[derive(Debug)]
pub enum BoatExpr {
    Value(String),
    Var(String),
    Function {
        name: String,
        args: Vec<BoatExpr>,
    },
    BinOp {
        lhs: Box<BoatExpr>,
        op: BoatOp,
        rhs: Box<BoatExpr>,
    },
}

#[derive(Debug)]
pub enum BoatOp {
    Add,
    Sub,
    Mul,
    Div,
    Conc,
}

#[derive(pest_derive::Parser)]
#[grammar = "expr.pest"]
#[grammar = "base.pest"]
pub struct BoatExprParser;

lazy_static::lazy_static! {
    static ref BOAT_EXPR_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
            .op(Op::infix(concat, Left))
    };
}

pub fn parse_pairs(pairs: Pairs<Rule>) -> BoatExpr {
    BOAT_EXPR_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::string => BoatExpr::Value(primary.into_inner().next().unwrap().as_str().to_owned()),
            Rule::integer => BoatExpr::Value(primary.as_str().to_owned()),
            Rule::expr => parse_pairs(primary.into_inner()),
            Rule::function => {
                let mut inner = primary.into_inner();
                let name = inner.next().unwrap().as_str().to_owned();
                let args = inner.map(|pair| parse_pairs(pair.into_inner())).collect();
                BoatExpr::Function { name, args }
            }
            Rule::name => BoatExpr::Var(primary.as_str().to_owned()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => BoatOp::Add,
                Rule::subtract => BoatOp::Sub,
                Rule::multiply => BoatOp::Mul,
                Rule::divide => BoatOp::Div,
                Rule::concat => BoatOp::Conc,
                _ => unreachable!(),
            };
            BoatExpr::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .parse(pairs)
}

pub fn parse_string(s: &str) -> BoatExpr {
    parse_pairs(BoatExprParser::parse(Rule::equation, s).unwrap().next().unwrap().into_inner())
}
