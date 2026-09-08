use algorithms::core::LinearRegression;

fn main() {
    use env_logger::Env;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let x = vec![1., 2., 3.2];
    let y = vec![5., 10., 16.];

    let fit = match LinearRegression::default().fit(&x, &y) {
        Ok(fit) => fit,
        Err(e) => {
            eprintln!("error: {e}");
            return;
        }
    };
    println!("fitted: m = {}, c = {}", fit.slope, fit.intercept);
}
