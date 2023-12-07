use anyhow::{anyhow, Context};
use atty::Stream;
use colored::*;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::io::BufRead;

mod common;

mod day1;

/* Common functions */

fn read_lines_stdin() -> anyhow::Result<Vec<String>> {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<String>, _>>()?;
    Ok(lines)
}

fn read_lines_from_file(filename: String) -> anyhow::Result<Vec<String>> {
    let file = std::fs::File::open(filename)?;
    let lines: Vec<String> = std::io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<String>, _>>()?;
    Ok(lines)
}

fn check_answer(answer: String, correct_answer: &str) -> anyhow::Result<()> {
    if answer == correct_answer {
        println!("{}", format!("CORRECT: {}", answer).green());
        Ok(())
    } else {
        println!(
            "{}",
            format!("incorrect: {}, expected {}", answer, correct_answer).red()
        );
        Err(anyhow!("incorrect"))
    }
}

enum TaskType {
    A,
    B,
}

type FnProcess = Box<dyn Fn(Vec<String>) -> String>;
struct Task {
    answer: common::Answer,
    process_a: FnProcess,
    process_b: FnProcess,
}
type Registry = std::collections::HashMap<usize, Task>;

/// Macro to register one day from module `$module`
macro_rules! day {
    ($registry: ident, $module: ident) => {
        //ANSWER::path is like aoc202y::dayX
        // Extract day number from module name
        let path = $module::ANSWER.path;
        let re: Regex = Regex::new(r".*::day(?P<num>[[:digit:]]+)$")?;
        let day: usize = re
            .captures(path)
            .context("No match")?
            .name("num")
            .context("No match")?
            .as_str()
            .parse()?;
        //println!("Registered module {} for day num '{}'", path, day);
        $registry.insert(
            day,
            Task {
                answer: $module::ANSWER,
                process_a: Box::new($module::process_a),
                process_b: Box::new($module::process_b),
            },
        );
    };
}

/// Macro to register all days
macro_rules! days {
    ($registry: ident, ($( $daymod: ident ), +)) => {
        $(
            day!($registry, $daymod);
        )+
    };
}

/// Collect all days from crate
fn init_registry() -> anyhow::Result<Registry> {
    let mut registry: Registry = HashMap::new();
    days!(registry, (day1));
    Ok(registry)
}

fn main() -> anyhow::Result<()> {
    let day: usize = env::args().nth(1).expect("provide day").parse()?;
    println!("{}", format!("Run day {}", day).blue());
    let lines: Vec<String> = match env::args().nth(3) {
        Some(filename) => read_lines_from_file(filename)?,
        None => {
            if !atty::is(Stream::Stdin) {
                read_lines_stdin()?
            } else {
                read_lines_from_file(format!("data/day{}.dat", day))?
            }
        }
    };
    let task_type: TaskType = match env::args()
        .nth(2)
        .context("Expected a or b as second argument")?
        .as_str()
    {
        "a" => Ok(TaskType::A),
        "b" => Ok(TaskType::B),
        _ => Err(anyhow!("wrong task type")),
    }?;
    let registry = init_registry()?;
    let task: &Task = registry.get(&day).context("No such day")?;
    let answer: String = match task_type {
        TaskType::A => (task.process_a)(lines),
        TaskType::B => (task.process_b)(lines),
    };
    let expected_answer: &str = match task_type {
        TaskType::A => task.answer.a,
        TaskType::B => task.answer.b,
    };
    check_answer(answer, expected_answer)
}
