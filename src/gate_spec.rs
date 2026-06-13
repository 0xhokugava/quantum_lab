use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateSpec {
    H(usize),
    X(usize),
    Y(usize),
    Z(usize),
    S(usize),
    T(usize),
    Cnot { control: usize, target: usize },
    Cz { control: usize, target: usize },
}

impl FromStr for GateSpec {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (gate_name, operands) = input
            .split_once(':')
            .ok_or_else(|| format!("Invalid gate format: {input}. Expected gate:operands"))?;

        let gate_name = gate_name.trim().to_ascii_lowercase();
        let operands = operands.trim();

        match gate_name.as_str() {
            "h" => Ok(GateSpec::H(parse_qubit(operands)?)),
            "x" => Ok(GateSpec::X(parse_qubit(operands)?)),
            "y" => Ok(GateSpec::Y(parse_qubit(operands)?)),
            "z" => Ok(GateSpec::Z(parse_qubit(operands)?)),
            "s" => Ok(GateSpec::S(parse_qubit(operands)?)),
            "t" => Ok(GateSpec::T(parse_qubit(operands)?)),
            "cnot" => {
                let (control, target) = parse_control_target(operands, &gate_name)?;
                Ok(GateSpec::Cnot { control, target })
            }
            "cz" => {
                let (control, target) = parse_control_target(operands, &gate_name)?;
                Ok(GateSpec::Cz { control, target })
            }
            _ => Err(format!("Unknown gate: {gate_name}")),
        }
    }
}

fn parse_qubit(value: &str) -> Result<usize, String> {
    value
        .trim()
        .parse::<usize>()
        .map_err(|_| format!("Invalid qubit index: {}", value.trim()))
}

fn parse_control_target(operands: &str, gate_name: &str) -> Result<(usize, usize), String> {
    let (control, target) = operands
        .split_once(',')
        .ok_or_else(|| format!("{gate_name} requires control,target"))?;

    let control = parse_qubit(control)?;
    let target = parse_qubit(target)?;

    if control == target {
        return Err(format!("{gate_name} control and target must be different"));
    }

    Ok((control, target))
}

#[test]
fn parses_gate_with_whitespace() {
    let gate: GateSpec = " cnot : 0, 1 ".parse().unwrap();

    assert_eq!(
        gate,
        GateSpec::Cnot {
            control: 0,
            target: 1,
        }
    );
}
