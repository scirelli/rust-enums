#![allow(unused)]

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call_me(self) {
        match self {
            Message::Quit => println!("AHHH! Bye"),
            Message::Move {x, y} => println!("{x} {y}"),
            Message::Write(s) => println!("{s}"),
            Message::ChangeColor(r, g, b) => println!("{r} {g} {b}"),
        }
    }
}
fn main() {
    let w = Message::Write(String::from("Hello"));
    w.call_me();
    w.call_me();
}
