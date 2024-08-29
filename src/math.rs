use std::ops::{Add, Mul};

/// Additive identity.
trait Zero {
    const ZERO: Self;
}

impl Zero for f32 {
    const ZERO: Self = 0.0;
}

impl Zero for usize {
    const ZERO: Self = 0;
}

#[derive(Debug, PartialEq)]
struct Array<T, D> {
    data: Vec<T>,
    dims: D,
}

impl<T, I0, I1> std::ops::Index<(I0, I1)> for Array<T, (I0, I1)>
where
    I0: Copy + Into<usize>,
    I1: Copy + Into<usize>,
{
    type Output = T;

    fn index(&self, (i0, i1): (I0, I1)) -> &Self::Output {
        let (d0, d1) = self.dims;
        let (i0, i1, d0, _) = (i0.into(), i1.into(), d0.into(), d1.into());
        &self.data[i1 * d0 + i0]
    }
}

fn dim_range<T: Copy + Into<usize> + From<usize>>(t: T) -> impl Iterator<Item = T> {
    (0..t.into()).map(T::from)
}

impl<T, I0, I1, I2> std::ops::Mul<Array<T, (I2, I0)>> for Array<T, (I0, I1)>
where
    T: Copy + Zero + Add<Output = T> + Mul<Output = T>,
    I0: Copy + PartialEq + Into<usize> + From<usize>,
    I1: Copy + PartialEq + Into<usize> + From<usize>,
    I2: Copy + PartialEq + Into<usize> + From<usize>,
{
    type Output = Result<Array<T, (I2, I1)>, (Self, Array<T, (I2, I0)>)>;

    fn mul(self, rhs: Array<T, (I2, I0)>) -> Self::Output {
        let common_dim = self.dims.0;
        if common_dim != rhs.dims.1 {
            return Err((self, rhs));
        }

        let dims = (rhs.dims.0, self.dims.1);

        let mut data = Vec::with_capacity(dims.0.into() * dims.1.into());

        for i1 in dim_range(dims.1) {
            for i2 in dim_range(dims.0) {
                let mut val = Zero::ZERO;
                for i0 in dim_range(common_dim) {
                    val = val + self[(i0, i1)] * rhs[(i2, i0)];
                }
                data.push(val);
            }
        }

        Ok(Array { data, dims })
    }
}

#[macro_export]
macro_rules! index_type {
    ($T:ident) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        struct $T(usize);

        impl From<$T> for usize {
            fn from(value: $T) -> Self {
                value.0
            }
        }

        impl From<usize> for $T {
            fn from(value: usize) -> Self {
                Self(value)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    index_type!(A);
    index_type!(B);
    index_type!(C);

    #[test]
    fn x() {
        let a1 = Array {
            data: vec![
                1, 2, 3, //
                4, 5, 6, //
            ],
            dims: (A(3), B(2)),
        };

        assert_eq!(a1[(A(1), B(1))], 5);

        let a2 = Array {
            data: vec![
                1, 2, 3, 4, //
                5, 6, 7, 8, //
                9, 10, 11, 12, //
            ],
            dims: (C(4), A(3)),
        };

        let a3 = (a1 * a2).unwrap_or_else(|_| panic!("dimensionality mismatch"));

        assert_eq!(
            a3,
            Array {
                data: vec![
                    38, 44, 50, 56, //
                    83, 98, 113, 128, //
                ],
                dims: (C(4), B(2))
            }
        )
    }
}
