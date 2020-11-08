use aoc_2015::io::read_input;
use std::collections::HashMap;
use std::str::FromStr;
use Operation::*;

#[derive(Debug, PartialEq)]
struct ParseError;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Port {
    Const(u16),
    Wire(Label),
}

impl Port {
    fn from_label(label: &str) -> Self {
        Self::Wire(label.into())
    }
}

impl FromStr for Port {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u16>()
            .map(Port::Const)
            .or(Ok(Port::Wire(s.to_string())))
    }
}

#[derive(Debug, PartialEq)]
enum Operation {
    Assign(Port),
    Not(Port),
    And(Port, Port),
    Or(Port, Port),
    LShift(Port, Port),
    RShift(Port, Port),
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split(" ").collect();

        match parts.as_slice() {
            [port] => port.parse().map(Operation::Assign),
            ["NOT", port] => port.parse().map(Operation::Not),
            [a, op, b] => {
                let a: Port = a.parse()?;
                let b: Port = b.parse()?;

                match *op {
                    "AND" => Ok(Operation::And(a, b)),
                    "OR" => Ok(Operation::Or(a, b)),
                    "LSHIFT" => Ok(Operation::LShift(a, b)),
                    "RSHIFT" => Ok(Operation::RShift(a, b)),
                    _ => Err(ParseError),
                }
            }
            _ => Err(ParseError),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Gate {
    operation: Operation,
    out: Label,
}

impl FromStr for Gate {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.rsplitn(2, " -> ").collect();

        let op_str = parts.get(1).ok_or(ParseError)?;
        let out = parts.get(0).map(|p| p.to_string()).ok_or(ParseError)?;
        let operation: Operation = op_str.parse()?;

        Ok(Gate { operation, out })
    }
}

type Label = String;

type Circuit = HashMap<Label, Operation>;

fn get_circuit(data: &Vec<String>) -> Circuit {
    data.into_iter()
        .map(|s| s.parse())
        .filter_map(Result::ok)
        .map(|r: Gate| (r.out, r.operation))
        .collect()
}

fn evaluate(circuit: &Circuit, port: &Port, cache: &mut HashMap<String, u16>) -> u16 {
    use Port::*;

    if let Port::Wire(label) = port {
        if cache.contains_key(label) {
            return *cache.get(label).unwrap();
        }
    }

    let result = match port {
        Const(n) => *n,
        Wire(label) => match circuit.get(label).unwrap() {
            Assign(port) => evaluate(circuit, port, cache),
            Not(port) => !evaluate(circuit, port, cache),
            And(port1, port2) => evaluate(circuit, port1, cache) & evaluate(circuit, port2, cache),
            Or(port1, port2) => evaluate(circuit, port1, cache) | evaluate(circuit, port2, cache),
            LShift(port1, port2) => {
                evaluate(circuit, port1, cache) << evaluate(circuit, port2, cache)
            }
            RShift(port1, port2) => {
                evaluate(circuit, port1, cache) >> evaluate(circuit, port2, cache)
            }
        },
    };

    if let Port::Wire(ref label) = port {
        cache.insert(label.clone(), result);
    }

    result
}

fn solve_a(data: &Vec<String>) -> u16 {
    let circuit = get_circuit(data);
    let mut cache = HashMap::new();
    evaluate(&circuit, &Port::from_label("a"), &mut cache)
}

fn solve_b(data: &Vec<String>) -> u16 {
    let circuit = get_circuit(data);
    let mut cache = HashMap::new();
    let new_b = evaluate(&circuit, &Port::from_label("a"), &mut cache);

    let mut cache = HashMap::new();
    cache.insert("b".to_string(), new_b);
    evaluate(&circuit, &Port::from_label("a"), &mut cache)
}

fn main() {
    let data = read_input();

    let solution_a = solve_a(&data);
    let solution_b = solve_b(&data);

    println!("{}", solution_a);
    println!("{}", solution_b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            "lx -> a".parse(),
            Ok(Gate {
                operation: Operation::Assign(Port::Wire(String::from("lx"))),
                out: String::from("a")
            })
        );

        assert_eq!(
            "NOT pr -> q".parse(),
            Ok(Gate {
                operation: Operation::Not(Port::Wire(String::from("pr"))),
                out: String::from("q")
            })
        );

        assert_eq!(
            "b RSHIFT 1 -> v".parse(),
            Ok(Gate {
                operation: Operation::RShift(Port::Wire(String::from("b")), Port::Const(1)),
                out: String::from("v")
            })
        );
    }
}
