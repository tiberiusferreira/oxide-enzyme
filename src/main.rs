extern {
    fn __enzyme_autodiff(_: usize, ...);
}

/// Yep, magic numbers, we look for those number in the output llvm-ir and replace them with metadata
const ENZYME_CONST: i128 = 1321523312;
const ENZYME_DUP: i128 = 314210384213;


/// Dummy implementation
fn linear_layer(tensor: &Tensor, weights: &Tensor) -> Tensor{
    multiply(tensor, weights)
}

/// Dummy implementation
fn reduce_sum_layer(tensor: &Tensor) -> f64{
    tensor.data.iter().sum()
}

/// Dummy implementation
fn softmax(tensor: &Tensor) -> Tensor{
    let sum_exp_all: f64 = tensor.data.iter().fold(0., |acc, d|{
        acc + d.exp()
    });
    let mut out: Vec<f64> = vec![0.; tensor.data.len()];
    for i in 0..tensor.data.len(){
        out[i] = tensor.data[i].exp()/sum_exp_all;
    }
    // TODO -> crashes
    // let exp_all: Vec<f64> = tensor.data.iter().map(|d|{
    //     d.exp()/sum_exp_all
    // }).collect();
    Tensor{
        data: out,
        shape: tensor.shape.clone()
    }
}

/// Dummy implementation
/// probdist = [0.3, 0.7]
/// label = [1, 0]
/// - (1*log(0.3) + 0*log(0.7))
fn cross_entropy(input: &Tensor, target: &Tensor) -> f64{
    let mut out = 0.;
    for i in 0..input.data.len(){
        out = out - target.data[i]*input.data[i].log(10.);
    }
    out
}


/// Dummy implementation
fn dummy_nn_with_loss(input: *mut f64, input_len: usize, linear_weights: *mut f64, linear_weights_len: usize,
                      target: *mut f64, target_len: usize, net_output: *mut f64, net_output_len: usize) -> f64{
    let out = dummy_nn(input, input_len, linear_weights, linear_weights_len);
    let target_as_slice = unsafe{
        std::slice::from_raw_parts_mut(target, target_len as usize)
    };
    let net_output_as_slice = unsafe{
        std::slice::from_raw_parts_mut(net_output, net_output_len as usize)
    };
    for i in 0..target_as_slice.len(){
        net_output_as_slice[i] = out.data[i];
    }
    // TODO
    // input_as_slice.to_vec() crashes!
    let mut target_vec = vec![0.; target_as_slice.len()];
    for i in 0..target_as_slice.len(){
        target_vec[i] = target_as_slice[i];
    }
    let target_tensor = Tensor{
        data: target_vec,
        shape: [2, 2]
    };

    // println!("{:?}", soft_out); // todo this crashes!
    cross_entropy(&out, &target_tensor)
}


/// Dummy implementation
fn dummy_nn(input: *mut f64, input_len: usize, linear_weights: *mut f64, linear_weights_len: usize) -> Tensor{

    let input_as_slice = unsafe{
        std::slice::from_raw_parts_mut(input, input_len as usize)
    };
    let weights_as_slice = unsafe{
        std::slice::from_raw_parts_mut(linear_weights, linear_weights_len as usize)
    };
    // TODO
    // input_as_slice.to_vec() crashes!
    let mut input_vec = vec![0.; input_as_slice.len()];
    for i in 0..input_as_slice.len(){
        input_vec[i] = input_as_slice[i];
    }
    let mut weight_vec = vec![0.; weights_as_slice.len()];
    for i in 0..weights_as_slice.len(){
        weight_vec[i] = weights_as_slice[i];
    }
    let input_tensor = Tensor{
        data: input_vec,
        shape: [2, 2]
    };
    let weights_tensor = Tensor{
        data: weight_vec,
        shape: [2, 2]
    };
    let linear_out = linear_layer(&input_tensor, &weights_tensor);
    softmax(&linear_out)
}
#[derive(Debug, Clone)]
struct Tensor{
    data: Vec<f64>,
    shape: [usize; 2]
}

/// Multiplies Square 2D Tensors dummy implementation
fn multiply(left: &Tensor, right: &Tensor) -> Tensor
{
    let mut res : Vec<f64> = vec![0.; (left.shape[0]*left.shape[1]) as usize];
    for i in 0..left.shape[0] {
        for j in 0..left.shape[0] {
            res[i*left.shape[0] + j] = 0.;
            for k in 0..left.shape[0]{
                res[i*left.shape[0] + j] += left.data[i*left.shape[0] + k] * right.data[k*left.shape[0] + j];
            }
        }
    }
    Tensor{
        data: res,
        shape: [2, 2]
    }
}


fn main() {
    let mut input = vec![1., 2., 3., 4.];
    let mut weights = vec![1.; 4];
    let mut weights_shadow = vec![0.; 4];
    let mut target = vec![0., 0., 1., 0.];
    let mut net_output = vec![0., 0., 1., 0.];
    for _i in 0..70{
        let res = dummy_nn_with_loss(input.as_mut_ptr(),
                                     input.len(),
                                     weights.as_mut_ptr(),
                                     weights.len(),
                                     target.as_mut_ptr(),
                                     target.len(),
                                     net_output.as_mut_ptr(),
                                     net_output.len());
        println!("Loss: {:?}", res);
        println!("Net output: {:?}", net_output);
        unsafe {
            __enzyme_autodiff(dummy_nn_with_loss as usize,
                              ENZYME_CONST,
                              input.as_mut_ptr(),
                              ENZYME_CONST, input.len(),
                              ENZYME_DUP, weights.as_mut_ptr(), weights_shadow.as_mut_ptr(),
                              ENZYME_CONST, weights.len(),
                              ENZYME_CONST, target.as_mut_ptr(),
                              ENZYME_CONST, target.len(),
                              ENZYME_CONST, net_output.as_mut_ptr(),
                              ENZYME_CONST, net_output.len());
        }
        weights.iter_mut().zip(weights_shadow.iter()).for_each(|(weight, grad)|{
            *weight = *weight - *grad*0.01;
        });
    }
}