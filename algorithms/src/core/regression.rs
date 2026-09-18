use log::{debug, info};
use ndarray::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataError {
    Empty,
    ShapeMismatch { x: usize, y: usize },
}

impl std::fmt::Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "Input data is empty"),
            Self::ShapeMismatch { x, y } => write!(f, "Length mismatch: x = {x}, y = {y}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RegressorND {
    pub slope: Array1<f64>,
    pub intercept: Array0<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Regressor {
    pub slope: f64,
    pub intercept: f64,
}

impl Regressor {
    #[must_use]
    pub fn predict(&self, x: f64) -> f64 {
        self.slope * x + self.intercept
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearRegression {
    pub iters: usize,
    pub alpha: f64,
}

impl Default for LinearRegression {
    fn default() -> Self {
        Self {
            iters: 10_000,
            alpha: 0.01,
        }
    }
}

impl LinearRegression {
    pub fn fit_ndim(
        &self,
        x: ArrayView2<f64>,
        y: ArrayView1<f64>,
    ) -> Result<RegressorND, DataError> {
        if x.is_empty() {
            return Err(DataError::Empty);
        }

        let mut slope = Array1::<f64>::zeros(x.ncols());
        let mut intercept = arr0(0.);
        let inv = 2. / (x.shape()[0] as f64);

        println!("{:?}", x);

        for _ in 0..self.iters {
            let err = x.dot(&slope) + &intercept - &y;
            let grad_w = x.t().dot(&err) * inv;
            let grad_b = err.sum_axis(Axis(0)) * inv;

            slope.scaled_add(-self.alpha, &grad_w);
            intercept.scaled_add(-self.alpha, &grad_b);
        }

        let fit = RegressorND { slope, intercept };

        Ok(fit)
    }

    pub fn fit(&self, x: &[f64], y: &[f64]) -> Result<Regressor, DataError> {
        if x.is_empty() || y.is_empty() {
            return Err(DataError::Empty);
        }
        if x.len() != y.len() {
            return Err(DataError::ShapeMismatch {
                x: x.len(),
                y: y.len(),
            });
        }
        let n = x.len() as f64;
        let inv_n = 1. / n;

        let mut c = 0.;
        let mut m = 1.;

        for i in 0..self.iters {
            let mut sq_err = 0.;
            let mut grad_sum = 0.;
            let mut grad_bias_sum = 0.;
            for (a, b) in x.iter().zip(y) {
                let err = m * a + c - b;
                sq_err += err * err;

                // m
                grad_sum += a * err;
                // c
                grad_bias_sum += err;
            }

            let loss = sq_err * inv_n;
            let grad = grad_sum * 2. * inv_n;
            let bias = grad_bias_sum * 2. * inv_n;

            m -= self.alpha * grad;
            c -= self.alpha * bias;

            if i % 1000 == 0 {
                debug!("iter = {}", i);
                debug!("loss = {}", loss);
                debug!("grad = {}", grad);
                debug!("m = {}", m);
                debug!("c = {}", c);
            }
        }

        let fit = Regressor {
            slope: m,
            intercept: c,
        };
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

    #[test]
    fn predicts_approximately() {
        let lr = LinearRegression::default();
        let x = [0., 1., 2., 3.];
        let y = [1., 2., 3., 4.];
        let fit = lr.fit(&x, &y).unwrap();

        let a = fit.predict(1.);
        assert!((a - 2.).abs() < 0.01);
    }
}
