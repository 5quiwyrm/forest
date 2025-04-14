use std::fmt;

#[derive(Clone, PartialEq)]
pub enum ForestValue {
    Nil,
    Int(i64),
    String(String),
    Table(Vec<(ForestValue, ForestValue)>),
}

impl fmt::Display for ForestValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ForestValue::Nil => write!(f, "nil"),
            ForestValue::Int(i) => write!(f, "{}", i),
            ForestValue::String(s) => write!(f, "{}", s),
            ForestValue::Table(t) => {
                let mut ret = String::new();
                ret.push('{');
                for (key, val) in t {
                    ret.push_str(format!("{} {} ", key, val).as_str());
                }
                ret.pop();
                ret.push('}');
                write!(f, "{}", ret)
            }
        }
    }
}

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
            Self::Exit => write!(f, "Exit"),
        }
    }
}

enum ForestError {
    Halt,
    Underflow,
    TypeMismatch(ForestValue, ForestValue),
    UnbalancedIfEnd,
    UnbalancedLoopEnd,
}

impl fmt::Display for ForestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Halt => write!(f, "Halted!"),
            Self::Underflow => write!(f, "Underflow!"),
            Self::TypeMismatch(v, t) => write!(f, "Expceted: {}, got {}", t, v),
            Self::UnbalancedIfEnd => write!(f, "Unbalanced IfEnd!"),
            Self::UnbalancedLoopEnd => write!(f, "Unbalanced LoopEnd!"),
        }
    }
}

pub struct ForestRuntime {
    stack: Vec<ForestValue>,
    program: Vec<ForestInstruction>,
    programidx: usize,
    jumplist: Vec<usize>,
}

enum ForestDumpError {
    ProgramidxOOB,
}

impl ForestRuntime {
    pub fn new(inptprogram: &[ForestInstruction]) -> Self {
        ForestRuntime {
            stack: vec![],
            program: inptprogram.to_vec(),
            programidx: 0,
            jumplist: vec![],
        }
    }

    pub fn step(&mut self) -> Result<(), ForestError> {
        let mut inst = &self.program[self.programidx];
        match inst {
            ForestInstruction::Push(v) => {
                self.stack.push(v.clone());
                self.programidx += 1;
                Ok(())
            }
            ForestInstruction::Duplicate => {
                if self.stack.len() < 1 {
                    Err(ForestError::Underflow)
                } else {
                    self.stack.push(self.stack[self.stack.len() - 1].clone());
                    self.programidx += 1;
                    Ok(())
                }
            }
            ForestInstruction::Drop => {
                _ = self.stack.pop();
                self.programidx += 1;
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
                            self.programidx += 1;
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
                            self.programidx += 1;
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
                            self.programidx += 1;
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
                            self.programidx += 1;
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
                    self.programidx += 1;
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
                            self.programidx += 1;
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
                    print!("{}", a);
                    self.programidx += 1;
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
                                .filter(|s| s.0 == key)
                                .nth(0)
                                .unwrap_or(&(ForestValue::Nil, ForestValue::Nil))
                                .1
                                .clone(),
                        );
                        self.programidx += 1;
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
                    let val = self.stack.pop().unwrap();
                    let key = self.stack.pop().unwrap();
                    if key == ForestValue::Nil {
                        return Err(ForestError::TypeMismatch(key, ForestValue::Nil));
                    }
                    let table = self.stack.pop().unwrap();
                    if let ForestValue::Table(t) = table {
                        let mut tt = t;
                        tt.push((key, val));
                        self.stack.push(ForestValue::Table(tt));
                    } else {
                        return Err(ForestError::TypeMismatch(table, ForestValue::Table(vec![])));
                    }
                    self.programidx += 1;
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
                                .map(|(i, s)| (ForestValue::Int(i as i64), s.1.clone()))
                                .collect(),
                        ));
                        self.programidx += 1;
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
                            self.programidx += 1;
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
                    self.programidx += 1;
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
                    self.programidx += 1;
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
                    self.programidx += 1;
                    Ok(())
                }
            }
            ForestInstruction::Exit => Err(ForestError::Halt),
            ForestInstruction::If => {
                if self.stack[self.stack.len() - 1] == ForestValue::Nil {
                    let mut layers = 1;
                    while layers != 0 {
                        self.programidx += 1;
                        inst = &self.program[self.programidx];
                        match inst {
                            ForestInstruction::If => {
                                layers += 1;
                            }
                            ForestInstruction::IfEnd => {
                                layers -= 1;
                            }
                            _ => {}
                        }
                    }
                } else {
                    self.programidx += 1;
                }
                Ok(())
            }
            ForestInstruction::IfEnd => {
                self.programidx += 1;
                Ok(())
            }
            ForestInstruction::Loop => {
                self.jumplist.push(self.programidx);
                self.programidx += 1;
                Ok(())
            }
            ForestInstruction::LoopEnd => {
                if let Some(a) = self.jumplist.pop() {
                    self.programidx = a;
                    Ok(())
                } else {
                    Err(ForestError::UnbalancedLoopEnd)
                }
            }
            ForestInstruction::Break => {
                let mut layers = 1;
                while layers != 0 {
                    self.programidx += 1;
                    inst = &self.program[self.programidx];
                    match inst {
                        ForestInstruction::Loop => {
                            layers += 1;
                        }
                        ForestInstruction::LoopEnd => {
                            layers -= 1;
                        }
                        _ => {}
                    }
                }
                self.programidx += 1;
                Ok(())
            }
        }
    }

    pub fn dump(&self) -> Result<(), ForestDumpError> {
        println!("Stack: ");
        for val in &self.stack {
            println!("  {}", val);
        }
        println!("programidx: {}", self.programidx);
        if self.programidx >= self.program.len() {
            Err(ForestDumpError::ProgramidxOOB)
        } else {
            println!("Current instruction: {}", self.program[self.programidx]);
            Ok(())
        }
    }
}
