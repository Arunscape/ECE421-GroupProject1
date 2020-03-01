#[macro_use]
extern crate nom;
use rust_black_trees::test;

use std::io::{self, BufRead, Write};

use nom::{branch::alt, sequence::separated_pair, character::is_digit, IResult};

#[derive(Debug)]
enum Cmd {
    Add(isize),
    Delete(isize),
    Print,
    Quit,
    Clear,
    Help,
}

// fn delete(input: &str) -> IResult<&str, Cmd> {
//     separated_pair(
//         alt!(tag!("del") | tag!("delete") | tag!("sub") | tag!("rm")),
//         char!(' '),
//         take_while!(is_digit),
//     )(input).map(|(a, b)| (a, Cmd::Quit))
// }

fn add(input: &[u8]) -> IResult<&[u8], Cmd> {
    named!(
        addnameparser,
        alt!(tag!("add") | tag!("insert") | tag!("i") | tag!("a"))
    );
    named!( addparser( &[u8] ) -> (&[u8], &[u8]),
            separated_pair!(
                addnameparser,
                char!(' '),
                take_while!(is_digit)
            )
    );
    addparser(input).map(|(s, (_a, b))| (s, Cmd::Add(3)))
}

fn help(input: &[u8]) -> IResult<&[u8], Cmd> {
    named!(quitparse, alt!(tag!("help") | tag!("h")));
    let x = quitparse(input);
    x.map(|(a, b)| (a, Cmd::Help))
}

fn quit(input: &[u8]) -> IResult<&[u8], Cmd> {
    named!(quitparse, alt!(tag!("quit") | tag!("exit")));
    let x = quitparse(input);
    x.map(|(a, b)| (a, Cmd::Quit))
}

fn clear(input: &[u8]) -> IResult<&[u8], Cmd> {
    named!(clearparse, alt!(tag!("clear") | tag!("clr")));
    let x = clearparse(input);
    x.map(|(a, b)| (a, Cmd::Clear))
}

fn print(input: &[u8]) -> IResult<&[u8], Cmd> {
    named!(printparse, alt!(tag!("print") | tag!("p")));
    let x = printparse(input);
    x.map(|(a, b)| (a, Cmd::Print))
}

fn command(input: &[u8]) -> IResult<&[u8], Cmd> {
    //named!(commandparse, alt!(quit | print | clear) );
    //let x = commandparse(input);
    let x = alt((quit, print, clear, help))(input);
    x
}

fn read_line() -> String {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line
}

fn eval(cmd: Cmd) {
    std::dbg!(&cmd);
    match cmd {
        Cmd::Quit => {
            std::process::exit(0);
        }
        Cmd::Clear => println!("Clear"),
        Cmd::Print => println!("Print"),
        Cmd::Add(v) => println!("Add: {}", v),
        Cmd::Delete(v) => println!("Del: {}", v),
        Cmd::Help => {
            println!("Commands:");
            println!("  add [VAL]");
            println!("  delete [VAL]");
            println!("  print");
            println!("  clear");
            println!("  quit");
        }
    }
}

fn read_and_eval() {
    print!("> ");
    io::stdout().flush().unwrap();

    let s = read_line();
    if s == String::from("") {
        // user hit Ctrl-D
        println!();
        std::process::exit(0);
    }
    let res = command(s.as_bytes());

    if let Ok((_s, cmd)) = res {
        eval(cmd);
    } else {
        println!("Invalid Command. Try: help")
    }
}

fn main() {
    println!("Tree Editor CLI v1.0.0");

    loop {
        read_and_eval();
    }
    // test();
}
