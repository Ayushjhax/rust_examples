struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Color(r, g, b) = black;
    println!("r: {r}, g: {g}, b: {b}");

    let Point(x, y, z) = origin;
    println!("x: {x}, y: {y}, z: {z}");
}