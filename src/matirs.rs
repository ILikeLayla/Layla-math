use super::Vector;

struct RawMatrix<const N: usize, const M: usize>([Vector<N>; M]);


impl<const N: usize, const M: usize> std::ops::Index<usize> for RawMatrix<N, M> {
    type Output = Vector<M>;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < N, "Index out of bounds.");
        &self[index]
    }
}

impl<const N: usize, const M: usize> std::ops::IndexMut<usize> for RawMatrix<N, M> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < N, "Index out of bounds.");
        &mut self[index]
    }
}

impl<const N: usize, const M: usize> RawMatrix<N, M> {
    fn zeros() -> Self {
        Self([Vector::zeros(); M])
    }

    fn ones() -> Self {
        Self([Vector::ones(); M])
    }

    fn rand() -> Self {
        Self([Vector::rand(); M])
    }

    fn new(data: [Vector<N>; M]) -> Self {
        Self(data)
    }

    fn transpose(&self) -> RawMatrix<M, N> {
        let mut data = [Vector::<M>::zeros(); N]; // M cloumns, N rows.
        for (i, row) in self.0.iter().enumerate() { // N cloumns, M rows.
            for (j, elem) in row.data().iter().enumerate() { // N cloumns, M rows.
                data[j][i] = *elem; // M cloumns, N rows.
            }
        }
        RawMatrix(data)
    }
}

pub struct Matrix<const N: usize, const M: usize> {
    data: RawMatrix<N, M>, // N cloumns, M rows.
    transpose: Option<RawMatrix<M, N>>,
}

impl<const N: usize, const M: usize> Matrix<N, M> {
    pub fn new(data: [Vector<N>; M]) -> Self {
        Self {
            data: RawMatrix::new(data),
            transpose: None,
        }
    }

    pub fn zeros() -> Self {
        Self {
            data: RawMatrix::zeros(), // N cloumns, M rows.
            transpose: None,
        }
    }

    pub fn ones() -> Self {
        Self {
            data: RawMatrix::ones(), // N cloumns, M rows.
            transpose: None,
        }
    }
    
    pub fn rand() -> Self {
        Self {
            data: RawMatrix::rand(), // N cloumns, M rows.
            transpose: None,
        }
    }

    // pub fn transpose(&self) -> Matrix<M, N> {
        
    // }
}

impl<const N: usize, const M: usize> std::fmt::Display for Matrix<N, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for (i, row) in self.data.0.iter().enumerate() { 
            string.push_str(&format!("{}  {:?}\n", i, row.data())); // N cloumns, M rows.
        }
        write!(f, "{}", string) // N cloumns, M rows.
    }
}

impl<const N: usize, const M: usize> std::ops::Index<usize> for Matrix<N, M> {
    type Output = Vector<M>;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < N, "Index out of bounds.");
        &self.data[index]
    }
}

impl<const N: usize, const M: usize> std::ops::IndexMut<usize> for Matrix<N, M> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < N, "Index out of bounds.");
        &mut self.data[index]
    }
}