use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoatCmd {
    Push, // Push value on top of stack
    Goto, // Go to instruction with index
    Input, // Block until input value from pin and push it on top of stack
    Output, // Output to pin at pos 1 value at pos 2
    Add, // Push sum of two values to stack
    Sub, // Push difference of two values to stack
    Mul, // Push product of two values to stack
    Div, // Push quotient of two values to stack
    Conc, // Push concatenation of two values to stack
    KVReSet, // Drops existing key and assign to value in key-value storage.
    KVSet, // Set key to value in key-value storage
    KVDel, // Delete value by key from key-value storage
    Cmp, // goto instruction at pos 2 if pos 1 value is 1
    Lt, // Push 1 if the first is less than the second one or 0
    Eq, // Push 1 if values are equal or 0
    Gt, // Push 1 if the first is greater than the second or 0
    Sleep, // Do nothing for a duration
    Display, // Paint 7x7 display pixel in x, y
    DisplayClear, // Clear 7x7 display
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
            KVReSet => write!(f, "kr"),
            Cmp  => write!(f, "c"),
            Eq => write!(f, "="),
            Lt => write!(f, "<"),
            Gt => write!(f, ">"),
            Sleep => write!(f, "s"),
            Display => write!(f, "di"),
            DisplayClear => write!(f, "dc"),
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
            write!(f, "{}", self.cmd)?;
        } else {
            let args = self.args
                .iter()
                .map(|arg| format!("{arg}"))
                .collect::<Vec<String>>()
                .join(" ");
            write!(f, "{} {args}", self.cmd)?;
        }
        Ok(())
    }
}

pub fn translated_to_string(inses: Vec<BoatIns>) -> String {
    inses
        .into_iter()
        .map(|ins| format!("{ins};"))
        .collect::<Vec<String>>()
        .join("")
}

pub fn translated_to_string2(inses: Vec<BoatIns>, preety: bool) -> String {
    let iter = inses
        .into_iter()
        .enumerate();
    if preety {
        let len = iter.len().to_string().len();
        iter.map(|(i, ins)| format!("{:>len$}|{ins}", i + 1))
            .collect::<Vec<String>>()
            .join("\n")
    } else {
        iter.map(|(i, ins)| format!("|{}|{ins}", i + 1))
            .collect::<Vec<String>>()
            .join("")
    }
}
