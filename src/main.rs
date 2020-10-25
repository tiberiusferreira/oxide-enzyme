extern {
    fn __enzyme_autodiff(_: usize, ...);
}
use ndarray::Array2;
fn vec_op_test_raw(left_ptr: *mut f64, left_len: i64, right_ptr: *mut f64, right_len: i64) -> f64{
    let mut a_slice = unsafe{
        std::slice::from_raw_parts_mut(left_ptr, left_len as usize)
    };
    let mut b_slice = unsafe{
        std::slice::from_raw_parts_mut(right_ptr, right_len as usize)
    };
    let arr_a = unsafe {
        // unchecked to prevent exception code from being generated
        ndarray::Array2::from_shape_vec_unchecked([2, 2], a_slice.to_vec())
    };
    let arr_b = unsafe {
        // unchecked to prevent exception code from being generated
        ndarray::Array2::from_shape_vec_unchecked([2, 2], b_slice.to_vec())
    };
    let c = arr_a.dot(&arr_b);
    let res = c.iter().sum();
    res
}

fn main() {

    let mut a = vec![1., 2., 3., 4.];
    let mut a_shadow = vec![0., 0., 0., 0.];
    let mut b = vec![1., 2., 3., 4.];
    let mut b_shadow = vec![0., 0., 0., 0.];
    println!("{:?}", vec_op_test_raw(a.as_mut_ptr(), a.len() as i64,
                                     b.as_mut_ptr(), b.len() as i64));

    unsafe {
        __enzyme_autodiff(vec_op_test_raw as usize,
                          a.as_mut_ptr(), a_shadow.as_mut_ptr(), a_shadow.len() as i64,
                          b.as_mut_ptr(), b_shadow.as_mut_ptr(), b_shadow.len() as i64);
    }
    dbg!(a);
    dbg!(a_shadow);
}