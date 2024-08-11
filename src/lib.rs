mod vector;
mod matirs;
mod setting;
pub use vector::*;
pub use matirs::*;
// #[cfg(feature = "python-support")]
// use pyo3::prelude::*;

pub const E:f64 = std::f64::consts::E;
pub const PI:f64 = std::f64::consts::PI;

#[cfg(not(feature = "rough-num"))]
type NUM = f64;
#[cfg(feature = "rough-num")]
type NUM = f32;

#[cfg(feature = "cuda-support")]
pub mod cuda {
    use super::*;

    extern {
        fn hello_cuda_unsafe();
        #[cfg(feature = "rough-num")]
        fn print_vector_f32_unsafe(v: *const f32, length: usize);
        #[cfg(not(feature = "rough-num"))]
        fn print_vector_f64_unsafe(v: *const f64, length: usize);
    }

    pub mod safe {
        use super::*;

        pub fn hello_cuda() { 
            unsafe { hello_cuda_unsafe() }
        }

        pub fn print_vector<const N: usize>(v: &Vector<N>) {
            unsafe {
                #[cfg(not(feature = "rough-num"))]
                print_vector_f64_unsafe(v.as_ptr(), N);

                #[cfg(feature = "rough-num")]
                print_vector_f32_unsafe(v.as_ptr(), N)
            }
        }
    }
}

// #[cfg(feature = "python-support")]
// #[allow(deprecated)]
// #[pymodule]
// fn layla_math(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add("E", E)?;
//     m.add("PI", PI)?;
//     Ok(())
// }