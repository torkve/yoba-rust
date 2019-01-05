use pest::Parser;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;
use rug::Integer as Bigint;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct YobaParser;

#[derive(Eq, PartialEq, Debug)]
pub enum Expression<'a> {
    Literal(&'a str),
    Integer(Bigint),
    Add(Box<Expression<'a>>, Box<Expression<'a>>),
    Sub(Box<Expression<'a>>, Box<Expression<'a>>),
}

#[derive(Eq, PartialEq, Debug)]
pub enum Statement<'a> {
    Noop,
    ModAssign(&'a str, Expression<'a>),
    Assign(&'a str, Expression<'a>),
    Define(&'a str),
    Call(&'a str),
    Print(&'a str),
    Stats,
    Condition(&'a str, Bigint, Box<Statement<'a>>, Box<Statement<'a>>),
    Function(&'a str, Vec<Statement<'a>>),
}

fn parse_int_var_pair<'a>(mut seq: Pairs<'a, Rule>) -> (&'a str, Bigint, Pairs<'a, Rule>) {
    let first = seq.next().unwrap();
    match first.as_rule() {
        Rule::ident => (
            first.as_str(),
            Bigint::from_str_radix(seq.next().unwrap().as_str(), 10).unwrap(),
            seq,
        ),
        Rule::int_number => (
            seq.next().unwrap().as_str(),
            Bigint::from_str_radix(first.as_str(), 10).unwrap(),
            seq,
        ),
        _ => unreachable!()
    }
}

fn parse_arithmetics<'a>(mut seq: Pairs<'a, Rule>) -> Expression<'a> {
    let item = seq.next().unwrap();
    let left = match item.as_rule() {
        Rule::ident => Expression::Literal(item.as_str()),
        Rule::int_number => Expression::Integer(Bigint::from_str_radix(item.as_str(), 10).unwrap()),
        _ => unreachable!()
    };
    match seq.next() {
        None => left,
        Some(x) => {
            match x.as_rule() {
                Rule::and => {
                    let right = parse_arithmetics(seq);
                    Expression::Add(Box::new(left), Box::new(right))
                },
                Rule::without => {
                    let right = parse_arithmetics(seq);
                    Expression::Sub(Box::new(left), Box::new(right))
                },
                _ => unreachable!()
            }
        }
    }
}

pub fn parse_program<'a>(data: &'a str) -> Result<Vec<Statement<'a>>, Error<Rule>> {
    let program = YobaParser::parse(Rule::program, data)?.next().unwrap();

    fn parse_value(pair: Pair<Rule>) -> Statement {
        match pair.as_rule() {
            Rule::noop => Statement::Noop,
            Rule::stats => Statement::Stats,
            Rule::rulez => Statement::Define(pair.into_inner().next().unwrap().as_str()),
            Rule::call => Statement::Call(pair.into_inner().next().unwrap().as_str()),
            Rule::print => Statement::Print(pair.into_inner().next().unwrap().as_str()),
            Rule::assign => {
                let mut seq = pair.into_inner();
                let ident = seq.next().unwrap().as_str();
                let calculation = parse_arithmetics(seq);
                Statement::Assign(ident, calculation)
            },
            Rule::memorize => {
                let mut seq = pair.into_inner();
                let ident = seq.next().unwrap().as_str();
                let funcs: Vec<_> = seq.map(parse_value).collect();
                Statement::Function(ident, funcs)
            }
            Rule::if_expr => {
                let (ident, val, mut parts) = parse_int_var_pair(pair.into_inner());
                let branch_if = parse_value(parts.next().unwrap());
                let branch_else = parse_value(parts.next().unwrap());
                Statement::Condition(
                    ident, 
                    val, 
                    Box::new(branch_if),
                    Box::new(branch_else),
                )
            },
            Rule::give => {
                let (ident, num, _) = parse_int_var_pair(pair.into_inner());
                Statement::ModAssign(ident, Expression::Integer(num))
            },
            Rule::take => {
                let (ident, num, _) = parse_int_var_pair(pair.into_inner());
                Statement::ModAssign(ident, Expression::Integer(-num))
            },
            _ => unreachable!("{:?}", pair)
        }
    }

    Ok(
        program.into_inner().map(parse_value).collect()
    )
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use pest::{parses_to, consumes_to};

    #[test]
    fn test_simple_ast() {
        parses_to!{
            parser: YobaParser,
            input: "чо есть 12 кококо тада есть семки 8 тада иди нахуй или покажь семки или усеки сиськи это чо иди нахуй йоба йоба",
            rule: Rule::program,
            tokens: [
                program(0, 197, [
                        if_expr(5, 189, [
                                int_number(14, 16),
                                ident(17, 29),
                                if_expr(39, 118, [
                                        ident(48, 58),
                                        int_number(59, 60),
                                        noop(70, 87),
                                        print(95, 118, [ident(108, 118)])
                                ]),
                                memorize(126, 189, [
                                         ident(137, 149),
                                         noop(162, 179)
                                ])
                        ])
                ])
            ]
        };
    }

    #[test]
    fn test_simple_program_parser() {
        assert_eq!(
            parse_program("чо есть 12 кококо тада есть семки 8 тада иди нахуй или покажь семки или усеки сиськи это чо иди нахуй йоба йоба"),
            Ok(
                vec![
                    Statement::Condition(
                        "кококо",
                        12.into(),
                        Box::new(
                            Statement::Condition(
                                "семки",
                                8.into(),
                                Box::new(Statement::Noop),
                                Box::new(Statement::Print("семки"))
                            )
                        ),
                        Box::new(
                            Statement::Function(
                                "сиськи",
                                vec![Statement::Noop]
                            )
                        )
                    )
                ]
            )
        );
    }

    pub const FIBONACCI: &'static str = "чо люблю сэмки йоба
чо люблю пиво йоба
чо люблю яга йоба
чо люблю итерации йоба
чо пиво это 1 йоба
чо яга это 2 йоба

чо усеки результат это
чо покажь итерации йоба
чо покажь сэмки йоба
йоба

чо усеки фибоначчи это
чо сэмки это пиво и яга йоба
чо пиво это яга йоба
чо яга это сэмки йоба
чо итерации это итерации и 1 йоба
чо есть итерации 50 тада хуйни результат или хуйни фибоначчи йоба
йоба

чо хуйни фибоначчи йоба";

    #[test]
    fn test_fibonacci_parser() {
        assert_eq!(
            parse_program(FIBONACCI),
            Ok(
                vec![
                    Statement::Define("сэмки"),
                    Statement::Define("пиво"),
                    Statement::Define("яга"),
                    Statement::Define("итерации"),
                    Statement::Assign("пиво", Expression::Integer(1.into())),
                    Statement::Assign("яга", Expression::Integer(2.into())),

                    Statement::Function(
                        "результат",
                        vec![
                            Statement::Print("итерации"),
                            Statement::Print("сэмки"),
                        ]
                    ),

                    Statement::Function(
                        "фибоначчи",
                        vec![
                            Statement::Assign(
                                "сэмки",
                                Expression::Add(
                                    Box::new(Expression::Literal("пиво")),
                                    Box::new(Expression::Literal("яга")),
                                )
                            ),
                            Statement::Assign("пиво", Expression::Literal("яга")),
                            Statement::Assign("яга", Expression::Literal("сэмки")),
                            Statement::Assign(
                                "итерации",
                                Expression::Add(
                                    Box::new(Expression::Literal("итерации")),
                                    Box::new(Expression::Integer(1.into())),
                                )
                            ),
                            Statement::Condition(
                                "итерации",
                                50.into(),
                                Box::new(Statement::Call("результат")),
                                Box::new(Statement::Call("фибоначчи"))
                            )
                        ]
                    ),

                    Statement::Call("фибоначчи"),
                ]
            )
        );
    }
}
