pub fn bubble_sort<T: Ord>(items: &mut Vec<T>) {
    let item_count = items.len();

    if item_count == 0 {
        return;
    }

    for i in 0..item_count - 1 {
        for j in 0..(item_count - 1 - i) {
            if items[j] > items[j + 1] {
                items.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::bubble_sort;

    #[test]
    fn test_sorts_array() {
        let mut arr = vec![4, 2, 5, 1, 5, 6, 3];

        bubble_sort(&mut arr);

        assert_eq!(arr, vec![1, 2, 3, 4, 5, 5, 6])
    }
}
