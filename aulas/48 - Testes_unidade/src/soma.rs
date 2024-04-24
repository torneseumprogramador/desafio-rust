pub fn soma(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testa_soma() {
        assert_eq!(soma(2, 2), 4);
    }
}
