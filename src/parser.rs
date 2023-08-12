use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{alpha0, multispace0},
    combinator::{map, opt},
    multi::many1,
    sequence::{delimited, terminated, tuple},
    IResult,
};

// Atoms

#[derive(Debug, PartialEq)]
pub enum Atom {
    String(String),
}

pub fn parse_string(input: &str) -> IResult<&str, Atom> {
    let parser = delimited(tag("\""), take_until("\""), tag("\""));
    map(parser, |string: &str| Atom::String(string.to_string()))(input)
}

// Expressions

#[derive(Debug, PartialEq)]
pub enum Expression {
    SpellCast(Spell, Option<Atom>),
}

#[derive(Debug, PartialEq)]
pub enum Spell {
    AvadaKedabra,
    Revelio,
    Periculum,
    Lumus,
}

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    parse_spell_cast(input)
}

pub fn parse_spell_cast(input: &str) -> IResult<&str, Expression> {
    // take until ; or ->
    let spell_parser = delimited(tag("~"), alpha0, opt(tag(" ")));
    let target_parser = parse_string;
    let parser = tuple((spell_parser, opt(target_parser)));

    map(parser, |(spell, target)| match spell {
        "AvadaKedabra" => Expression::SpellCast(Spell::AvadaKedabra, target),
        "Revelio" => Expression::SpellCast(Spell::Revelio, target),
        "Periculum" => Expression::SpellCast(Spell::Periculum, target),
        "Lumus" => Expression::SpellCast(Spell::Lumus, target),
        _ => panic!("Wand broken: Unknown spell: {}", spell),
    })(input)
}

// Statements

#[derive(Debug, PartialEq)]
pub enum Statement {
    ExpressionStatement(Expression),
}

fn parse_statement(input: &str) -> IResult<&str, Statement> {
    parse_expression_statement(input)
}

fn parse_expression_statement(input: &str) -> IResult<&str, Statement> {
    let (rest, expression) = terminated(parse_expression, multispace0)(input)?;
    let statement = Statement::ExpressionStatement(expression);
    Ok((rest, statement))
}

// Program

#[derive(Debug, PartialEq)]
pub struct Program(pub Vec<Statement>);

pub fn parse_program(input: &str) -> IResult<&str, Program> {
    map(many1(terminated(parse_statement, multispace0)), Program)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Atoms

    #[test]
    fn test_parse_string() {
        let input = "\"Hello, world!\"";
        let expected = Atom::String("Hello, world!".to_string());
        let (_, actual) = parse_string(input).unwrap();
        assert_eq!(expected, actual);
    }

    // Expressions

    #[test]
    fn test_parse_spell_cast() {
        let input = "~AvadaKedabra";
        let expected = Expression::SpellCast(Spell::AvadaKedabra, None);
        let (_, actual) = parse_spell_cast(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_spell_cast_with_string() {
        let input = "~Revelio \"Hello, world!\"";
        let expected = Expression::SpellCast(
            Spell::Revelio,
            Some(Atom::String("Hello, world!".to_string())),
        );
        let (_, actual) = parse_spell_cast(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_spell_cast_with_string_and_space() {
        let input = "~Revelio \"Hello, world!\" ";
        let expected = Expression::SpellCast(
            Spell::Revelio,
            Some(Atom::String("Hello, world!".to_string())),
        );
        let (_, actual) = parse_spell_cast(input).unwrap();
        assert_eq!(expected, actual);
    }

    // Statements

    #[test]
    fn test_parse_expression_statement() {
        let input = "~AvadaKedabra";
        let expected =
            Statement::ExpressionStatement(Expression::SpellCast(Spell::AvadaKedabra, None));
        let (_, actual) = parse_expression_statement(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_expression_statement_with_string() {
        let input = "~Revelio \"Hello, world!\"";
        let expected = Statement::ExpressionStatement(Expression::SpellCast(
            Spell::Revelio,
            Some(Atom::String("Hello, world!".to_string())),
        ));
        let (_, actual) = parse_expression_statement(input).unwrap();
        assert_eq!(expected, actual);
    }

    // Program

    #[test]
    fn test_parse_program() {
        let input = "~AvadaKedabra\n~Revelio \"Hello, world!\"";
        let expected = Program(vec![
            Statement::ExpressionStatement(Expression::SpellCast(Spell::AvadaKedabra, None)),
            Statement::ExpressionStatement(Expression::SpellCast(
                Spell::Revelio,
                Some(Atom::String("Hello, world!".to_string())),
            )),
        ]);
        let (_, actual) = parse_program(input).unwrap();
        assert_eq!(expected, actual);
    }
}
