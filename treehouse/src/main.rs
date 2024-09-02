use std::{io::stdin, thread, time};

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNone { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }
    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the treehouse.{}", self.name),
            VisitorAction::AcceptWithNone { note } => {
                println!("Welcome to the treehouse,{}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} if now a probationary member.", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
    }
}
fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Faild to read line!");
    your_name.trim().to_lowercase()
}
fn main() {
    let mut visitor_list = vec![
        Visitor::new("bert", VisitorAction::Accept, 45),
        Visitor::new(
            "steve",
            VisitorAction::AcceptWithNone {
                note: "Lactose-free milk is in the fridge.".to_string(),
            },
            15,
        ),
        Visitor::new("fred", VisitorAction::Refuse, 30),
    ];
    visitor_list.push(Visitor::new("lzc", VisitorAction::Probation, 20));

    loop {
        println!("Hello, what's your name!");
        let name = what_is_your_name();

        let known_visitor: Option<&Visitor> =
            visitor_list.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    println!("The final list of visitor:");
                    println!("{:#?}", visitor_list);
                    break;
                } else {
                    println!("{} is not on the visitor list.", name);
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 20));
                }
            }
        }
        println!(" ");
        thread::sleep(time::Duration::from_millis(200));
    }
}
