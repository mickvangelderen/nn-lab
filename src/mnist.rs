use super::Result;
use byteorder::{BigEndian, ReadBytesExt};
use std::{
    fmt::{self, Write},
    io::Read,
};

pub fn read_images_from_file(path: &str) -> Result<Vec<Image>> {
    let mut reader = flate2::read::GzDecoder::new(std::io::BufReader::new(std::fs::File::open(
        path
    )?));

    read_images(&mut reader)
}

pub fn read_labels_from_file(path: &str) -> Result<Vec<u8>> {
    let mut reader = flate2::read::GzDecoder::new(std::io::BufReader::new(std::fs::File::open(
        path
    )?));

    read_labels(&mut reader)
}

pub fn read_labels<R: Read>(reader: &mut R) -> Result<Vec<u8>> {
    let magic = reader.read_i32::<BigEndian>()?;
    assert_eq!(magic, 0x0801);
    let count: usize = reader.read_i32::<BigEndian>()?.try_into()?;
    let mut labels = vec![0; count];
    reader.read_exact(&mut labels[..])?;
    Ok(labels)
}

pub fn read_images<R: Read>(reader: &mut R) -> Result<Vec<Image>> {
    let magic = reader.read_i32::<BigEndian>()?;
    assert_eq!(magic, 0x0803);
    let count = reader.read_i32::<BigEndian>()?;
    let width: u32 = reader.read_i32::<BigEndian>()?.try_into()?;
    let height: u32 = reader.read_i32::<BigEndian>()?.try_into()?;
    let images = (0..count)
        .map(|_| {
            let mut pixels = vec![0; (width * height) as usize];
            reader.read_exact(&mut pixels[..])?;
            Result::Ok(Image {
                pixels,
                width,
                height,
            })
        })
        .collect::<Result<Vec<Image>>>()?;
    Ok(images)
}

pub struct Image {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('\n')?;

        f.write_char('+')?;
        for _ in 0..self.width {
            for _ in 0..2 {
                f.write_char('-')?;
            }
        }
        f.write_char('+')?;
        f.write_char('\n')?;

        for y in 0..self.height {
            f.write_char('|')?;

            for x in 0..self.width {
                let pixel = self.pixels[(y * self.width + x) as usize];
                for _ in 0..2 {
                    f.write_char(match pixel {
                        0..=63 => ' ',
                        64..=127 => '-',
                        128..=191 => 'x',
                        192..=255 => 'X',
                    })?;
                }
            }

            f.write_char('|')?;
            f.write_char('\n')?;
        }

        f.write_char('+')?;
        for _ in 0..self.width {
            for _ in 0..2 {
                f.write_char('-')?;
            }
        }
        f.write_char('+')?;
        f.write_char('\n')?;

        Ok(())
    }
}
