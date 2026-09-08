use log::{debug, error, info};
use algorithms::core::regression;
use ndarray::{prelude::*, range};


fn linear_regression(x: &[f64], y: &[f64]) -> Result<(f64, f64), String> {
    if x.is_empty() || y.is_empty() {
        return Err(format!("Data is empty: x = {}, y = {}", x.len(), y.len()))
    }
    if x.len() != y.len() {
        return Err(format!("x shape does not match y: x = {}, y = {}", x.len(), y.len()))
    }
    let n = x.len() as f64;
    let inv_n = 1. / n;

    let mut c = 0.;
    let mut m = 1.;

    let iters = 10_000;
    let alpha = 0.01;

    for i in 0..iters {
        
        let mut sq_err = 0.;
        let mut grad_sum = 0.;
        let mut grad_bias_sum = 0.;
        for (a, b) in x.iter().zip(y.iter()) {
            let err = m*a +c - b;
            sq_err += err*err;

            // m
            grad_sum += a*err;
            // c
            grad_bias_sum += err
        }

        let loss = sq_err * inv_n;
        let grad = grad_sum * 2. * inv_n;
        let bias = grad_bias_sum * 2. * inv_n;

        m -= alpha*grad;
        c -= alpha*bias;

        if i % 1000 == 0 {
            debug!("iter = {}", i);
            debug!("loss = {}", loss);
            debug!("grad = {}", grad);
            debug!("m = {}", m);
            debug!("c = {}", c);
        }
    }

    Ok((m, c))
}

fn main () {
    use env_logger::Env;
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let x = vec![1., 2., 3.2];
    let y = vec![5., 10., 16.];

    let (m, c) = match linear_regression(&x, &y) {
        Ok((m, c)) =>  (m, c),
        Err(e) => {
            eprintln!("error: {e}");
            return;
        }
    };
    println!("filled: m = {m}, c = {c}");


}