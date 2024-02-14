use pest::pratt_parser::PrattParser;
use pest::iterators::Pairs;
use crate::program_parser::Rule;
use crate::boat_program::{BoatExpr, BoatOp};

lazy_static::lazy_static! {
    static ref BOAT_EXPR_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(land, Left) | Op::infix(lor, Left))
            .op(Op::infix(gt, Left) | Op::infix(lt, Left) | Op::infix(eq, Left))
            .op(Op::infix(concat, Left))
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
            .op(Op::prefix(unary_minus))
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
                Rule::gt => BoatOp::Gt,
                Rule::lt => BoatOp::Lt,
                Rule::eq => BoatOp::Eq,
                Rule::land => BoatOp::Mul,
                Rule::lor => BoatOp::Add,
                _ => unreachable!(),
            };
            BoatExpr::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, exp| {
            BoatExpr::BinOp { lhs: Box::new(BoatExpr::Value("0".to_owned())), op: BoatOp::Sub, rhs: Box::new(exp) }
        })
        .parse(pairs)
}
