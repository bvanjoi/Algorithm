#[derive(Debug)]
pub struct TestCase<T, U> {
    input: T,
    expected: U,
}

impl<T, U> TestCase<T, U> {
    pub fn new(input: T, expected: U) -> Self {
        TestCase { input, expected }
    }

    pub fn get_input(&self) -> &T {
        &self.input
    }

    pub fn get_expected(&self) -> &U {
        &self.expected
    }
}

pub mod sort;
