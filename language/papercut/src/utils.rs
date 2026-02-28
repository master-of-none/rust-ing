pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    (&s[1..], &s[0..1])
}

#[cfg(test)]
mod tests {
    use super::*;
    
}
