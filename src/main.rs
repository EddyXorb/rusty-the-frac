mod complex;

fn main() {
    let a = complex::Cx::new(1, 1);
    let b = complex::Cx::new(2, 2);
    let c = a * b;

    println!("{}", c);
}
