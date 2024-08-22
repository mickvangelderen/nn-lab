use nn::Result;

fn main() -> Result<()> {
    let train_images = nn::mnist::read_images_from_file("data/train-images-idx3-ubyte.gz")?;
    let train_labels = nn::mnist::read_labels_from_file("data/train-labels-idx1-ubyte.gz")?;

    for i in 0..5 {
        dbg!(&train_images[i]);
        dbg!(&train_labels[i]);
    }

    Ok(())
}
