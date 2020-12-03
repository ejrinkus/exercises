use std::collections::HashMap;

type WireType = u16;

#[derive(Debug, PartialEq)]
pub enum GateType {
  Unknown,
  And,
  Or,
  Not,
  Lshift,
  Rshift,
}

#[derive(Debug, PartialEq)]
enum Operand {
  Empty,
  Label(String),
  Value(WireType),
}

pub struct Gate {
  gate_type: GateType,
  l_operand: Operand,
  r_operand: Operand,
  out_wire: String,
}

impl Gate {
  pub fn default() -> Gate {
    Gate {
      gate_type: GateType::Unknown,
      l_operand: Operand::Empty,
      r_operand: Operand::Empty,
      out_wire: "".to_owned(),
    }
  }

  pub fn new(gt: GateType, lw: Option<String>, rw: Option<String>, ow: String) -> Gate {
    Gate {
      gate_type: gt,
      l_operand: lw.map_or(Operand::Empty, |v| Operand::Label(v)),
      r_operand: rw.map_or(Operand::Empty, |v| Operand::Label(v)),
      out_wire: ow,
    }
  }

  pub fn resolve(&self) -> Result<WireType, String> {
    return match &self.gate_type {
      GateType::And => {
        if let Operand::Value(left) = self.l_operand {
          if let Operand::Value(right) = self.r_operand {
            Ok(left & right)
          } else {
            Err("Right operand of AND gate is not resolved.".to_owned())
          }
        } else {
          Err("Left operand of AND gate is not resolved.".to_owned())
        }
      }
      GateType::Or => {
        if let Operand::Value(left) = self.l_operand {
          if let Operand::Value(right) = self.r_operand {
            Ok(left | right)
          } else {
            Err("Right operand of OR gate is not resolved.".to_owned())
          }
        } else {
          Err("Left operand of OR gate is not resolved.".to_owned())
        }
      }
      GateType::Not => {
        if let Operand::Value(right) = self.r_operand {
          Ok(!right)
        } else {
          Err("Right operand of NOT gate is not resolved.".to_owned())
        }
      }
      GateType::Lshift => {
        if let Operand::Value(left) = self.l_operand {
          if let Operand::Value(right) = self.r_operand {
            Ok(left << right)
          } else {
            Err("Right operand of LSHIFT gate is not resolved.".to_owned())
          }
        } else {
          Err("Left operand of LSHIFT gate is not resolved.".to_owned())
        }
      }
      GateType::Rshift => {
        if let Operand::Value(left) = self.l_operand {
          if let Operand::Value(right) = self.r_operand {
            Ok(left >> right)
          } else {
            Err("Right operand of RSHIFT gate is not resolved.".to_owned())
          }
        } else {
          Err("Left operand of RSHIFT gate is not resolved.".to_owned())
        }
      }
      GateType::Unknown => Err("Cannot resolve Unknown gate type".to_owned()),
    };
  }
}

pub struct Circuit {
  // Map of all resolved wires, keyed by their label.
  wires: HashMap<String, WireType>,
  // List of all unresolved gates.
  gates: Vec<Gate>,
  // List of all unresolved wire assignments.
  assignments: Vec<(String, String)>,
}

/// Implementation for the Circuit struct.
impl Circuit {
  /// Constructs a new empty circuit.
  pub fn new() -> Circuit {
    Circuit {
      wires: HashMap::new(),
      gates: Vec::new(),
      assignments: Vec::new(),
    }
  }

  /// Figure out if an instruction is for a wire or gate, then add it to the circuit.
  pub fn handle_instruction(&mut self, instruction: &str) -> Result<(), String> {
    if instruction.contains("AND")
      || instruction.contains("OR")
      || instruction.contains("NOT")
      || instruction.contains("LSHIFT")
      || instruction.contains("RSHIFT")
    {
      self.add_gate(instruction)
    } else {
      self.add_wire(instruction)
    }
  }

  /// Add a wire to the circuit.
  pub fn add_wire(&mut self, w: &str) -> Result<(), String> {
    let mut sides = w.split(" -> ");
    let val_string = sides.next().unwrap();
    let label = sides.next().unwrap();
    if self.wires.contains_key(label) {
      return Err(format!("inserted duplicate wire: {}", label));
    }
    if let Ok(val) = val_string.parse::<WireType>() {
      self.wires.insert(label.to_owned(), val);
    } else {
      if let Some(val) = self.wires.get(val_string) {
        self.wires.insert(label.to_owned(), *val);
      } else {
        self
          .assignments
          .push((val_string.to_owned(), label.to_owned()));
      }
    }
    Ok(())
  }

  /// Add a gate to the circuit.  Attempt to resolve it if possible, otherwise
  /// it will be added to a list for later resolution.
  pub fn add_gate(&mut self, g: &str) -> Result<(), String> {
    let mut sides = g.split(" -> ");
    let mut pieces = sides.next().unwrap().split(' ');
    let mut gate = Gate::default();
    gate.out_wire = sides.next().unwrap().to_owned();

    let mut found_left = false;
    while let Some(piece) = pieces.next() {
      if piece == "NOT" {
        found_left = true;
        gate.l_operand = Operand::Empty;
        gate.gate_type = GateType::Not;
      } else if !found_left {
        found_left = true;
        if let Ok(val) = piece.parse::<WireType>() {
          gate.l_operand = Operand::Value(val);
        } else if self.wires.contains_key(piece) {
          gate.l_operand = Operand::Value(*self.wires.get(piece).unwrap());
        } else {
          gate.l_operand = Operand::Label(piece.to_owned());
        }
      } else if piece == "AND" {
        gate.gate_type = GateType::And;
      } else if piece == "OR" {
        gate.gate_type = GateType::Or;
      } else if piece == "LSHIFT" {
        gate.gate_type = GateType::Lshift;
      } else if piece == "RSHIFT" {
        gate.gate_type = GateType::Rshift;
      } else {
        if let Ok(val) = piece.parse::<WireType>() {
          gate.r_operand = Operand::Value(val);
        } else if self.wires.contains_key(piece) {
          gate.r_operand = Operand::Value(*self.wires.get(piece).unwrap());
        } else {
          gate.r_operand = Operand::Label(piece.to_owned());
        }
      }
    }

    if let Ok(val) = gate.resolve() {
      self.wires.insert(gate.out_wire, val);
    } else {
      self.gates.push(gate);
    }

    Ok(())
  }

  /// Attempt to resolve all unresolved gates.
  pub fn resolve(&mut self) -> Result<(), String> {
    loop {
      let gates_before = self.gates.len();
      let assignments_before = self.assignments.len();
      self.resolve_gates();
      self.resolve_assignments();

      if self.gates.len() == 0 && self.assignments.len() == 0 {
        return Ok(());
      }
      if gates_before == self.gates.len() && assignments_before == self.assignments.len() {
        return Err("Could not resolve circuit.".to_owned());
      }
    }
  }

  fn resolve_gates(&mut self) {
    let mut to_retry: Vec<Gate> = Vec::new();
    while let Some(mut gate) = self.gates.pop() {
      if let Operand::Label(l) = &gate.l_operand {
        if let Some(val) = self.wires.get(l) {
          gate.l_operand = Operand::Value(*val);
        }
      }
      if let Operand::Label(l) = &gate.r_operand {
        if let Some(val) = self.wires.get(l) {
          gate.r_operand = Operand::Value(*val);
        }
      }
      if let Ok(val) = gate.resolve() {
        self.wires.insert(gate.out_wire, val);
      } else {
        to_retry.push(gate);
      }
    }
    self.gates = to_retry;
  }

  fn resolve_assignments(&mut self) {
    let mut to_retry: Vec<(String, String)> = Vec::new();
    while let Some((left, right)) = self.assignments.pop() {
      if let Some(val) = self.wires.get(&left) {
        self.wires.insert(right, *val);
      } else {
        to_retry.push((left, right));
      }
    }
    self.assignments = to_retry;
  }

  /// Attempt to get the value of a given wire label from the circuit.
  pub fn get_wire(&self, label: &str) -> Option<WireType> {
    if let Some(val) = self.wires.get(label) {
      Some(*val)
    } else {
      None
    }
  }
}
