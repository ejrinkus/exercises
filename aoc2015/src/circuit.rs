use std::collections::HashMap;

pub struct Circuit {
  wires: HashMap<String, u16>,
}

/// Implementation for the Circuit struct.  Gates (and input signals) are
/// represented as methods.
impl Circuit {
  /// Constructs a new circuit with no wires.
  pub fn new() -> Circuit {
    Circuit {
      wires: HashMap::new(),
    }
  }

  /// Parses an instruction string and runs the instruction on this circuit.
  ///
  /// Every instruction should be placed on its own line.  Each instruction can
  /// be one of the following:
  ///
  /// * (value) -> (wire) - Store (value) inside (wire).
  /// * NOT (wire) -> (owire) - Store the bitwise compliment of the value in
  /// (wire) in (owire).
  /// * (lwire) AND (rwire) -> (owire) - Perform a bitwise AND on lwire and
  /// rwire, and store the result in owire.
  /// * (lwire) OR (rwire) -> (owire) - Perform a bitwise OR on lwire and
  /// rwire, and store the result in owire.
  /// * (wire) LSHIFT (dist) -> (owire) - Left shift the signal in (wire) by
  /// (dist) places, and store the result in owire.
  /// * (wire) RSHIFT (dist) -> (owire) - Right shift the signal in (wire) by
  /// (dist) places, and store the result in owire.
  ///
  /// # Arguments
  ///
  /// * `instruction` - A string slice containing the instruction to run.
  ///
  /// # Returns
  ///
  /// An empty Result if the instruction was successful, or an Err otherwise.
  ///
  /// # Examples
  ///
  /// ```
  /// use aoc2015::circuit::Circuit;
  ///
  /// let mut c = Circuit::new();
  /// c.handle_instruction("123 -> x").unwrap();
  /// c.handle_instruction("456 -> y").unwrap();
  /// c.handle_instruction("x AND y -> d").unwrap();
  /// c.handle_instruction("x OR y -> e").unwrap();
  /// c.handle_instruction("x LSHIFT 2 -> f").unwrap();
  /// c.handle_instruction("y RSHIFT 2 -> g").unwrap();
  /// c.handle_instruction("NOT x -> h").unwrap();
  /// c.handle_instruction("NOT y -> i").unwrap();
  /// assert_eq!(c.get_wire("d"), Some(&72));
  /// assert_eq!(c.get_wire("e"), Some(&507));
  /// assert_eq!(c.get_wire("f"), Some(&492));
  /// assert_eq!(c.get_wire("g"), Some(&114));
  /// assert_eq!(c.get_wire("h"), Some(&65412));
  /// assert_eq!(c.get_wire("i"), Some(&65079));
  /// assert_eq!(c.get_wire("x"), Some(&123));
  /// assert_eq!(c.get_wire("y"), Some(&456));
  pub fn handle_instruction(&mut self, instruction: &str) -> Result<(), &'static str> {
    let parts: Vec<&str> = instruction.split(" -> ").collect();
    if parts.len() != 2 {
      return Err("Instructions must contain exactly one '->' operator");
    }
    let expression = parts[0];
    let owire = parts[1];

    let parts: Vec<&str> = expression.split(' ').collect();

    // This must be an input signal if there's only one piece after splitting
    // the expression.
    if parts.len() == 1 {
      let signal = match parts[0].parse::<u16>() {
        Ok(s) => s,
        Err(_) => return Err("Input signal must be a valid 16-bit unsigned integer"),
      };
      self.input_signal(owire, signal);
      return Ok(());
    }

    // This must be a NOT gate if there's only two pieces.
    if parts.len() == 2 {
      if parts[0] != "NOT" {
        return Err("Invalid instruction (did you mean NOT?)");
      }
      self.not(parts[1], owire).unwrap();
      return Ok(());
    }

    // This could either be an AND, OR, LSHIFT, or RSHIFT gate.
    if parts.len() == 3 {
      match parts[1] {
        "AND" => self.and(parts[0], parts[2], owire).unwrap(),
        "OR" => self.or(parts[0], parts[2], owire).unwrap(),
        "LSHIFT" => self
          .lshift(parts[0], parts[2].parse::<usize>().unwrap(), owire)
          .unwrap(),
        "RSHIFT" => self
          .rshift(parts[0], parts[2].parse::<usize>().unwrap(), owire)
          .unwrap(),
        _ => return Err("Invalid instruction (did you mean AND, OR, LSHIFT or RSHIFT?)"),
      };
      return Ok(());
    }

    Err("Invalid instruction")
  }

  /// Retrieve the value stored in the given wire.
  ///
  /// # Arguments
  ///
  /// * `wire` - A string slice key representing the wire to be retrieved.
  ///
  /// # Returns
  ///
  /// A reference to the stored 16-bit unsigned value in the wire, or None if
  /// the wire doesn't exist.
  ///
  /// # Examples
  ///
  /// ```
  /// use aoc2015::circuit::Circuit;
  ///
  /// let mut c = Circuit::new();
  /// c.input_signal("a", 1);
  /// assert_eq!(c.get_wire("a"), Some(&1));
  /// c.input_signal("b", 2);
  /// assert_eq!(c.get_wire("b"), Some(&2));
  /// c.input_signal("a", 3);
  /// assert_eq!(c.get_wire("a"), Some(&3));
  /// assert_eq!(c.get_wire("c"), None);
  /// ```
  pub fn get_wire(&self, wire: &str) -> Option<&u16> {
    self.wires.get(wire)
  }

  /// Sends the given signal to the given wire.  The wire will be added to the
  /// circuit if it does not yet exist.
  ///
  /// # Arguments
  ///
  /// * `wire` - A string slice key representing the wire to send the signal to.
  /// * `signal` - The 16-bit unsigned signal to send to the wire.
  ///
  /// # Returns
  ///
  /// If the wire already exists, then an Option with the old value is returned.
  /// Otherwise, None is returned.
  ///
  /// # Examples
  ///
  /// ```
  /// use aoc2015::circuit::Circuit;
  ///
  /// let mut c = Circuit::new();
  /// assert_eq!(c.input_signal("a", 1), None);
  /// assert_eq!(c.input_signal("b", 2), None);
  /// assert_eq!(c.input_signal("a", 3), Some(1));
  /// ```
  pub fn input_signal(&mut self, wire: &str, signal: u16) -> Option<u16> {
    self.wires.insert(String::from(wire), signal)
  }

  /// Perform a bitwise AND on the signals stored in two wires, and send the
  /// result to a third wire.  The third wire will be added to the circuit if it
  /// does not yet exist.
  ///
  /// # Arguments
  ///
  /// * `lwire` - A string slice key representing the wire on the left side of
  ///             the AND gate.
  /// * `rwire` - A string slice key representing the wire on the right side of
  ///             the AND gate.
  ///
  /// # Returns
  ///
  /// A Result containing the value produced by the gate, or an Err if either
  /// input wire didn't exist.
  ///
  /// # Examples
  ///
  /// ```
  /// use aoc2015::circuit::Circuit;
  ///
  /// let mut c = Circuit::new();
  /// c.input_signal("a", 0b101);
  /// c.input_signal("b", 0b110);
  /// assert_eq!(c.and("a", "b", "c"), Ok(0b100));
  /// assert_eq!(c.get_wire("c"), Some(&0b100));
  /// assert!(c.and("foo", "bar", "c").is_err());
  /// ```
  pub fn and(&mut self, lwire: &str, rwire: &str, owire: &str) -> Result<u16, &'static str> {
    let lsig = match self.wires.get(lwire) {
      Some(s) => s,
      None => return Err("left wire doesn't exist"),
    };
    let rsig = match self.wires.get(rwire) {
      Some(s) => s,
      None => return Err("right wire doesn't exist"),
    };
    let signal = lsig & rsig;
    self.wires.insert(String::from(owire), signal);
    Ok(signal)
  }

  /// Perform a bitwise OR on the signals stored in two wires, and send the
  /// result to a third wire.  The third wire will be added to the circuit if it
  /// does not yet exist.
  ///
  /// # Arguments
  ///
  /// * `lwire` - A string slice key representing the wire on the left side of
  ///             the OR gate.
  /// * `rwire` - A string slice key representing the wire on the right side of
  ///             the OR gate.
  ///
  /// # Returns
  ///
  /// A Result containing the value produced by the gate, or an Err if either
  /// input wire didn't exist.
  ///
  /// # Examples
  ///
  /// ```
  /// use aoc2015::circuit::Circuit;
  ///
  /// let mut c = Circuit::new();
  /// c.input_signal("a", 0b001);
  /// c.input_signal("b", 0b010);
  /// assert_eq!(c.or("a", "b", "c"), Ok(0b011));
  /// assert_eq!(c.get_wire("c"), Some(&0b011));
  /// assert!(c.or("foo", "bar", "c").is_err());
  /// ```
  pub fn or(&mut self, lwire: &str, rwire: &str, owire: &str) -> Result<u16, &'static str> {
    let lsig = match self.wires.get(lwire) {
      Some(s) => s,
      None => return Err("left wire doesn't exist"),
    };
    let rsig = match self.wires.get(rwire) {
      Some(s) => s,
      None => return Err("right wire doesn't exist"),
    };
    let signal = lsig | rsig;
    self.wires.insert(String::from(owire), signal);
    Ok(signal)
  }

  /// Perform a bitwise OR on the signals stored in two wires, and send the
  /// result to a third wire.  The third wire will be added to the circuit if it
  /// does not yet exist.
  ///
  /// # Arguments
  ///
  /// * `lwire` - A string slice key representing the wire on the left side of
  ///             the OR gate.
  /// * `rwire` - A string slice key representing the wire on the right side of
  ///             the OR gate.
  ///
  /// # Returns
  ///
  /// A Result containing the value produced by the gate, or an Err if either
  /// input wire didn't exist.
  ///
  /// # Examples
  ///
  /// ```
  /// use aoc2015::circuit::Circuit;
  ///
  /// let mut c = Circuit::new();
  /// c.input_signal("a", 0b1010101010101010);
  /// assert_eq!(c.not("a", "b"), Ok(0b0101010101010101));
  /// assert_eq!(c.get_wire("b"), Some(&0b0101010101010101));
  /// assert!(c.not("foo", "c").is_err());
  /// ```
  pub fn not(&mut self, wire: &str, owire: &str) -> Result<u16, &'static str> {
    let sig = match self.wires.get(wire) {
      Some(s) => s,
      None => return Err("wire doesn't exist"),
    };
    let signal = sig ^ u16::MAX;
    self.wires.insert(String::from(owire), signal);
    Ok(signal)
  }

  /// Perform a binary left-shift on the signals stored in two wires, and send
  /// the result to a third wire.  The third wire will be added to the circuit
  /// if it does not yet exist.
  ///
  /// # Arguments
  ///
  /// * `wire` - A string slice key representing the wire on the left side of
  ///            the LSHIFT gate.
  /// * `dist` - A usize representing how far to shift the wire's signal.
  ///
  /// # Returns
  ///
  /// A Result containing the value produced by the gate, or an Err if either
  /// input wire didn't exist.
  ///
  /// # Examples
  ///
  /// ```
  /// use aoc2015::circuit::Circuit;
  ///
  /// let mut c = Circuit::new();
  /// c.input_signal("a", 0b001);
  /// assert_eq!(c.lshift("a", 2, "b"), Ok(0b100));
  /// assert_eq!(c.get_wire("b"), Some(&0b100));
  /// assert!(c.lshift("foo", 2, "c").is_err());
  /// ```
  pub fn lshift(&mut self, wire: &str, dist: usize, owire: &str) -> Result<u16, &'static str> {
    let sig = match self.wires.get(wire) {
      Some(s) => s,
      None => return Err("wire doesn't exist"),
    };
    let signal = sig << dist;
    self.wires.insert(String::from(owire), signal);
    Ok(signal)
  }

  /// Perform a binary left-shift on the signals stored in two wires, and send
  /// the result to a third wire.  The third wire will be added to the circuit
  /// if it does not yet exist.
  ///
  /// # Arguments
  ///
  /// * `wire` - A string slice key representing the wire on the left side of
  ///            the LSHIFT gate.
  /// * `dist` - A usize representing how far to shift the wire's signal.
  ///
  /// # Returns
  ///
  /// A Result containing the value produced by the gate, or an Err if either
  /// input wire didn't exist.
  ///
  /// # Examples
  ///
  /// ```
  /// use aoc2015::circuit::Circuit;
  ///
  /// let mut c = Circuit::new();
  /// c.input_signal("a", 0b100);
  /// assert_eq!(c.rshift("a", 2, "b"), Ok(0b001));
  /// assert_eq!(c.get_wire("b"), Some(&0b001));
  /// assert!(c.lshift("foo", 2, "c").is_err());
  /// ```
  pub fn rshift(&mut self, wire: &str, dist: usize, owire: &str) -> Result<u16, &'static str> {
    let sig = match self.wires.get(wire) {
      Some(s) => s,
      None => return Err("wire doesn't exist"),
    };
    let signal = sig >> dist;
    self.wires.insert(String::from(owire), signal);
    Ok(signal)
  }
}
