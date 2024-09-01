use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut},
};

pub trait TryIndex<I>: Index<I> {
    fn get(&self, index: I) -> Option<&Self::Output>;
}

impl<T, const N: usize, I: std::slice::SliceIndex<[T]>> TryIndex<I> for [T; N] {
    fn get(&self, index: I) -> Option<&Self::Output> {
        <[T]>::get(self, index)
    }
}

// impl<T> TryIndex for [T] {
//     fn get(&self, index: usize) -> Option<&Self::Output> {
//         <[T]>::get(self, index)
//     }
// }

// impl<T> TryIndex for &[T] {
//     fn get(&self, index: usize) -> Option<&Self::Output> {
//         <[T]>::get(self, index)
//     }
// }

// impl<T> TryIndex for Vec<T> {
//     fn get(&self, index: usize) -> Option<&Self::Output> {
//         <&Vec<T>>::get(self, index)
//     }
// }

// pub trait TryIndexMut: TryIndex + IndexMut {
//     fn get_mut(&mut self) -> Option<&mut Self::Output>;
// }

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

// #[derive(Debug, PartialEq)]
// struct Array<T, D> {
//     data: Vec<T>,
//     dims: D,
// }

// impl<T, I0, I1> std::ops::Index<(I0, I1)> for Array<T, (I0, I1)>
// where
//     I0: Copy + Into<usize>,
//     I1: Copy + Into<usize>,
// {
//     type Output = T;

//     fn index(&self, (i0, i1): (I0, I1)) -> &Self::Output {
//         let (d0, d1) = self.dims;
//         let (i0, i1, d0, _) = (i0.into(), i1.into(), d0.into(), d1.into());
//         &self.data[i1 * d0 + i0]
//     }
// }

// fn dim_range<T: Copy + Into<usize> + From<usize>>(t: T) -> impl Iterator<Item = T> {
//     (0..t.into()).map(T::from)
// }

// impl<T, I0, I1, I2> std::ops::Mul<Array<T, (I2, I0)>> for Array<T, (I0, I1)>
// where
//     T: Copy + Zero + Add<Output = T> + Mul<Output = T>,
//     I0: Copy + PartialEq + Into<usize> + From<usize>,
//     I1: Copy + PartialEq + Into<usize> + From<usize>,
//     I2: Copy + PartialEq + Into<usize> + From<usize>,
// {
//     type Output = Result<Array<T, (I2, I1)>, (Self, Array<T, (I2, I0)>)>;

//     fn mul(self, rhs: Array<T, (I2, I0)>) -> Self::Output {
//         let common_dim = self.dims.0;
//         if common_dim != rhs.dims.1 {
//             return Err((self, rhs));
//         }

//         let dims = (rhs.dims.0, self.dims.1);

//         let mut data = Vec::with_capacity(dims.0.into() * dims.1.into());

//         for i1 in dim_range(dims.1) {
//             for i2 in dim_range(dims.0) {
//                 let mut val = Zero::ZERO;
//                 for i0 in dim_range(common_dim) {
//                     val = val + self[(i0, i1)] * rhs[(i2, i0)];
//                 }
//                 data.push(val);
//             }
//         }

//         Ok(Array { data, dims })
//     }
// }

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

pub trait Indices {
    type Array;

    fn into_usize(self) -> Self::Array;

    fn from_usize(array: Self::Array) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Static<const N: usize>;

impl<const N: usize> From<Static<N>> for usize {
    fn from(_: Static<N>) -> Self {
        N
    }
}

pub trait Indexer: IntoIterator<Item = Self::Expanded> {
    type Expanded;

    fn flatten(&self, indices: Self::Expanded) -> Option<usize>;

    fn expand(&self, index: usize) -> Option<Self::Expanded>;

    fn len(&self) -> usize;
}

pub struct Square<E>(E);

impl<E0> Indexer for Square<E0>
where
    E0: Copy + Into<usize>,
{
    type Expanded = [usize; 2];

    fn flatten(&self, [i0, i1]: Self::Expanded) -> Option<usize> {
        let e0 = self.0.into();

        if i0 >= e0 || i1 >= e0 {
            return None;
        }

        Some(i1 * e0 + i0)
    }

    fn expand(&self, i: usize) -> Option<Self::Expanded> {
        let e0 = self.0.into();

        if i >= e0 * e0 {
            return None
        }

        let i0 = i % e0;
        let i1 = i / e0;

        Some([i0, i1])
    }

    fn len(&self) -> usize {
        let e0 = self.0.into();
        e0 * e0
    }
}

struct IndexerIter<X> {
    index: usize,
    indexer: X,
}

impl<X> IndexerIter<X> {
    fn new(indexer: X) -> Self {
        Self {
            index: 0,
            indexer,
        }
    }
}

impl<X> Iterator for IndexerIter<X> where X: Indexer {
    type Item = X::Expanded;

    fn next(&mut self) -> Option<Self::Item> {
        self.indexer.expand(self.index).inspect(|_| self.index += 1)
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len()
    }
}

impl<X> ExactSizeIterator for IndexerIter<X> where X: Indexer {
    fn len(&self) -> usize {
        self.indexer.len() - self.index
    }
}

impl<E0> IntoIterator for Square<E0> where E0: Copy + Into<usize> {
    type Item = <Self as Indexer>::Expanded;

    type IntoIter = IndexerIter<Self>;

    fn into_iter(self) -> Self::IntoIter {
        IndexerIter::new(self)
    }
}

pub struct SquareSymmetric<E>(E);

impl<E0> Indexer for SquareSymmetric<E0>
where
    E0: Copy + Into<usize>,
{
    type Expanded = [usize; 2];

    fn flatten(&self, [i0, i1]: Self::Expanded) -> Option<usize> {
        let e0 = self.0.into();

        if i0 >= e0 || i1 >= e0 {
            return None;
        }

        // Ensure i0 >= i1.
        let (i0, i1) = if i0 >= i1 { (i0, i1) } else { (i1, i0) };

        // Compute flat index.
        //      i0
        //    0, 1, 2,
        // i1 -, 3, 4,
        //    -, -, 5,
        // TODO: Should we index so that extending the matrix does not change the indices? So instead use
        //      i0
        //    0, 1, 3,
        // i1 -, 2, 4,
        //    -, -, 5,
        // this can impact perf

        Some(i0 + i1 * (2 * e0 - 1 - i1) / 2)
    }

    fn expand(&self, i: usize) -> Option<Self::Expanded> {
        let e0 = self.0.into();

        if i >= e0 * e0 {
            return None
        }

        let i0 = i % e0;
        let i1 = i / e0;

        Some([i0, i1])
    }

    fn len(&self) -> usize {
        let e0 = self.0.into();
        e0 * e0
    }
}

impl<E0> IntoIterator for SquareSymmetric<E0> where E0: Copy + Into<usize> {
    type Item = <Self as Indexer>::Expanded;

    type IntoIter = IndexerIter<Self>;

    fn into_iter(self) -> Self::IntoIter {
        IndexerIter::new(self)
    }
}

pub struct Array<D, X, I> {
    data: D,
    indexer: X,
    _index_type: PhantomData<I>,
}

// pub type Array2<D, X> = Array<D, X, [usize; 2]>;
// pub type Array3<D, X> = Array<D, X, [usize; 3]>;

impl<D, X, I> Array<D, X, I> {
    pub fn new(data: D, indexer: X) -> Self {
        Self {
            data,
            indexer,
            _index_type: PhantomData,
        }
    }

    pub fn into_inner(self) -> D {
        self.data
    }
}

fn oob() -> ! {
    panic!("out of bounds")
}

impl<D, X, I> Index<I> for Array<D, X, I>
where
    D: Deref<Target: Index<usize>>,
    X: Indexer<I::Array>,
    I: IntoArray<usize>,
{
    type Output = <D::Target as Index<usize>>::Output;

    fn index(&self, indices: I) -> &Self::Output {
        &self.data[self.indexer.flatten(indices.into_array()).unwrap_or_else(|| oob())]
    }
}

impl<D, X, I> IndexMut<I> for Array<D, X, I>
where
    D: DerefMut<Target: IndexMut<usize>>,
    X: Indexer<I::Array>,
    I: IntoArray<usize>,
{
    fn index_mut(&mut self, indices: I) -> &mut Self::Output {
        &mut self.data[self.indexer.flatten(indices.into_array()).unwrap_or_else(|| oob())]
    }
}

// impl<D0, X0, I, D1, X1> PartialEq<Array<D1, X1, I>> for Array<D0, X0, I>
// where
//     X0: Indexer<I::Array> + Copy + IntoArray<usize, Array = I::Array>,
//     X1: Indexer<I::Array> + Copy + IntoArray<usize, Array = I::Array>,
//     I: IntoArray<usize, Array: Eq>,
// {
//     fn eq(&self, other: &Array<D1, X1, I>) -> bool {
//         self.indexer.into_array() == other.indexer.into_array()
//         self.indexer.iter()
//     }
// }

trait IntoArray<T> {
    type Array;

    fn into_array(self) -> Self::Array;
}

impl<const N: usize> IntoArray<usize> for [usize; N] {
    type Array = Self;
    fn into_array(self) -> Self::Array {
        self
    }
}

// Does anyone use this?
impl<A, O> IntoArray<O> for (A,) where A: Into<O> {
    type Array = [O; 1];

    fn into_array(self) -> Self::Array {
        [self.0.into()]
    }
}

impl<A, B, O> IntoArray<O> for (A, B) where A: Into<O>, B: Into<O> {
    type Array = [O; 2];

    fn into_array(self) -> Self::Array {
        [self.0.into(), self.1.into()]
    }
}

impl<A, B, C, O> IntoArray<O> for (A, B, C) where A: Into<O>, B: Into<O>, C: Into<O> {
    type Array = [O; 3];

    fn into_array(self) -> Self::Array {
        [self.0.into(), self.1.into(), self.2.into()]
    }
}

impl<N, O> IntoArray<O> for SquareSymmetric<N> where N: Into<O>, O: Copy {
    type Array = [O; 2];

    fn into_array(self) -> Self::Array {
        let n = self.0.into();
        [n, n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The extent defines the number of elements in each dimension.
    // The representation of the extent can be zero sized (statically-known) and compressed (each dimension of a square matrix has the same number of elements).
    // The index types define the type of the index for each dimension.
    // The number of dimensions in the extent should match the number of index types.
    // An indexer maps valid multidimensional indices to flat indices.

    index_type!(Col);
    index_type!(Row);

    #[test]
    fn sym_tri() {
        let data = &mut [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let view = Array::<_, _, (Col, Row)>::new(data, SquareSymmetric(Static::<4>));

        assert_eq!(
            std::mem::size_of_val(&view),
            std::mem::size_of::<usize>(),
        );

        // let expected = Array::<_, _, (Col, Row)>::new([
        //     0, 1, 2, 3,
        //     1, 4, 5, 6,
        //     2, 5, 7, 8,
        //     3, 7, 8, 9,
        // ], Square(Static::<4>));

        // 0, 1, 2, 3,
        // 1, 4, 5, 6,
        // 2, 5, 7, 8,
        // 3, 6, 8, 9,

        assert_eq!(view[(Col(0), Row(0))], 0);
        assert_eq!(view[(Col(1), Row(0))], 1);
        assert_eq!(view[(Col(2), Row(0))], 2);
        assert_eq!(view[(Col(3), Row(0))], 3);

        assert_eq!(view[(Col(0), Row(1))], 1);
        assert_eq!(view[(Col(1), Row(1))], 4);
        assert_eq!(view[(Col(2), Row(1))], 5);
        assert_eq!(view[(Col(3), Row(1))], 6);

        assert_eq!(view[(Col(0), Row(2))], 2);
        assert_eq!(view[(Col(1), Row(2))], 5);
        assert_eq!(view[(Col(2), Row(2))], 7);
        assert_eq!(view[(Col(3), Row(2))], 8);

        assert_eq!(view[(Col(0), Row(3))], 3);
        assert_eq!(view[(Col(1), Row(3))], 6);
        assert_eq!(view[(Col(2), Row(3))], 8);
        assert_eq!(view[(Col(3), Row(3))], 9);
    }

    // index_type!(A);
    // index_type!(B);
    // index_type!(C);

    // #[test]
    // fn x() {
    //     let a1 = Array {
    //         data: vec![
    //             1, 2, 3, //
    //             4, 5, 6, //
    //         ],
    //         dims: (A(3), B(2)),
    //     };

    //     assert_eq!(a1[(A(1), B(1))], 5);

    //     let a2 = Array {
    //         data: vec![
    //             1, 2, 3, 4, //
    //             5, 6, 7, 8, //
    //             9, 10, 11, 12, //
    //         ],
    //         dims: (C(4), A(3)),
    //     };

    //     let a3 = (a1 * a2).unwrap_or_else(|_| panic!("dimensionality mismatch"));

    //     assert_eq!(
    //         a3,
    //         Array {
    //             data: vec![
    //                 38, 44, 50, 56, //
    //                 83, 98, 113, 128, //
    //             ],
    //             dims: (C(4), B(2))
    //         }
    //     )
    // }
}
