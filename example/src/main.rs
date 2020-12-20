fn main() {
    let result = simple_mul(&mut 2., &mut 3.);
    println!("Hello, world! {}", result);
}

fn simple_mul(left: &mut f32, right: &mut f32) -> f32 {
    *left * *right
}
