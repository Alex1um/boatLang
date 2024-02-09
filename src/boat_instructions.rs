use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum BoatCmd {
    Push, // Push value on top of stack
    Pop,
    Goto, // Go to instruction with index
    Input, // Block until input value from pin and push it on top of stack
    Output, // Pop value from top of stack and output it to pin
    Add, // Pop two values from top of stack and push their sum
    Sub, // Pop two values from top of stack and push their difference
    Mul, // Pop two values from top of stack and push their product
    Div, // Pop two values from top of stack and push their quotient
    Conc, // Pop two values from top of stack and push their concatenation
    KVSet, // Pop value from top of stack and set key to that value on key-value storage
    KVDel, // Delete value by key from key-value storage
    Cmp, // Pop value from top of stack and goto instruction if it is 1
    Eq, // Pop two values from top of stack and push 1 if they are equal or 0
    Gt, // Pop two values from top of stack and push 1 if the first is greater than the second or 0
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoatArg {
    FromStack,
    Const(String),
    FromKVS(String),
}

#[derive(Debug, Clone)]
pub struct BoatIns {
    pub cmd: BoatCmd,
    pub args: Vec<BoatArg>,
}

impl Display for BoatCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BoatCmd::*;
        match self {
            Push => write!(f, "p"),
            Pop => write!(f, "x"),
            Goto  => write!(f, "g"),
            Input => write!(f, "i"),
            Output => write!(f, "o"),
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            Div => write!(f, "/"),
            Conc => write!(f, ".."),
            KVSet => write!(f, "ka"),
            KVDel => write!(f, "kd"),
            Cmp  => write!(f, "c"),
            Eq => write!(f, "="),
            Gt => write!(f, ">"),
        }
    }
}

impl Display for BoatArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoatArg::FromStack => write!(f, "$"),
            BoatArg::Const(s) => write!(f, "{s}"),
            BoatArg::FromKVS(s) => write!(f, "${s}"),
        }
    }
}

impl Display for BoatIns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.args.is_empty() {
            write!(f, "{};", self.cmd)?;
        } else {
            let args = self.args
                .iter()
                .map(|arg| format!("{arg}"))
                .collect::<Vec<String>>()
                .join(" ");
            write!(f, "{} {args};", self.cmd)?;
        }
        Ok(())
    }
}

pub fn translated_to_string(inses: Vec<BoatIns>) -> String {
    inses
        .into_iter()
        .map(|ins| format!("{ins}"))
        .collect::<Vec<String>>()
        .join("")
}

pub fn translated_debug(inses: &Vec<BoatIns>) {
    let len = inses.len().to_string().len();
    for (i, ins) in inses.iter().enumerate() {
        let line = i + 1;
        println!("{line:>len$}| {ins}");
    }
}
