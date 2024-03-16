/**
 * My website only supports en-gb locale, so this wont work for other locales
 */
fn format<T>(n: T, precision: usize, commas: bool) -> String
where
    T: std::fmt::Display,
{
    let mut formatted = format!("{:.*}", precision, n);

    let mut parts = formatted.split('.');

    let mut left = parts.next().unwrap_or("").to_string();
    let right = parts.next().unwrap_or("");

    if commas {
        left = left
            .chars()
            .rev()
            .enumerate()
            .fold(String::new(), |mut acc, (i, c)| {
                if i > 0 && i % 3 == 0 {
                    acc.push(',');
                }
                acc.push(c);
                acc
            })
            .chars()
            .rev()
            .collect();
    }

    if precision == 0 {
        return left.to_string();
    }

    format!("{}.{}", left.chars().collect::<String>(), right)
}

pub trait FormatNumber {
    fn format(&self, precision: usize, commas: bool) -> String;
}

impl FormatNumber for f32 {
    fn format(&self, precision: usize, commas: bool) -> String {
        format(*self, precision, commas)
    }
}

impl FormatNumber for f64 {
    fn format(&self, precision: usize, commas: bool) -> String {
        format(*self, precision, commas)
    }
}

impl FormatNumber for i32 {
    fn format(&self, precision: usize, commas: bool) -> String {
        format(*self, precision, commas)
    }
}

impl FormatNumber for i64 {
    fn format(&self, precision: usize, commas: bool) -> String {
        format(*self, precision, commas)
    }
}

impl FormatNumber for u32 {
    fn format(&self, precision: usize, commas: bool) -> String {
        format(*self, precision, commas)
    }
}

impl FormatNumber for u64 {
    fn format(&self, precision: usize, commas: bool) -> String {
        format(*self, precision, commas)
    }
}

impl FormatNumber for usize {
    fn format(&self, precision: usize, commas: bool) -> String {
        format(*self, precision, commas)
    }
}

impl FormatNumber for isize {
    fn format(&self, precision: usize, commas: bool) -> String {
        format(*self, precision, commas)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        assert_eq!(format(1000.5, 0, true), "1,000");
        assert_eq!(format(1000.5, 1, true), "1,000.5");
        assert_eq!(format(1000.5, 2, true), "1,000.50");
        assert_eq!(format(1000.5, 3, true), "1,000.500");
        assert_eq!(format(1000.5, 0, false), "1000");
        assert_eq!(format(1000.5, 1, false), "1000.5");
        assert_eq!(format(1000.5, 2, false), "1000.50");
        assert_eq!(format(1000.5, 3, false), "1000.500");
        assert_eq!(format(0.5, 3, false), "0.500");
    }
}
