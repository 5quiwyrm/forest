use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Clone)]
pub struct TablePair {
    key: ForestValue,
    value: ForestValue,
}

#[derive(Clone, PartialEq)]
pub enum ForestValue {
    Nil,
    Int(i64),
    String(String),
    Table(Vec<TablePair>),
}

impl fmt::Display for ForestValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ForestValue::Nil => write!(f, "nil"),
            ForestValue::Int(i) => write!(f, "{}", i),
            ForestValue::String(s) => write!(f, "{:?}", s),
            ForestValue::Table(t) => {
                let mut ret = String::new();
                ret.push('{');
                for TablePair { key, value } in t {
                    ret.push_str(format!("{} {} ", key, value).as_str());
                }
                ret.pop();
                ret.push('}');
                write!(f, "{}", ret)
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum ForestInstruction {
    Push(ForestValue),
    Duplicate,
    Drop,
    Add,
    Subtract,
    Multiply,
    Divide,
    Stringify,
    Concatenate,
    Print,
    Get,
    Associate,
    Keys,
    If,
    IfEnd,
    And,
    Or,
    Not,
    Eq,
    Loop,
    LoopEnd,
    Break,
    MakeWord(String),
    EndWord,
    InvokeWord(String),
    Swap,
    Exit,
}

impl fmt::Display for ForestInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Push(v) => write!(f, "Push {}", v),
            Self::Duplicate => write!(f, "Duplicate"),
            Self::Drop => write!(f, "Drop"),
            Self::Add => write!(f, "Add"),
            Self::Subtract => write!(f, "Subtract"),
            Self::Multiply => write!(f, "Multiply"),
            Self::Divide => write!(f, "Divide"),
            Self::Stringify => write!(f, "Stringify"),
            Self::Concatenate => write!(f, "Concatenate"),
            Self::Print => write!(f, "Print"),
            Self::Get => write!(f, "Get"),
            Self::Associate => write!(f, "Associate"),
            Self::Keys => write!(f, "Keys"),
            Self::If => write!(f, "If"),
            Self::IfEnd => write!(f, "IfEnd"),
            Self::And => write!(f, "And"),
            Self::Or => write!(f, "Or"),
            Self::Not => write!(f, "Not"),
            Self::Eq => write!(f, "Eq"),
            Self::Loop => write!(f, "Loop"),
            Self::LoopEnd => write!(f, "LoopEnd"),
            Self::Break => write!(f, "Break"),
            Self::MakeWord(w) => write!(f, "MakeWord {}", w),
            Self::EndWord => write!(f, "EndWord"),
            Self::InvokeWord(w) => write!(f, "InvokeWord {}", w),
            Self::Swap => write!(f, "Swap"),
            Self::Exit => write!(f, "Exit"),
        }
    }
}

#[allow(dead_code)]
pub enum ForestError {
    Halt,
    Underflow,
    TypeMismatch(ForestValue, ForestValue),
    UnbalancedIfEnd,
    UnbalancedLoopEnd,
    EndedWithoutHalting,
    UnbalancedWordEnd,
    UseOfUndeclaredWord(String),
    Unimplemented(String),
}

impl fmt::Display for ForestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Halt => write!(f, "Halted!"),
            Self::Underflow => write!(f, "Underflow!"),
            Self::TypeMismatch(v, t) => write!(f, "Expceted: {}, got {}", t, v),
            Self::UnbalancedIfEnd => write!(f, "Unbalanced IfEnd!"),
            Self::UnbalancedLoopEnd => write!(f, "Unbalanced LoopEnd!"),
            Self::EndedWithoutHalting => write!(f, "Program ended without halting!"),
            Self::UnbalancedWordEnd => write!(f, "Unbalanced WordEnd!"),
            Self::UseOfUndeclaredWord(n) => write!(f, "Use of undeclared word {}!", n),
            Self::Unimplemented(feature) => write!(f, "{} is not implemented yet!", feature),
        }
    }
}

pub struct ForestRuntime {
    stack: Vec<ForestValue>,
    program: Vec<ForestInstruction>,
    jumplist: Vec<Vec<ForestInstruction>>,
    wordlist: HashMap<String, Vec<ForestInstruction>>,
}

#[allow(dead_code)]
pub enum ForestDumpError {
    ProgramidxOOB,
}

impl ForestRuntime {
    pub fn new(inptprogram: &[ForestInstruction]) -> Self {
        let mut revprogram: Vec<ForestInstruction> = inptprogram.to_vec();
        revprogram.reverse();
        ForestRuntime {
            stack: vec![],
            program: revprogram,
            jumplist: vec![],
            wordlist: HashMap::new(),
        }
    }

    pub fn push_instrs(&mut self, instrs: &[ForestInstruction]) -> () {
        let mut revprogram: Vec<ForestInstruction> = instrs.to_vec();
        revprogram.reverse();
        self.program.append(&mut revprogram);
    }

    pub fn step(&mut self) -> Result<(), ForestError> {
        if let Some(inst) = self.program.pop() {
            match inst {
                ForestInstruction::Push(v) => {
                    self.stack.push(v.clone());
                    Ok(())
                }
                ForestInstruction::Duplicate => {
                    if self.stack.len() < 1 {
                        Err(ForestError::Underflow)
                    } else {
                        self.stack.push(self.stack[self.stack.len() - 1].clone());
                        Ok(())
                    }
                }
                ForestInstruction::Drop => {
                    _ = self.stack.pop();
                    Ok(())
                }
                ForestInstruction::Add => {
                    if self.stack.len() < 2 {
                        Err(ForestError::Underflow)
                    } else {
                        let a = self.stack.pop().unwrap();
                        if let ForestValue::Int(va) = a {
                            let b = self.stack.pop().unwrap();
                            if let ForestValue::Int(vb) = b {
                                self.stack.push(ForestValue::Int(va + vb));
                                Ok(())
                            } else {
                                Err(ForestError::TypeMismatch(b, ForestValue::Int(0)))
                            }
                        } else {
                            Err(ForestError::TypeMismatch(a, ForestValue::Int(0)))
                        }
                    }
                }
                ForestInstruction::Subtract => {
                    if self.stack.len() < 2 {
                        Err(ForestError::Underflow)
                    } else {
                        let a = self.stack.pop().unwrap();
                        if let ForestValue::Int(va) = a {
                            let b = self.stack.pop().unwrap();
                            if let ForestValue::Int(vb) = b {
                                self.stack.push(ForestValue::Int(vb - va));
                                Ok(())
                            } else {
                                Err(ForestError::TypeMismatch(b, ForestValue::Int(0)))
                            }
                        } else {
                            Err(ForestError::TypeMismatch(a, ForestValue::Int(0)))
                        }
                    }
                }
                ForestInstruction::Multiply => {
                    if self.stack.len() < 2 {
                        Err(ForestError::Underflow)
                    } else {
                        let a = self.stack.pop().unwrap();
                        if let ForestValue::Int(va) = a {
                            let b = self.stack.pop().unwrap();
                            if let ForestValue::Int(vb) = b {
                                self.stack.push(ForestValue::Int(va * vb));
                                Ok(())
                            } else {
                                Err(ForestError::TypeMismatch(b, ForestValue::Int(0)))
                            }
                        } else {
                            Err(ForestError::TypeMismatch(a, ForestValue::Int(0)))
                        }
                    }
                }
                ForestInstruction::Divide => {
                    if self.stack.len() < 2 {
                        Err(ForestError::Underflow)
                    } else {
                        let a = self.stack.pop().unwrap();
                        if let ForestValue::Int(va) = a {
                            let b = self.stack.pop().unwrap();
                            if let ForestValue::Int(vb) = b {
                                self.stack.push(ForestValue::Int(vb / va));
                                Ok(())
                            } else {
                                Err(ForestError::TypeMismatch(b, ForestValue::Int(0)))
                            }
                        } else {
                            Err(ForestError::TypeMismatch(a, ForestValue::Int(0)))
                        }
                    }
                }
                ForestInstruction::Stringify => {
                    if self.stack.len() < 1 {
                        Err(ForestError::Underflow)
                    } else {
                        let a = self.stack.pop().unwrap();
                        self.stack.push(ForestValue::String(format!("{}", a)));
                        Ok(())
                    }
                }
                ForestInstruction::Concatenate => {
                    if self.stack.len() < 2 {
                        Err(ForestError::Underflow)
                    } else {
                        let a = self.stack.pop().unwrap();
                        if let ForestValue::String(va) = a {
                            let b = self.stack.pop().unwrap();
                            if let ForestValue::String(vb) = b {
                                self.stack
                                    .push(ForestValue::String(format!("{}{}", va, vb)));
                                Ok(())
                            } else {
                                Err(ForestError::TypeMismatch(
                                    b,
                                    ForestValue::String("".to_string()),
                                ))
                            }
                        } else {
                            Err(ForestError::TypeMismatch(
                                a,
                                ForestValue::String("".to_string()),
                            ))
                        }
                    }
                }
                ForestInstruction::Print => {
                    if self.stack.len() < 1 {
                        Err(ForestError::Underflow)
                    } else {
                        let a = self.stack.pop().unwrap();
                        print!("{a}");
                        Ok(())
                    }
                }
                ForestInstruction::Get => {
                    if self.stack.len() < 2 {
                        Err(ForestError::Underflow)
                    } else {
                        let key = self.stack.pop().unwrap();
                        if key == ForestValue::Nil {
                            return Err(ForestError::TypeMismatch(key, ForestValue::Nil));
                        }
                        let forest_table = self.stack[self.stack.len() - 1].clone();
                        if let ForestValue::Table(t) = forest_table {
                            self.stack.push(
                                t.iter()
                                    .filter(|s| s.key == key)
                                    .nth(0)
                                    .unwrap_or(&TablePair {
                                        key: ForestValue::Nil,
                                        value: ForestValue::Nil,
                                    })
                                    .value
                                    .clone(),
                            );
                            Ok(())
                        } else {
                            Err(ForestError::TypeMismatch(
                                forest_table,
                                ForestValue::Table(vec![]),
                            ))
                        }
                    }
                }
                ForestInstruction::Associate => {
                    if self.stack.len() < 3 {
                        Err(ForestError::Underflow)
                    } else {
                        let value = self.stack.pop().unwrap();
                        let key = self.stack.pop().unwrap();
                        if key == ForestValue::Nil {
                            return Err(ForestError::TypeMismatch(key, ForestValue::Nil));
                        }
                        let table = self.stack.pop().unwrap();
                        if let ForestValue::Table(t) = table {
                            let mut tt = t;
                            tt.push(TablePair { key, value });
                            self.stack.push(ForestValue::Table(tt));
                        } else {
                            return Err(ForestError::TypeMismatch(
                                table,
                                ForestValue::Table(vec![]),
                            ));
                        }
                        Ok(())
                    }
                }
                ForestInstruction::Keys => {
                    if self.stack.len() < 1 {
                        Err(ForestError::Underflow)
                    } else {
                        let table = self.stack.pop().unwrap();
                        if let ForestValue::Table(t) = table {
                            self.stack.push(ForestValue::Table(
                                t.iter()
                                    .enumerate()
                                    .map(|(i, s)| TablePair {
                                        key: ForestValue::Int(i as i64),
                                        value: s.value.clone(),
                                    })
                                    .collect(),
                            ));
                            Ok(())
                        } else {
                            Err(ForestError::TypeMismatch(table, ForestValue::Table(vec![])))
                        }
                    }
                }
                ForestInstruction::And => {
                    if self.stack.len() < 2 {
                        Err(ForestError::Underflow)
                    } else {
                        let a = self.stack.pop().unwrap();
                        if let ForestValue::Int(va) = a {
                            let b = self.stack.pop().unwrap();
                            if let ForestValue::Int(vb) = b {
                                if va != 0 && vb != 0 {
                                    self.stack.push(ForestValue::Int(1));
                                } else {
                                    self.stack.push(ForestValue::Nil);
                                }
                                Ok(())
                            } else {
                                Err(ForestError::TypeMismatch(b, ForestValue::Int(0)))
                            }
                        } else {
                            Err(ForestError::TypeMismatch(a, ForestValue::Int(0)))
                        }
                    }
                }
                ForestInstruction::Or => {
                    if self.stack.len() < 2 {
                        Err(ForestError::Underflow)
                    } else {
                        let a = self.stack.pop().unwrap();
                        let b = self.stack.pop().unwrap();
                        if a != ForestValue::Int(0) || b != ForestValue::Int(0) {
                            self.stack.push(ForestValue::Int(1));
                        } else {
                            self.stack.push(ForestValue::Nil);
                        }
                        Ok(())
                    }
                }
                ForestInstruction::Not => {
                    if self.stack.len() < 1 {
                        Err(ForestError::Underflow)
                    } else {
                        let a = self.stack.pop().unwrap();
                        if a != ForestValue::Nil {
                            self.stack.push(ForestValue::Nil);
                        } else {
                            self.stack.push(ForestValue::Int(1));
                        }
                        Ok(())
                    }
                }
                ForestInstruction::Eq => {
                    if self.stack.len() < 2 {
                        Err(ForestError::Underflow)
                    } else {
                        let a = self.stack.pop().unwrap();
                        let b = self.stack.pop().unwrap();
                        if a == b {
                            self.stack.push(ForestValue::Int(1));
                        } else {
                            self.stack.push(ForestValue::Nil);
                        }
                        Ok(())
                    }
                }
                ForestInstruction::Swap => {
                    if self.stack.len() < 2 {
                        Err(ForestError::Underflow)
                    } else {
                        let a = self.stack.pop().unwrap();
                        let b = self.stack.pop().unwrap();
                        self.stack.push(a);
                        self.stack.push(b);
                        Ok(())
                    }
                }
                ForestInstruction::Exit => Err(ForestError::Halt),
                ForestInstruction::If => {
                    if self.stack[self.stack.len() - 1] == ForestValue::Nil {
                        let mut layers = 1;
                        while layers != 0 {
                            if let Some(inst) = self.program.pop() {
                                match inst {
                                    ForestInstruction::If => {
                                        layers += 1;
                                    }
                                    ForestInstruction::IfEnd => {
                                        layers -= 1;
                                    }
                                    _ => {}
                                }
                            } else {
                                return Err(ForestError::EndedWithoutHalting);
                            }
                        }
                    }
                    Ok(())
                }
                ForestInstruction::IfEnd => Ok(()),
                ForestInstruction::Loop => {
                    self.jumplist.push(self.program.clone());
                    Ok(())
                }
                ForestInstruction::LoopEnd => {
                    if let Some(a) = self.jumplist.pop() {
                        let mut p = a;
                        p.push(ForestInstruction::Loop);
                        self.program = p;
                        Ok(())
                    } else {
                        Err(ForestError::UnbalancedLoopEnd)
                    }
                }
                ForestInstruction::Break => {
                    let mut layers = 1;
                    while layers != 0 {
                        if let Some(inst) = self.program.pop() {
                            match inst {
                                ForestInstruction::Loop => {
                                    layers += 1;
                                }
                                ForestInstruction::LoopEnd => {
                                    layers -= 1;
                                }
                                _ => {}
                            }
                        } else {
                            return Err(ForestError::UnbalancedLoopEnd);
                        }
                    }
                    Ok(())
                }
                ForestInstruction::MakeWord(name) => {
                    let mut instructions: Vec<ForestInstruction> = Vec::new();
                    let mut layers = 1;
                    'read: loop {
                        if let Some(inst) = self.program.pop() {
                            match inst {
                                ForestInstruction::MakeWord(_) => {
                                    layers += 1;
                                }
                                ForestInstruction::EndWord => {
                                    layers -= 1;
                                    if layers == 0 {
                                        break 'read;
                                    }
                                }
                                _ => {}
                            }
                            instructions.push(inst);
                        } else {
                            return Err(ForestError::UnbalancedWordEnd);
                        }
                    }
                    self.wordlist.insert(name, instructions);
                    Ok(())
                }
                ForestInstruction::EndWord => Ok(()),
                ForestInstruction::InvokeWord(name) => {
                    let instrs = match self.wordlist.get(&name) {
                        Some(v) => v.clone(),
                        None => {
                            return Err(ForestError::UseOfUndeclaredWord(name));
                        }
                    };
                    self.push_instrs(&instrs);
                    Ok(())
                } // _ => Err(ForestError::Unimplemented(
                  //     "Variant is not implemented yet!".to_string(),
                  // )),
            }
        } else {
            return Err(ForestError::EndedWithoutHalting);
        }
    }

    pub fn dump(&self) -> Result<(), ForestDumpError> {
        println!("\n\n\nStack: ");
        for val in &self.stack {
            println!("  {}", val);
        }
        let inst = &self.program[self.program.len() - 1];
        println!("Current instruction: {}", inst);
        println!("Jumplists: {}", self.jumplist.len());
        println!("Wordlist:");
        self.wordlist.iter().for_each(|w| {
            print!("  {} | ", w.0);
            w.1.iter().for_each(|i| print!("{} ", i));
            println!();
        });
        Ok(())
    }
}

#[macro_export]
macro_rules! execute_runtime {
    ($runtime_name: ident, $dump: expr) => {
        loop {
            if $dump {
                match $runtime_name.dump() {
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
            match $runtime_name.step() {
                Ok(_) => {}
                Err(e) => {
                    match e {
                        ForestError::Halt => {}
                        _ => {
                            eprintln!("{}", e);
                        }
                    };
                    break;
                }
            };
        }
    };
}
