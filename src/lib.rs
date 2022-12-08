pub fn do_thing(left: usize, right: usize) -> usize {
    if left > right { left + right } else { left * right }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gt_works() {
        let result = do_thing(4, 2);
        assert_eq!(result, 6);
    }
}
