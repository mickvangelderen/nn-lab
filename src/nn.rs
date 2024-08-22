pub trait ActivationFunction {
    fn activate(&self, value: f32) -> f32;
}

pub struct ReLU;

impl ActivationFunction for ReLU {
    fn activate(&self, value: f32) -> f32 {
        f32::max(0.0, value)
    }
}

pub struct FullyConnectedLayer<A = ReLU> {
    input_count: usize,
    output_count: usize,
    weights_and_biases: Vec<f32>,
    activation_function: A,
}

impl FullyConnectedLayer<ReLU> {
    pub fn new<I: IntoIterator<Item = f32>>(
        input_count: usize,
        output_count: usize,
        initializer: I,
    ) -> Self {
        Self {
            input_count,
            output_count,
            weights_and_biases: initializer
                .into_iter()
                .take((input_count + 1) * output_count)
                .collect(),
            activation_function: ReLU,
        }
    }
}

impl<A> FullyConnectedLayer<A>
where
    A: ActivationFunction,
{
    pub fn infer(&self, inputs: &[f32], outputs: &mut [f32]) {
        assert_eq!(self.input_count, inputs.len());
        assert_eq!(self.output_count, outputs.len());

        #[allow(clippy::needless_range_loop)]
        for output_idx in 0..self.output_count {
            let mut sum = 0.0;
            #[allow(clippy::needless_range_loop)]
            for input_idx in 0..self.input_count {
                sum += inputs[input_idx]
                    * self.weights_and_biases[(self.input_count + 1) * output_idx + input_idx];
            }

            let bias =
                self.weights_and_biases[(self.input_count + 1) * output_idx + self.input_count];

            outputs[output_idx] = self.activation_function.activate(sum + bias);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let layer =
            FullyConnectedLayer::new(2, 3, [11.0, 12.0, 13.0, 21.0, 22.0, 23.0, 31.0, 32.0, 33.0]);
        let inputs = [1.0, 2.0];
        let mut outputs = [0.0, 0.0, 0.0];
        layer.infer(&inputs, &mut outputs);
        assert_eq!(outputs, [48.0, 88.0, 128.0]);
    }
}
