#[macro_use]
extern crate nom;
extern crate term_size;
extern crate rustyline;
extern crate isatty;

use isatty::{stdin_isatty};

use rustyline::error::ReadlineError;
use rustyline::Editor;

use nom::{
    branch::alt,
    character::{is_alphabetic, is_digit},
    IResult,
};
use rust_black_trees::tree::BaseTree;
use rust_black_trees::tree::Tree;
use rust_black_trees::{avltree::AVLTree, rbtree::RBTree, unbalancetree::BSTree};

#[derive(Debug)]
enum Cmd {
    Add(isize),
    Delete(isize),
    Print,
    Quit,
    Clear,
    Help,
    New(TreeSelection),
    NumberError,
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
        let x = std::str::from_utf8(b).unwrap().parse();
        if let Ok(val) = x {
            (s, Cmd::Delete(val))
        } else {
            (s, Cmd::NumberError)
        }
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
        let x = std::str::from_utf8(b).unwrap().parse();
        if let Ok(val) = x {
            (s, Cmd::Add(val))
        } else {
            (s, Cmd::NumberError)
        }
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
        "rb" => Ok((s, Cmd::New(TreeSelection::RedBlack))),
        "avl" => Ok((s, Cmd::New(TreeSelection::AVL))),
        "bst" => Ok((s, Cmd::New(TreeSelection::BST))),
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

fn eval(
    cmd: Cmd,
    rb: &mut RBTree<isize>,
    avl: &mut AVLTree<isize>,
    bs: &mut BSTree<isize>,
    tree_type: &mut TreeSelection,
) {
    match cmd {
        Cmd::Quit => {
            std::process::exit(0);
        }
        Cmd::Clear => {
            // print!("\x1B[2J"); // Clear should clear the tree not the screen
            *rb = RBTree::new();
            *avl = AVLTree::new();
            *bs = BSTree::new();
        }
        Cmd::Print => match tree_type {
            TreeSelection::RedBlack => {
                if let Some(root) = rb.get_root() {
                    if let Some(s) = rust_black_trees::prettynodeprinter::printprettyrb(
                        rb.get(root),
                    ) {
                        println!("{}", s)
                    } else {
                        println!("{}", rb.to_pretty_string())
                    }
                } else {
                    println!("Empty Red Black Tree")
                }
            }
            TreeSelection::AVL => {
                if let Some(root) = avl.get_root() {
                    if let Some(s) = rust_black_trees::prettynodeprinter::printprettyavl(
                        avl.get(root),
                    ) {
                        println!("{}", s)
                    } else {
                        println!("{}", avl.to_pretty_string())
                    }
                } else {
                    println!("Empty AVL Tree")
                }
            }
            TreeSelection::BST => {
                if let Some(root) = bs.get_root() {
                    if let Some(s) = rust_black_trees::prettynodeprinter::printprettybst(
                        bs.get(root),
                    ) {
                        println!("{}", s)
                    } else {
                        println!("{}", avl.to_pretty_string())
                    }
                } else {
                    println!("Empty Binary Search Tree")
                }
            }
            TreeSelection::Undefined => eprintln!("Need to create a tree first!"),
        },
        Cmd::Add(v) => match tree_type {
            TreeSelection::RedBlack => rb.insert(v),
            TreeSelection::AVL => avl.insert(v),
            TreeSelection::BST => bs.insert(v),
            TreeSelection::Undefined => eprintln!("Need to create a tree first!"),
        },
        Cmd::Delete(v) => match tree_type {
            TreeSelection::RedBlack => {
                rb.delete(v);
            }
            TreeSelection::AVL => {
                avl.delete(v);
            }
            TreeSelection::BST => {
                bs.delete(v);
            }
            TreeSelection::Undefined => eprintln!("Need to create a tree first!"),
        },
        Cmd::Help => {
            println!("Commands:");
            println!("  new [avl | rb | bst]");
            println!("  add [VAL]");
            println!("  delete [VAL]");
            println!("  print");
            println!("  clear");
            println!("  quit");
        }
        Cmd::New(v) => {
            *tree_type = v;
            *rb = RBTree::new();
            *avl = AVLTree::new();
            *bs = BSTree::new();
        }
        Cmd::NumberError => {
            println!("Only unsigned word sized numbers are supported in the demo. Other datatypes can be purchased for $5.99.");
        }
    }
}

fn read_and_eval(
    rl: &mut Editor<()>,
    mut rb: &mut RBTree<isize>,
    mut avl: &mut AVLTree<isize>,
    mut bs: &mut BSTree<isize>,
    tree_type: &mut TreeSelection,
) {
    let readline = rl.readline("> ");
    match readline {
        Ok(line) => {
            rl.add_history_entry(line.as_str());
            let ss = line.as_str().to_string() + " ";
            let res = command(ss.as_bytes());
            if let Ok((_s, cmd)) = res {
                eval(cmd, &mut rb, &mut avl, &mut bs, tree_type);
            } else {
                println!("Invalid Command. Try: help")
            }
        },
        Err(ReadlineError::Interrupted) => {
            println!();
            std::process::exit(0);
        },
        Err(ReadlineError::Eof) => {
            println!();
            std::process::exit(0);
        },
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }

}

#[derive(Debug)]
enum TreeSelection {
    RedBlack,
    AVL,
    BST,
    Undefined,
}
fn main() {
    if stdin_isatty() {
        println!("Tree Editor CLI v1.0.0");
    }

    let mut rbtree = RBTree::new();
    let mut avltree = AVLTree::new();
    let mut bstree = BSTree::new();
    let mut tree_type = TreeSelection::Undefined;
    let mut rl = Editor::<()>::new();
    loop {
        read_and_eval(&mut rl, &mut rbtree, &mut avltree, &mut bstree, &mut tree_type);
    }
}
