# Flax_and_teal_assignment
**Q.1**

This code implements the fourth-order Runge-Kutta (RK4) method to solve an ordinary differential equation (ODE) defined by the function `f(t, y) = 1.0 - t.powi(2) + y`. The ODE is solved over the interval [0, 2] with a step size of 0.2.

**Usage**

To run the code, one can do so by following the instructions on the official Rust website: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021.

**Code Explanation**

The `f(t, y)` function represents the differential equation `dy/dt = 1 - t^2 + y, 0≤t≤ 2, y(0) = 0.5, with n = 10.

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

**Q.2**
# Burger's Equation Solver

This Python script solves Burger's equation using the fourth-order Runge-Kutta (RK4) method with central differences for approximating derivatives.

## Requirements
- Python 3.x
- NumPy
- Matplotlib

## Description
- Burger's equation is a fundamental partial differential equation used in fluid dynamics and nonlinear physics. It describes the behavior of a viscous fluid undergoing convection and diffusion.
- The equation to be solved is: ∂u/∂t + u ∂u/∂x = ν ∂^2u/∂x^2
- The initial condition for u(x,0) is set as sin(x*pi/L), where 0 <= x <= 2*pi and t > 0.
- The spatial domain spans from 0 to 2*pi and is discretized into Nx grid points.
- The time domain spans from 0 to T and is discretized into Nt time steps.
- The script uses the RK4 method to numerically solve the equation.
- The solution is plotted at regular time intervals and printed to the console at specified time steps.

## Parameters
- L: Length of the spatial domain
- Nx: Number of spatial grid points
- Nt: Number of time steps
- nu: Viscosity coefficient
- T: Total simulation time

## Output
- The script produces a plot showing the numerical solution of Burger's equation over time.

**License**

This code is provided under the MIT License. Feel free to use and modify it according to your needs.
