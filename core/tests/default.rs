/*
    Appellation: default <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[test]
fn lib_compiles() {
    /// a generic function for adding two numbers
    fn adder<A, B>(a: A, b: B) -> A::Output
    where
        A: core::ops::Add<B>,
    {
        a + b
    }
    //
    assert_eq!(adder(10, 10), 20);
    assert_ne!(adder(1.0, 1.0), 3.0);
}
