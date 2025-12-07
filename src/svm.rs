pub fn dual_objective(alpha: &[f64], y: &[i32], x: &[Vec<f64>]) -> f64 {
    let n = alpha.len();
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;

    for i in 0..n {
        sum1 += alpha[i];
    }

    for i in 0..n {
        for j in 0..n {
            let dot: f64 = x[i].iter().zip(&x[j]).map(|(a,b)| a*b).sum();
            sum2 += alpha[i] * alpha[j] * (y[i] as f64) * (y[j] as f64) * dot;
        }
    }

    sum1 - 0.5 * sum2
}
