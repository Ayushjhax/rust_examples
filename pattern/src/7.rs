struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let origin = Point { x: 0, y: 0, z: 0 };
    let number = (2,3,4,5,6);
    let (a,b,c,d,e) = number;
    let num = Some(4);
    let x = Some(5);
    let y = 10;


    match origin {
        Point { x, .. } => println!("x is {x}"),

    }

    match number {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    match number {
        (a,b,c,d,e) => {
            println!("Some numbers: {a}, {b}, {c}, {d}, {e}");
        }
    }

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("x = {:?}", x);
    println!("y = {y}");
}