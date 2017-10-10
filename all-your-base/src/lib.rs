pub fn convert(digits: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, &'static str> {
    if from_base < 2 {
        return Err("Input base must be greater than 1");
    }

    if to_base < 2 {
        return Err("Output base must be greater than 1");
    }

    if digits.iter().any(|&d| d >= from_base) {
        return Err("Input contains invalid digits");
    }

    let number = digits.iter().rev().enumerate().fold(
        0,
        |acc, (index, digit)| {
            acc + digit * from_base.pow(index as u32)
        },
    );

    let mut converted = Vec::new();
    let mut power = 1;

    while number / power > 0 {
        converted.push((number % (power * to_base)) / power);
        power *= to_base;
    }

    converted.reverse();

    Ok(converted)
}
