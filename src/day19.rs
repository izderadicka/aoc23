use std::{
    collections::HashMap,
    io::{self, BufRead}, ops::RangeInclusive,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Property {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Relation {
    Less,
    Greater,
}

impl Relation {
    fn is_satisfied_by(&self, part: &Part, property: Property, value: i64) -> bool {
        match property {
            Property::X => self.compare(part.x, value),
            Property::M => self.compare(part.m, value),
            Property::A => self.compare(part.a, value),
            Property::S => self.compare(part.s, value),
        }
    }

    fn compare(&self, a: i64, b: i64) -> bool {
        match self {
            Relation::Less => a < b,
            Relation::Greater => a > b,
        }
    }
}

#[derive(Debug, Clone)]
struct PartLimits {
    x: RangeInclusive<i64>,
    m: RangeInclusive<i64>,
    a: RangeInclusive<i64>,
    s: RangeInclusive<i64>,
}

impl Default for PartLimits {
    fn default() -> Self {
        Self {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        }
    }
}

impl PartLimits {
    fn aplly_rule(self, rule: &Compare) -> Self {
        let modify_range =|range: RangeInclusive<i64>| -> RangeInclusive<i64> {
            match rule.relation {
                Relation::Less => *range.start()..=(*range.end()).min(rule.value),
                Relation::Greater => rule.value.max(*range.start())..=*range.end(),
            }

        };
        match rule.property {
            Property::X => Self {
                x: modify_range(self.x),
                ..self
            },
            Property::M => Self {
                m: modify_range(self.m),
                ..self
            },
            Property::A => Self {
                a: modify_range(self.a),
                ..self
            },
            Property::S => Self {
                s: modify_range(self.s),
                ..self
            },
        }
        
        
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn rating(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }

    fn is_accepted(&self, rules: &Rules) -> Result<bool, anyhow::Error> {
        let mut rule = rules
            .get("in")
            .ok_or_else(|| anyhow::anyhow!("No in rule"))?;

        loop {
            for r in rule {
                let maybe_action = match r {
                    Rule::Action(a) => Some(a),

                    Rule::Compare(c) => c.action_for(self),
                };

                if let Some(action) = maybe_action {
                    match action {
                        Action::Approve => return Ok(true),
                        Action::Reject => return Ok(false),
                        Action::Forward { target } => {
                            rule = rules
                                .get(target)
                                .ok_or_else(|| anyhow::anyhow!("No rule for {}", target))?;
                            break;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Action {
    Reject,
    Approve,
    Forward { target: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Compare {
    property: Property,
    relation: Relation,
    value: i64,
    action: Action,
}

impl Compare {
    fn action_for(&self, part: &Part) -> Option<&Action> {
        if self
            .relation
            .is_satisfied_by(part, self.property, self.value)
        {
            Some(&self.action)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Rule {
    Action(Action),
    Compare(Compare),
}

type Rules = HashMap<String, Vec<Rule>>;
type Parts = Vec<Part>;

fn parse_rules(
    lines: &mut impl Iterator<Item = Result<String, io::Error>>,
) -> Result<Rules, anyhow::Error> {
    let mut rules = HashMap::new();
    loop {
        match lines.next() {
            Some(Ok(line)) => {
                if line.is_empty() {
                    break;
                }

                let (rule_tag, rule) = parser::parse_rule(&line)?;
                rules.insert(rule_tag, rule);
            }
            None => break,
            Some(Err(e)) => return Err(e.into()),
        }
    }
    Ok(rules)
}

fn parse_parts(
    lines: &mut impl Iterator<Item = Result<String, io::Error>>,
) -> Result<Parts, anyhow::Error> {
    let mut parts = Vec::new();
    loop {
        match lines.next() {
            Some(Ok(line)) => {
                if line.is_empty() {
                    break;
                }
                let p = parser::parse_part(&line)?;
                parts.push(p);
            }
            None => break,
            Some(Err(e)) => return Err(e.into()),
        }
    }
    Ok(parts)
}

fn find_ranges(rules: &Rules, rule_tag: &str, limits: PartLimits)  {
   
    for rule in rules.get(rule_tag).unwrap() {
       match rule {
        
       }
    }
    
}

pub fn nineteenth_task_1(f: impl BufRead) -> u64 {
    let mut iter = f.lines();
    let rules = parse_rules(&mut iter).unwrap();
    println!("Rules {:?}", rules);
    let parts = parse_parts(&mut iter).unwrap();
    println!("Parts {:?}", parts);
    let sum: i64 = parts.into_iter().filter(|p| p.is_accepted(&rules).unwrap()).map(|p| p.rating()).sum();
    sum as u64
}

pub fn nineteenth_task_2(f: impl BufRead) -> u64 {
    let mut iter = f.lines();
    let rules = parse_rules(&mut iter).unwrap();
    println!("Rules {:?}", rules);
    0
}

mod parser {
    use super::*;
    use anyhow::bail;
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while},
        character::complete::{char, digit1},
        combinator::map,
        multi::separated_list1,
        sequence::{delimited, separated_pair, tuple},
        IResult,
    };

    pub fn parse_part(line: &str) -> Result<Part, anyhow::Error> {
        match part(line) {
            Ok((_, res)) => Ok(res),
            Result::Err(e) => bail!("Parsing error {}", e),
        }
    }

    fn part_property(input: &str) -> IResult<&str, (Property, i64)> {
        let (input, (p, v)) = separated_pair(property, tag("="), digit1)(input)?;
        let v: i64 = v.parse().unwrap();
        Ok((input, (p, v)))
    }

    fn part(input: &str) -> IResult<&str, Part> {
        let (input, properties) =
            delimited(tag("{"), separated_list1(tag(","), part_property), tag("}"))(input)?;
        let mut part = Part::default();
        for (property, value) in properties {
            match property {
                Property::X => part.x = value,
                Property::M => part.m = value,
                Property::A => part.a = value,
                Property::S => part.s = value,
            }
        }
        Ok((input, part))
    }

    pub fn parse_rule(line: &str) -> Result<(String, Vec<Rule>), anyhow::Error> {
        match full_rule(line) {
            Ok((_, res)) => Ok(res),
            Result::Err(e) => bail!("Parsing error {}", e),
        }
    }

    fn parse_label(input: &str) -> IResult<&str, String> {
        let (input, tag) = take_while(|c: char| c.is_ascii_lowercase())(input)?;
        Ok((input, tag.to_string()))
    }

    fn approve(input: &str) -> IResult<&str, Action> {
        let (input, _) = tag("A")(input)?;
        Ok((input, Action::Approve))
    }

    fn reject(input: &str) -> IResult<&str, Action> {
        let (input, _) = tag("R")(input)?;
        Ok((input, Action::Reject))
    }

    fn action(input: &str) -> IResult<&str, Action> {
        alt((
            approve,
            reject,
            map(parse_label, |label| Action::Forward { target: label }),
        ))(input)
    }

    fn property(input: &str) -> IResult<&str, Property> {
        map(
            alt((char('x'), char('m'), char('a'), char('s'))),
            |c| match c {
                'x' => Property::X,
                'm' => Property::M,
                'a' => Property::A,
                's' => Property::S,
                _ => unreachable!(),
            },
        )(input)
    }

    fn compare(input: &str) -> IResult<&str, Compare> {
        let (input, res) = tuple((
            property,
            alt((char('<'), char('>'))),
            digit1,
            tag(":"),
            action,
        ))(input)?;
        let (property, relation, value, _, action) = res;
        let value = value.parse().unwrap();
        let relation = match relation {
            '<' => Relation::Less,
            '>' => Relation::Greater,
            _ => unreachable!(),
        };
        Ok((
            input,
            Compare {
                property,
                relation,
                value,
                action,
            },
        ))
    }

    fn rule(input: &str) -> IResult<&str, Rule> {
        alt((
            map(compare, |c| Rule::Compare(c)),
            map(action, |a| Rule::Action(a)),
        ))(input)
    }

    fn full_rule(input: &str) -> IResult<&str, (String, Vec<Rule>)> {
        let (input, label) = parse_label(input)?;
        let (input, rules) = delimited(tag("{"), separated_list1(tag(","), rule), tag("}"))(input)?;
        Ok((input, (label, rules)))
    }

    #[test]
    fn test_parse_rule() {
        let sample = "px{a<2006:qkq,m>2090:A,rfg}".to_string();
        let (expected_tag, expected_rule) = parse_rule(&sample).unwrap();
        assert_eq!("px", expected_tag);
        assert_eq!(3, expected_rule.len());
    }

    // fn test_list() {
    //     let sample = "px{a<2006:qkq,m>2090:A,rfg}".to_string();
    //     let (expected_tag, expected_rule) = parse_rule(&sample).unwrap();
    //     assert_eq!("px", expected_tag);
    //     assert_eq!(3, expected_rule.len());
    // }

    #[test]
    fn test_action() {
        assert_eq!(action("R"), Ok(("", Action::Reject)));
        assert_eq!(action("A"), Ok(("", Action::Approve)));
        assert_eq!(
            action("qkq"),
            Ok((
                "",
                Action::Forward {
                    target: "qkq".to_string()
                }
            ))
        );
    }

    #[test]
    fn test_compare() {
        assert_eq!(
            compare("x<2006:A"),
            Ok((
                "",
                Compare {
                    property: Property::X,
                    relation: Relation::Less,
                    value: 2006,
                    action: Action::Approve
                }
            ))
        );
        assert_eq!(
            compare("a<2006:qkq"),
            Ok((
                "",
                Compare {
                    property: Property::A,
                    relation: Relation::Less,
                    value: 2006,
                    action: Action::Forward {
                        target: "qkq".to_string()
                    }
                }
            ))
        );
    }

    #[test]
    fn test_rule() {
        let sample = "a<2006:qkq";
        let (_, r) = rule(sample).unwrap();

        assert_eq!(
            r,
            Rule::Compare(Compare {
                property: Property::A,
                relation: Relation::Less,
                value: 2006,
                action: Action::Forward {
                    target: "qkq".to_string()
                }
            })
        );

        let sample = "A";
        let (_, r) = rule(sample).unwrap();
        assert_eq!(r, Rule::Action(Action::Approve))
    }

    #[test]
    fn test_part() {
        let sample = "{x=787,m=2655,a=1222,s=2876}";
        let (_, p) = part(sample).unwrap();
        assert_eq!(
            p,
            Part {
                x: 787,
                m: 2655,
                a: 1222,
                s: 2876
            }
        );
    }
}
