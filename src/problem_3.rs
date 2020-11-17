use crate::util;

pub fn problem_3() -> Result<u64, String> {
    const N: u64 = 600851475143;
    for n in 1..N / 2 {
        if N % n == 0 {
            let factor = N / n;
            if util::is_prime(factor) {
                return Ok(factor);
            }
        }
    }
    Err(format!("Largest prime factor not found for number {}.", N))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let solution = problem_3();
        let solution = solution.unwrap();
        assert_eq!(6857, solution);
    }
}
