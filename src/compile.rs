use crate::forest_runtime::{ForestInstruction as fi, ForestValue};
use std::fmt;

pub enum ForestCompileError {
    OutOfBoundsWord,
    // this means a word that doesn't end by the time the program ends compilation.
}

impl fmt::Display for ForestCompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfBoundsWord => write!(f, "Makeword (:) was last instruction!"),
        }
    }
}

pub fn parse_number(inpt: &str) -> Option<fi> {
    match inpt.parse::<i64>() {
        Ok(i) => Some(fi::Push(ForestValue::Int(i))),
        Err(_) => None,
    }
}

pub fn unescape(escaped: &str) -> String {
    escaped
        .replace("\\r", "\r")
        .replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\\\", "\\")
        .replace("\\\"", "\"")
        .replace("\\s", " ")
}

pub fn parse_string(inpt: &str) -> Option<fi> {
    if inpt.len() > 1
        && inpt.chars().nth(0).unwrap() == '\"'
        && inpt.chars().nth_back(0).unwrap() == '\"'
    {
        Some(fi::Push(ForestValue::String(unescape(
            &inpt[1..(inpt.len() - 1)],
        ))))
    } else {
        None
    }
}

pub fn parse_nil(inpt: &str) -> Option<fi> {
    if inpt == "nil" {
        Some(fi::Push(ForestValue::Nil))
    } else {
        None
    }
}

pub fn parse_table(inpt: &str) -> Option<fi> {
    if inpt == "{}" {
        Some(fi::Push(ForestValue::Table(vec![])))
    } else {
        None
    }
}

pub fn compile(programstr: &str) -> Result<Vec<fi>, ForestCompileError> {
    let mut tokens = programstr.split_whitespace();
    let mut program: Vec<fi> = Vec::new();
    'compilation: loop {
        if let Some(tk) = tokens.next() {
            match tk {
                "dup" => program.push(fi::Duplicate),
                "drop" => program.push(fi::Drop),
                "+" => program.push(fi::Add),
                "-" => program.push(fi::Subtract),
                "*" => program.push(fi::Multiply),
                "/" => program.push(fi::Divide),
                "str" => program.push(fi::Stringify),
                "<>" => program.push(fi::Concatenate),
                "." => program.push(fi::Print),
                "get" => program.push(fi::Get),
                "assoc" => program.push(fi::Associate),
                "keys" => program.push(fi::Keys),
                "vals" => program.push(fi::Values),
                "if" => program.push(fi::If),
                "ifend" => program.push(fi::IfEnd),
                "&" => program.push(fi::And),
                "|" => program.push(fi::Or),
                "!" => program.push(fi::Not),
                "=" => program.push(fi::Eq),
                "[" => program.push(fi::Loop),
                "]" => program.push(fi::LoopEnd),
                "break" => program.push(fi::Break),
                "swap" => program.push(fi::Swap),
                "splat" => program.push(fi::Splat),
                ":" => {
                    if let Some(tk) = tokens.next() {
                        program.push(fi::MakeWord(tk.to_string()))
                    } else {
                        return Err(ForestCompileError::OutOfBoundsWord);
                    }
                }
                ";" => program.push(fi::EndWord),
                "exit" => program.push(fi::Exit),
                s => program.push({
                    parse_number(s)
                        .or_else(|| parse_string(s))
                        .or_else(|| parse_nil(s))
                        .or_else(|| parse_table(s))
                        .or_else(|| Some(fi::InvokeWord(s.to_string())))
                        .unwrap()
                }),
            }
        } else {
            break 'compilation;
        }
    }
    Ok(program)
}
