/// In-place selection sort
pub fn selection_sort<T: Ord>(items: &mut Vec<T>) {
    let item_count = items.len();

    if item_count == 0 {
        return;
    }

    for i in 0..item_count - 1 {
        let mut smallest_idx = i;
        // Find smallest value in remaining list
        for j in i..item_count - 1 {
            if items[j] < items[smallest_idx] {
                smallest_idx = j
            }
        }

        items.swap(i, smallest_idx);
    }
}

#[cfg(test)]
mod test {
    use super::selection_sort;

    #[test]
    fn test_basic() {
        let mut vec = vec![3, 2, 5, 6, 4, 8];

        selection_sort(&mut vec);

        assert_eq!(vec, vec![2, 3, 4, 5, 6, 8]);
    }
}
