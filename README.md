# Flax_and_teal_assignment
**Q.1**

This code implements the fourth-order Runge-Kutta (RK4) method to solve an ordinary differential equation (ODE) defined by the function `f(t, y) = 1.0 - t.powi(2) + y`. The ODE is solved over the interval [0, 2] with a step size of 0.2.

**Usage**

To run the code, you need to have Rust installed on your system. If you haven't installed Rust yet, you can do so by following the instructions on the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

Once Rust is installed, you can create a new Rust project and place the provided code in the `main.rs` file. Then, navigate to the project directory in your terminal and execute the following command to compile and run the code:

```bash
cargo run
```

This will compile the code and execute the resulting binary.

**Code Explanation**

The `f(t, y)` function represents the differential equation `dy/dt = 1 - t^2 + y`.

The `main` function initializes parameters such as the step size `h`, time vector `t`, and solution vector `y`. It then performs RK4 iterations to approximate the solution of the ODE. In each iteration, the RK4 method computes four intermediate slopes (`k1`, `k2`, `k3`, `k4`) and uses them to update the solution vector `y`. The final result is printed to the console.

**Example Output**

The program outputs the value of `y(t)` at each iteration and the final result. For example:

```
For Iteration: 0 Value of t = 0, y(t) = 0.5200000000000001
For Iteration: 1 Value of t = 0.2, y(t) = 0.5556666666666667
For Iteration: 2 Value of t = 0.4, y(t) = 0.6011481481481482
...
For Iteration: 9 Value of t = 1.8, y(t) = 1.0838980709302343
For Iteration: 10 Value of t = 2, y(t) = 1.1233788617581737
```

**License**

This code is provided under the MIT License. Feel free to use and modify it according to your needs.
