pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    for _ in 0..v.len() {
        for i in 0..v.len() - 1 {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // inside a module you can use its parent
    // module's properties using super
    use super::*;
    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);
    }
}
