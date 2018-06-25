const PRIME_LIMIT: u32 = 500_000;

struct Primes {
    sieve: Vec<bool>,
    last_prime: u32,
}

impl Primes {
    fn new() -> Self {
        Primes {
            sieve: (0..PRIME_LIMIT).map(|_| false).collect(),
            last_prime: 1,
        }
    }
}

impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let mut num = self.last_prime;

        loop {
            num += 1;

            if !self.sieve[num as usize] {
                let mut i = num * 2;

                while i < PRIME_LIMIT {
                    self.sieve[i as usize] = true;
                    i += num;
                }

                self.last_prime = num;
                return Some(num);
            }
        }
    }
}

pub fn nth(n: u32) -> Option<u32> {
    if n == 0 {
        None
    } else {
        Primes::new().nth((n - 1) as usize)
    }
}
