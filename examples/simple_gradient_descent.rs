//! This example demonstrates gradient descent learning. 
//! 
//! In this example we learn 2 parameters `a` and `b` in the equation `y_pred = (x + a)*b` from examples of pairs of `x` and `y = 2*x + 1`.
//! 
//! This lecture explains backpropagation quite well https://www.youtube.com/watch?v=dB-u77Y5a6A.

use rand::{rngs::StdRng, Rng, SeedableRng};

fn main() {
    let mut rng = StdRng::from_seed([0u8; 32]);

    let learning_rate = 0.01;

    // Construct the computation graph. 
    let mut graph = sq_diff(
        mul(
            add(
                // input
                0.0,
                // parameter
                5.0,
            ),
            // parameter
            -4.0,
        ),
        // output
        0.0,
    );

    for step in 0..100 {
        // Generate the input value. Using a range larger than -1..1 so that the effect of the linear component most
        // often weighs more heavily than the constant component.
        let x = rng.gen_range(-10.0..10.0);
        
        // Compute the expected output.
        let y = 2.0 * x + 1.0;

        // Set the input and output in the computation graph.
        graph.a.a.0 = x;
        graph.b = y;

        let loss = graph.forward();

        println!(
            "step: {step:3}, add.b: {add_b:7.3}, mul.b: {mul_b:7.3}, loss: {loss:8.3}",
            add_b = graph.a.a.1,
            mul_b = graph.a.b
        );

        // Pre-multiply dloss/dloss (1.0) with learning rate.
        graph.backward(learning_rate);
    }
}

trait Expression {
    fn forward(&mut self) -> f32;

    fn backward(&mut self, upstream: f32);
}

#[derive(Debug)]
struct Mul<A, B> {
    a: A,
    b: B,
    cached_a: f32,
    cached_b: f32,
}

impl<A, B> Expression for Mul<A, B>
where
    A: Expression,
    B: Expression,
{
    fn forward(&mut self) -> f32 {
        self.cached_a = self.a.forward();
        self.cached_b = self.b.forward();
        self.cached_a * self.cached_b
    }

    fn backward(&mut self, upstream: f32) {
        self.a.backward(upstream * self.cached_b);
        self.b.backward(upstream * self.cached_a);
    }
}

#[derive(Debug)]
struct Add<A, B>(A, B);

impl<A, B> Expression for Add<A, B>
where
    A: Expression,
    B: Expression,
{
    fn forward(&mut self) -> f32 {
        self.0.forward() + self.1.forward()
    }

    fn backward(&mut self, upstream: f32) {
        self.0.backward(upstream);
        self.1.backward(upstream);
    }
}

impl Expression for f32 {
    fn forward(&mut self) -> f32 {
        *self
    }

    fn backward(&mut self, upstream: f32) {
        *self -= upstream
    }
}

#[derive(Debug)]
struct SquaredDifference<A, B> {
    a: A,
    b: B,
    cache: f32,
}

impl<A, B> Expression for SquaredDifference<A, B>
where
    A: Expression,
    B: Expression,
{
    fn forward(&mut self) -> f32 {
        let a = self.a.forward();
        let b = self.b.forward();
        self.cache = a - b;
        self.cache.powi(2)
    }

    fn backward(&mut self, upstream: f32) {
        let grad = 2.0 * self.cache * upstream;
        self.a.backward(grad);
        self.b.backward(-grad);
    }
}

fn mul<A, B>(a: A, b: B) -> Mul<A, B> {
    Mul {
        a,
        b,
        cached_a: Default::default(),
        cached_b: Default::default(),
    }
}

fn add<A, B>(a: A, b: B) -> Add<A, B> {
    Add(a, b)
}

fn sq_diff<A, B>(a: A, b: B) -> SquaredDifference<A, B> {
    SquaredDifference {
        a,
        b,
        cache: Default::default(),
    }
}
