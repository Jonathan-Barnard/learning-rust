use algorithms::core::LinearRegression;
use ndarray::prelude::*;

fn main() {
    use env_logger::Env;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let x = vec![1., 2., 3.2];
    let y = vec![5., 10., 16.];

    let regr = match LinearRegression::default().fit(&x, &y) {
        Ok(fit) => fit,
        Err(e) => {
            eprintln!("error: {e}");
            return;
        }
    };
    println!("fitted: m = {}, c = {}", regr.slope, regr.intercept);
    let a = regr.predict(2.);
    println!("{a}");

    let x = array![[1., 4.], [2., 7.], [3.2, 9.4]];
    let y = array![5., 10., 16.];

    let regr = LinearRegression::default().fit_ndim(x.view(), y.view());
    println!("{:?}", regr)
}
