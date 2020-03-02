mod benchmark;
#[macro_use]
extern crate nom;
use rust_black_trees::test;

use std::io::{self, BufRead, Write};

use nom::{
    branch::alt,
    character::{is_alphabetic, is_digit},
    IResult,
};

#[derive(Debug)]
enum Cmd {
    Add(isize),
    Delete(isize),
    Print,
    Quit,
    Clear,
    Help,
    New(bool),
}

fn delete(input: &[u8]) -> IResult<&[u8], Cmd> {
    named!(
        delnameparser,
        alt!(tag!("delete") | tag!("del") | tag!("remove") | tag!("d") | tag!("r"))
    );
    named!( delparser( &[u8] ) -> (&[u8], &[u8]),
            separated_pair!(
                delnameparser,
                char!(' '),
                take_while!(is_digit)
            )
    );
    delparser(input).map(|(s, (_a, b))| {
        (
            s,
            Cmd::Delete(std::str::from_utf8(b).unwrap().parse().unwrap()),
        )
    })
}

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
    addparser(input).map(|(s, (_a, b))| {
        (
            s,
            Cmd::Add(std::str::from_utf8(b).unwrap().parse().unwrap()),
        )
    })
}

fn new(input: &[u8]) -> IResult<&[u8], Cmd> {
    named!(
        newnameparser,
        alt!(tag!("new") | tag!("create") | tag!("n") | tag!("c"))
    );
    named!( newparser( &[u8] ) -> (&[u8], &[u8]),
            separated_pair!(
                newnameparser,
                char!(' '),
                take_while!(is_alphabetic)
            )
    );
    newparser(input).and_then(|(s, (_a, b))| match std::str::from_utf8(b).unwrap() {
        "rb" => Ok((s, Cmd::New(false))),
        "avl" => Ok((s, Cmd::New(true))),
        _ => Err(nom::Err::Failure((s, nom::error::ErrorKind::NoneOf))),
    })
}

fn help(input: &[u8]) -> IResult<&[u8], Cmd> {
    named!(quitparse, alt!(tag!("help") | tag!("h")));
    let x = quitparse(input);
    x.map(|(a, _b)| (a, Cmd::Help))
}

fn quit(input: &[u8]) -> IResult<&[u8], Cmd> {
    named!(quitparse, alt!(tag!("quit") | tag!("exit")));
    let x = quitparse(input);
    x.map(|(a, _b)| (a, Cmd::Quit))
}

fn clear(input: &[u8]) -> IResult<&[u8], Cmd> {
    named!(clearparse, alt!(tag!("clear") | tag!("clr")));
    let x = clearparse(input);
    x.map(|(a, _b)| (a, Cmd::Clear))
}

fn print(input: &[u8]) -> IResult<&[u8], Cmd> {
    named!(printparse, alt!(tag!("print") | tag!("p")));
    let x = printparse(input);
    x.map(|(a, _b)| (a, Cmd::Print))
}

fn command(input: &[u8]) -> IResult<&[u8], Cmd> {
    //named!(commandparse, alt!(quit | print | clear) );
    //let x = commandparse(input);
    let x = alt((quit, print, clear, help, add, delete, new))(input);
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
        Cmd::New(v) => println!("New: {}", v),
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
