enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x}, and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }
    }

    let msg = Message::Move { x: 3, y: 4 };

    if let Message::Move { x, y } = msg {
        println!("Move in the x direction {x}, and in the y direction {y}");
    } else {
        println!("Not a move message");
    }

    let msg = Message::Write(String::from("Hello"));

    if let Message::Write(text) = msg {
        println!("Text message: {text}");
    } else {
        println!("Not a write message");
    }

    let msg = Message::ChangeColor(0, 160, 255);
    
    let msg = Message::ChangeColor(0, 160, 255);

    if let Message::ChangeColor(r, g, b) = msg {
        println!("Change the color to red {r}, green {g}, and blue {b}");
    } else {
        println!("Not a change color message");
    }
}