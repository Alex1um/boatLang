use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum BoatIns {
    Push { value: String }, // Push value on top of stack
    Pop, // Pop value from top of stack
    Goto { ins: u32 }, // Go to instruction with index
    Clone, // Clone value on top of stack
    Input { pin: u32 }, // Block until input value from pin and push it on top of stack
    Output{ pin: u32 }, // Pop value from top of stack and output it to pin
    Add, // Pop two values from top of stack and push their sum
    Sub, // Pop two values from top of stack and push their difference
    Mul, // Pop two values from top of stack and push their product
    Div, // Pop two values from top of stack and push their quotient
    Conc, // Pop two values from top of stack and push their concatenation
    KVSet { key: String }, // Pop value from top of stack and set key to that value on key-value storage
    KVGet { key: String }, // Push value from key-value storage to top of stack
    KVDel { key: String }, // Delete value by key from key-value storage
    Sleep { seconds: String }, // sleep for passed amount of time
    Cmp { ins: u32 }, // Pop value from top of stack and goto instruction if it is 1
    Eq, // Pop two values from top of stack and push 1 if they are equal or 0
    Gt, // Pop two values from top of stack and push 1 if the first is greater than the second or 0
}

impl Display for BoatIns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BoatIns::*;
        match self {
            Push { value } => write!(f, "p {};", value),
            Pop => write!(f, "d;"),
            Goto { ins } => write!(f, "g {};", ins),
            Cmp { ins } => write!(f, "c {};", ins),
            Clone => write!(f, "r;"),
            Input { pin } => write!(f, "i {};", pin),
            Output { pin } => write!(f, "o {};", pin),
            Add => write!(f, "+;"),
            Sub => write!(f, "-;"),
            Mul => write!(f, "*;"),
            Div => write!(f, "/;"),
            Conc => write!(f, "..;"),
            KVSet { key } => write!(f, "ks {};", key),
            KVGet { key } => write!(f, "kg {};", key),
            KVDel { key } => write!(f, "kd {};", key),
            Sleep { seconds } => write!(f, "s {};", seconds),
            Eq => write!(f, "=;"),
            Gt => write!(f, ">;"),
        }
    }
}

pub fn translated_to_string(inses: Vec<BoatIns>) -> String {
    inses
        .into_iter()
        .map(|ins| format!("{}", ins))
        .collect::<Vec<String>>()
        .join("")
}
