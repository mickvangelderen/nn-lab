use rand::Rng;

pub fn sample_sine<R>(rng: &mut R) -> (f32, f32) where R: Rng + ?Sized {
    let x = rng.gen();
    (x, x.sin())
}
