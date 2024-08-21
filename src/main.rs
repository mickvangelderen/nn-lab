mod mnist;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;


fn main() -> Result<()> {
    let train_images = mnist::read_images_from_file("data/train-images-idx3-ubyte.gz")?;
    let train_labels = mnist::read_labels_from_file("data/train-labels-idx1-ubyte.gz")?;

    for i in 0..5 {
        dbg!(&train_images[i]);
        dbg!(&train_labels[i]);
    }

    Ok(())
}

struct FullyConnectedLayer {
    input_count: usize,
    output_count: usize,
    weights_and_biases: Vec<f32>,
}

impl FullyConnectedLayer {
    fn new<I: IntoIterator<Item = f32>>(input_count: usize, output_count: usize, initializer: I) -> Self {
        Self {
            input_count,
            output_count,
            weights_and_biases: initializer.into_iter().take((input_count + 1) * output_count).collect(),
        }
    }

    fn infer(&self, inputs: &[f32], outputs: &mut[f32]) {
        assert_eq!(self.input_count, inputs.len());
        assert_eq!(self.output_count, outputs.len());

        for output_idx in 0..self.output_count {
            let mut sum = 0.0;
            for input_idx in 0..self.input_count {
                sum += inputs[input_idx] * self.weights_and_biases[(self.input_count + 1) * output_idx + input_idx];
            }
            
            let bias = self.weights_and_biases[(self.input_count + 1) * output_idx + self.input_count];

            outputs[output_idx] = sum + bias;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let layer = FullyConnectedLayer::new(2, 3, [
            11.0,
            12.0,
            13.0,
            21.0,
            22.0,
            23.0,
            31.0,
            32.0,
            33.0,
        ]);
        let inputs = [1.0, 2.0];
        let mut outputs = [0.0, 0.0, 0.0];
        layer.infer(&inputs, &mut outputs);
        assert_eq!(outputs, [48.0, 88.0, 128.0]);
    }
}
