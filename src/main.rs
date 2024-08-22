use nn::Result;
use rand::{distributions::Standard, rngs::StdRng, Rng, SeedableRng};

fn main() -> Result<()> {
    let mut rng = StdRng::from_seed([0u8; 32]);

    let l0 = nn::nn::FullyConnectedLayer::new(1, 8, (&mut rng).sample_iter(Standard));
    let mut l0_outputs = [0.0; 8];
    let l1 = nn::nn::FullyConnectedLayer::new(8, 8, (&mut rng).sample_iter(Standard));
    let mut l1_outputs = [0.0; 8];
    let l2 = nn::nn::FullyConnectedLayer::new(8, 1, (&mut rng).sample_iter(Standard));
    let mut l2_outputs = [0.0; 1];
    
    let (x, y) = nn::data::sample_sine(&mut rng);

    l0.infer(&[x], &mut l0_outputs);
    l1.infer(&l0_outputs, &mut l1_outputs);
    l2.infer(&l1_outputs, &mut l2_outputs);

    dbg!(x);
    dbg!(l0_outputs);
    dbg!(l1_outputs);
    dbg!(l2_outputs[0]);
    dbg!(y);

    Ok(())
}
