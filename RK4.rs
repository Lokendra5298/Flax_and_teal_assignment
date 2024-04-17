fn f(t: f64, y: f64) -> f64 {
    1.0 - t.powi(2) + y
}

fn main() {
    let h = 0.2;
    let t: Vec<f64> = (0..=(2.0 / h) as usize)
        .map(|i| i as f64 * h)
        .collect();
    let n = t.len();
    let mut y: Vec<f64> = vec![0.0; n];
    y[0] = 0.5;

    // Perform RK4 iterations
    for i in 0..(n - 1) {
        let k1 = h * f(t[i], y[i]);
        let k2 = h * f(t[i] + h / 2.0, y[i] + k1 / 2.0);
        let k3 = h * f(t[i] + h / 2.0, y[i] + k2 / 2.0);
        let k4 = h * f(t[i] + h, y[i] + k3);
        y[i + 1] = y[i] + (1.0 / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
        println!(
            "For Iteration: {} Value of t = {}, y(t) = {}",
            i,
            t[i],
            y[i + 1]
        );
    }

    // Print the final result
    println!(
        "For Iteration: {} Value of t = {}, y(t) = {}",
        n - 1,
        t[n - 1],
        y[n - 1]
    );
}

