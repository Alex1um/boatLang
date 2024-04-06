use std::{collections::HashMap, io::{Write, BufRead, Read}, path::Display, thread::sleep, time::Duration};

use crate::boat_instructions::{BoatCmd, BoatIns, BoatArg};

type Kvs = HashMap<String, Vec<String>>;

fn get_arg(arg: &BoatArg, stack: &mut Vec<String>, kvs: &Kvs) -> String {
    match arg {
        BoatArg::Const(c) => { c.to_string() },
        BoatArg::FromStack => { stack.pop().expect("stack has values") }
        BoatArg::FromKVS(k) => { kvs.get(k).unwrap_or_else(|| panic!("kvs has key {k}")).last().expect("kvs has value").to_string() }
    }
}

pub fn interpret(program: &Vec<BoatIns>, mut output: impl Write, mut input: impl BufRead, debug: bool) {
    let mut stack = Vec::<String>::new();
    let mut kvs = Kvs::new();
    let mut i = 0;
    let l = program.len();
    while i < l {
        let ins = &program[i];
        if debug {
            writeln!(output, "{}| {ins} -- {:?} -- {:?}", i + 1, stack, kvs);
        }
        let BoatIns {args, cmd} = ins;
        match cmd {
            BoatCmd::Push => {
                let arg = get_arg(args.first().expect("Push has 1 arg"), &mut stack, &kvs);
                stack.push(arg);
            },
            BoatCmd::Goto => {
                i = get_arg(args.first().expect("Goto has 1 arg"), &mut stack, &kvs).parse::<usize>().expect("Goto arg is integer") - 1usize;
                continue;
            },
            BoatCmd::Input => {
                let mut s = String::new();
                input.read_line(&mut s).expect("success read");
                stack.push(s.trim().to_string());
            },
            BoatCmd::Output => {
                let out_num = get_arg(args.get(0).expect("Output has 1 arg"), &mut stack, &kvs);
                let out = get_arg(args.get(1).expect("Output has 2 args"), &mut stack, &kvs);
                writeln!(output, "{out_num} <- {out}");
            },
            BoatCmd::Add => {
                let arg1 = get_arg(args.get(0).expect("operation has 1 arg"), &mut stack, &kvs);
                let arg2 = get_arg(args.get(1).expect("operation has 2 args"), &mut stack, &kvs);
                let parsed1 = arg1.trim().parse::<f32>().expect("argument 1 is f32");
                let parsed2 = arg2.trim().parse::<f32>().expect("argument 2 is f32");
                stack.push((parsed1 + parsed2).to_string());
            },
            BoatCmd::Sub => {
                let arg1 = get_arg(args.get(0).expect("operation has 1 arg"), &mut stack, &kvs);
                let arg2 = get_arg(args.get(1).expect("operation has 2 args"), &mut stack, &kvs);
                let parsed1 = arg1.trim().parse::<f32>().expect("argument 1 is f32");
                let parsed2 = arg2.trim().parse::<f32>().expect("argument 2 is f32");
                stack.push((parsed1 - parsed2).to_string());
            },
            BoatCmd::Mul => {
                let arg1 = get_arg(args.get(0).expect("operation has 1 arg"), &mut stack, &kvs);
                let arg2 = get_arg(args.get(1).expect("operation has 2 args"), &mut stack, &kvs);
                let parsed1 = arg1.trim().parse::<f32>().expect("argument 1 is f32");
                let parsed2 = arg2.trim().parse::<f32>().expect("argument 2 is f32");
                stack.push((parsed1 * parsed2).to_string());
            },
            BoatCmd::Div => {
                let arg1 = get_arg(args.get(0).expect("operation has 1 arg"), &mut stack, &kvs);
                let arg2 = get_arg(args.get(1).expect("operation has 2 args"), &mut stack, &kvs);
                let parsed1 = arg1.trim().parse::<f32>().expect("argument 1 is f32");
                let parsed2 = arg2.trim().parse::<f32>().expect("argument 2 is f32");
                stack.push((parsed1 / parsed2).to_string());
            },
            BoatCmd::Conc => {
                let arg1 = get_arg(args.get(0).expect("operation has 1 arg"), &mut stack, &kvs);
                let arg2 = get_arg(args.get(1).expect("operation has 2 args"), &mut stack, &kvs);
                stack.push(format!("{arg1}{arg2}").to_string());
            },
            BoatCmd::KVSet => {
                let arg1 = get_arg(args.get(0).expect("kvset has 1 arg"), &mut stack, &kvs);
                let arg2 = get_arg(args.get(1).expect("kvset has 2 args"), &mut stack, &kvs);
                kvs.entry(arg1).and_modify(|e| e.push(arg2.clone())).or_insert(vec![arg2]);
            },
            BoatCmd::KVDel => {
                let arg1 = get_arg(args.get(0).expect("kvdel has 1 arg"), &mut stack, &kvs);
                kvs.entry(arg1).and_modify(|e| { e.pop(); });
            },
            BoatCmd::Cmp => {
                let arg1 = get_arg(args.get(0).expect("cmp has 1 arg"), &mut stack, &kvs);
                let arg2 = get_arg(args.get(1).expect("cmp has 1 arg"), &mut stack, &kvs);
                if arg1.parse::<f32>().expect("arg1 is numeric") == 0. {
                    i = arg2.parse::<usize>().expect("arg2 is u32") - 1;
                    continue;
                }
            },
            BoatCmd::Eq => {
                let arg1 = get_arg(args.get(0).expect("operation has 1 arg"), &mut stack, &kvs);
                let arg2 = get_arg(args.get(1).expect("operation has 2 args"), &mut stack, &kvs);
                stack.push(((arg1 == arg2) as usize as f32).to_string());
            },
            BoatCmd::Gt => {
                let arg1 = get_arg(args.get(0).expect("operation has 1 arg"), &mut stack, &kvs);
                let arg2 = get_arg(args.get(1).expect("operation has 2 args"), &mut stack, &kvs);
                let parsed1 = arg1.parse::<f32>().expect("argument 1 is f32");
                let parsed2 = arg2.parse::<f32>().expect("argument 2 is f32");
                stack.push(((parsed1 > parsed2) as usize as f32).to_string());
            },
            BoatCmd::Lt => {
                let arg1 = get_arg(args.get(0).expect("operation has 1 arg"), &mut stack, &kvs);
                let arg2 = get_arg(args.get(1).expect("operation has 2 args"), &mut stack, &kvs);
                let parsed1 = arg1.parse::<f32>().expect("argument 1 is f32");
                let parsed2 = arg2.parse::<f32>().expect("argument 2 is f32");
                stack.push(((parsed1 < parsed2) as usize as f32).to_string());
            },
            BoatCmd::KVReSet => {
                let arg1 = get_arg(args.get(0).expect("kvset has 1 arg"), &mut stack, &kvs);
                let arg2 = get_arg(args.get(1).expect("kvset has 2 args"), &mut stack, &kvs);
                kvs.entry(arg1).and_modify(|e| { e.pop(); e.push(arg2.clone()) }).or_insert(vec![arg2]);
            },
            BoatCmd::Sleep => {
                let arg1 = get_arg(args.get(0).expect("kvset has 1 arg"), &mut stack, &kvs);
                sleep(Duration::from_secs_f64(arg1.parse().expect("arg1 is f64")));
            }
            BoatCmd::Display => {
                unimplemented!();
            }
        };
        i += 1;
    }
}
