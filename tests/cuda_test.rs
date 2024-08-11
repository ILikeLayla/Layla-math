#[cfg(feature = "cuda-support")]
mod tests{
    use layla_math;
    #[test]
    fn hello_cuda_test() {
        layla_math::cuda::safe::hello_cuda();
    }

    #[test]
    fn print_vector_test() {
        let v = layla_math::Vector::<5>::rand();
        println!("{}", v);
        layla_math::cuda::safe::print_vector(&v);
    }
}