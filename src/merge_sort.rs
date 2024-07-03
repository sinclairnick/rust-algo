use std::{cmp::Ordering, fmt::Debug};

/// A merge sort implementation without using any clones.
/// Allocations are made,
pub fn merge_sort<T: Ord + Debug>(items: Vec<T>) -> Vec<T> {
    let item_count = items.len();

    if item_count < 2 {
        return items;
    }

    let middle = item_count / 2 as usize;

    // Split items
    let mut split_a = items;
    let split_b = split_a.split_off(middle);

    println!("Split: {split_a:?} {split_b:?}");

    // Run merge sort on each
    let mut sorted: Vec<T> = Vec::new();
    let mut sorted_a = merge_sort(split_a);
    let mut sorted_b = merge_sort(split_b);

    // Sort the two arrays
    while sorted.len() < item_count {
        // If either array is empty, append the other to sorted
        if sorted_a.len() == 0 {
            sorted.append(&mut sorted_b);
            break;
        }
        if sorted_b.len() == 0 {
            sorted.append(&mut sorted_a);
            break;
        }

        let head_a = sorted_a.first();
        let head_b = sorted_b.first();

        // Given the break statements above, both head_a and head_b should
        // be defined here
        if let Some(a) = head_a {
            if let Some(b) = head_b {
                match a.cmp(b) {
                    Ordering::Greater => {
                        // sorted + b[0]
                        sorted.push(sorted_b.remove(0));
                    }
                    _ => {
                        // sorted + a[0]
                        sorted.push(sorted_a.remove(0));
                    }
                }
            }
        }
    }

    return sorted;
}

#[cfg(test)]
mod test {
    use super::merge_sort;

    #[test]
    fn test_basic() {
        let vec = vec![3, 6, 2, 6, 5, 1, 7, 8];

        let result = merge_sort(vec);

        println!("{result:?}");
    }
}
