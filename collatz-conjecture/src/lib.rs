struct Collatz(u64);

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.0 == 1 {
            return None;
        }

        self.0 = if self.0 % 2 == 0 {
            self.0 / 2
        } else {
            3 * self.0 + 1
        };

        Some(self.0)
    }
}

fn build_collatz(n: u64) -> Result<Collatz, &'static str> {
    if n == 0 {
        return Err("n must be greater than 0");
    }

    Ok(Collatz(n))
}

pub fn collatz(n: u64) -> Result<u64, &'static str> {
    let collatz = build_collatz(n)?;

    Ok(collatz.count() as u64)
}
