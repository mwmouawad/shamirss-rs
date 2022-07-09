use shamirss_rs::math::polynomial::Polynomial;

fn main() {
    println!("Hello, world!");
    let testPoly = Polynomial::build(3, 164.0, &[48.0, 52.0, -20.0]);
    println!("Polynomial is: {:?}", testPoly);
    println!("Polynomial return value is: {}", testPoly.apply(-15.0));
}
