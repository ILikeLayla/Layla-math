use layla_math;

#[test]
fn init_zeros() {
    let v = layla_math::Vector::<5>::zeros();
    println!("{}", v)
}

#[test]
fn init_rand() {
    let v = layla_math::Vector::<5>::rand();
    println!("{}", v)
}

#[test]
fn init_from_slice() {
    let v = layla_math::Vector::new([1.0, 2.0, 3.0, 4.0, 5.0]);
    println!("{}", v)
}

#[test]
fn sketch_test() {
    let v = layla_math::Vector::new([1.0, 2.0, 3.0, 4.0, 5.0]);
    println!("{}", v.sketch(5.0));
}

#[test]
fn iter_test() {
    let v = layla_math::Vector::new([1.0, 2.0, 3.0, 4.0, 5.0]);
    for i in v.data().iter() { println!("{}", i) }
}