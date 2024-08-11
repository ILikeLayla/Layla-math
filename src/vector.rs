use super::NUM;

#[derive(Copy, Clone, Debug)]
pub struct Vector<const N: usize> {
    data: [NUM; N],
}

impl<const N: usize> Vector<N> {
    pub fn zeros() -> Self {
        Self {
            data: [0.0; N],
        }
    }

    pub fn ones() -> Self {
        Self {
            data: [1.0; N],
        }
    }

    pub fn rand() -> Self {
        Self {
            data: [0.0; N].map(|_| rand::random()),
        }
    }

    pub fn new(data: [NUM; N]) -> Self {
        Self { data }
    }

    pub fn replace(&mut self, data: [NUM; N]) {
        self.data = data;
    }

    pub fn data(&self) -> &[NUM; N] { &self.data }
    pub fn data_mut(&mut self) -> &mut [NUM; N] { &mut self.data }

    pub fn sketch(&self, n: NUM) -> Self {
        let mut result = Self::zeros();
        for i in 0..N {
            result.data[i] = self.data[i] * n;
        };
        result
    }

    pub fn sketch_assign(&mut self, n: NUM) {
        for i in 0..N {
            self.data[i] *= n;
        };
    }

    pub fn as_ptr(&self) -> *const NUM {
        self.data.as_ptr()
    }

}

impl<const N:usize> std::fmt::Display for Vector<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<const N: usize> std::ops::Add for Vector<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        for i in 0..N {
            result.data[i] += rhs.data[i];
        };
        result
    }
}

impl<const N: usize> std::ops::Neg for Vector<N> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut result = self.clone();
        for i in 0..N {
            result.data[i] = -result.data[i];
        };
        result
    }
}

impl<const N: usize> std::ops::Sub for Vector<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        use std::ops::{Neg, Add};
        self.add(rhs.neg())
    }
}

impl<const N: usize> std::ops::AddAssign for Vector<N> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.data[i] += rhs.data[i];
        }
    }
}

impl<const N: usize> std::ops::SubAssign for Vector<N> {
    fn sub_assign(&mut self, rhs: Self) {
        use std::ops::{AddAssign, Neg};
        let rhs = rhs.neg();
        self.add_assign(rhs);
    }
}

impl<const N: usize> std::ops::Index<usize> for Vector<N> {
    type Output = NUM;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < N, "Index out of bounds.");
        &self.data()[index]
    }
}

impl<const N: usize> std::ops::IndexMut<usize> for Vector<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < N, "Index out of bounds.");
        &mut self.data_mut()[index]
    }
}