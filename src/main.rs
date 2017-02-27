extern crate rustyline;
extern crate calc;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use std::collections::HashMap;
use std::fmt::Display;

use calc::engine::{Engine, LineResult};

fn main() {
    let mut engine = Engine::new();
    let mut rl = Editor::<()>::new();
    let mut counter = 0;
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                if line == ":q" || line == ":quit" {
                    return;
                }
                if line.starts_with(":") {
                    if execute_cmd(&line, &engine) {
                        continue;
                    }
                }
                rl.add_history_entry(&line);
                let result = engine.eval_line(line);
                match result {
                    LineResult::Error(s) => {
                        println!("    error: {}", s)
                    },
                    LineResult::Value(v) => {
                        let counter_str = format!("${}", counter);
                        println!("    {}: {}", counter_str, v);
                        engine.set_val(counter_str, v);
                        counter += 1;
                    },
                    LineResult::Success => {}
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("[Interrupt]");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("[EOF]");
                break
            },
            Err(err) => {
                println!("[Runtime error: {:?}]", err);
                break
            }
        }
    }
}

fn execute_cmd(cmd: &str, engine: &Engine) -> bool {
    match cmd {
        ":vars" => {
            print_hashmap(engine.vars());
        },
        ":fns" => {
            for elem in engine.functions() {
                println!("    {} : {:?}", elem.0, elem.1.args());
            }
        },
        _ => return false
    }
    true
}

fn print_hashmap<T>(hm: &HashMap<String, T>) where T: Display {
    for elem in hm {
        println!("    {} : {}", elem.0, elem.1);
    }
}
