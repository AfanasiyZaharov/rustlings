// iterators4.rs


struct FactorialCounter{
    curVal: u64,
    steps: u64,
}

impl FactorialCounter {
    fn new() -> FactorialCounter {
        FactorialCounter { curVal: 1, steps: 1 }
    }
}

impl Iterator for FactorialCounter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.steps > 100 {
            return None;
        }
        self.steps += 1;
        self.curVal = self.curVal * self.steps;
        Some(self.curVal)
    }
}

pub fn factorial(num: u64) -> u64 {
    (1..=num).fold(1, |acc, v| acc * v)
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
