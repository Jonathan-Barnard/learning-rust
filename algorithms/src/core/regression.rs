use log::debug;


pub struct Fit {
    pub slope: f64,
    pub intercept: f64
}

pub struct LinearRegression {
    pub iters: usize,
    pub alpha: f64
}

impl Default for LinearRegression {
    fn default() -> Self {
        Self { iters: 10_000, alpha: 0.01 }
    }
}

impl LinearRegression {
    pub fn fit(&self, x: &[f64], y: &[f64]) -> Result<Fit, String> {
        if x.is_empty() || y.is_empty() {
            return Err(format!("Data is empty: x = {}, y = {}", x.len(), y.len()));
        }
        if x.len() != y.len() {
            return Err(format!(
                "x shape does not match y: x = {}, y = {}",
                x.len(),
                y.len()
            ));
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
                let err = m * a + c - b;
                sq_err += err * err;

                // m
                grad_sum += a * err;
                // c
                grad_bias_sum += err
            }

            let loss = sq_err * inv_n;
            let grad = grad_sum * 2. * inv_n;
            let bias = grad_bias_sum * 2. * inv_n;

            m -= alpha * grad;
            c -= alpha * bias;

            if i % 1000 == 0 {
                debug!("iter = {}", i);
                debug!("loss = {}", loss);
                debug!("grad = {}", grad);
                debug!("m = {}", m);
                debug!("c = {}", c);
            }
        }

        let fit = Fit{slope: m, intercept: c};
        Ok(fit)
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn errors_on_empty_input() {
        let lr = LinearRegression::default();
        assert!(lr.fit(&[], &[]).is_err());
    }

    #[test]
    fn errors_on_shape_mismatch() {
        let lr = LinearRegression::default();
        assert!(lr.fit(&[1., 2.], &[5.]).is_err());
    }

    #[test]
    fn fits_approximately() {
        let lr = LinearRegression::default();
        let x = [0., 1., 2., 3.];
        let y = [1.1, 2.9, 5.2, 6.8];
        let fit = lr.fit(&x, &y).unwrap();

        assert!((fit.slope - 2.).abs() < 0.1);
        assert!((fit.intercept - 1.).abs() < 0.1);
    }
}
