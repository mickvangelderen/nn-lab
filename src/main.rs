mod mnist;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn read_images_from_file(path: &str) -> Result<Vec<mnist::Image>> {
    let mut reader = flate2::read::GzDecoder::new(std::io::BufReader::new(std::fs::File::open(
        path
    )?));

    mnist::read_images(&mut reader)
}

fn read_labels_from_file(path: &str) -> Result<Vec<u8>> {
    let mut reader = flate2::read::GzDecoder::new(std::io::BufReader::new(std::fs::File::open(
        path
    )?));

    mnist::read_labels(&mut reader)
}

fn main() -> Result<()> {
    let train_images = read_images_from_file("data/train-images-idx3-ubyte.gz")?;
    let train_labels = read_labels_from_file("data/train-labels-idx1-ubyte.gz")?;

    for i in 0..5 {
        dbg!(&train_images[i]);
        dbg!(&train_labels[i]);
    }

    Ok(())
}
