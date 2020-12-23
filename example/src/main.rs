use enzyme_proc_macro::{autodiff, autodiff_result};
fn main() {
    let result = simple_mul(&mut 2., &mut 3.);
    let mut left_grad = 0.;
    let mut right_grad = 0.;
    unsafe {
        simple_mul_diff(&mut 2., &mut left_grad, &mut 3., &mut right_grad, 1.);
    }
    dbg!(left_grad);
    dbg!(right_grad);
    println!("Hello, world! {}", result);
}

#[no_mangle]
#[allow(unused)]
#[autodiff]
fn simple_mul(left: &mut f32, right: &mut f32) -> f32 {
    *left * *right
}

extern "C" {
    #[autodiff_result(simple_mul)]
    fn simple_mul_diff(
        left: &mut f32,
        left_grad: &mut f32,
        right: &mut f32,
        right_grad: &mut f32,
        out_grad: f32,
    );
}
