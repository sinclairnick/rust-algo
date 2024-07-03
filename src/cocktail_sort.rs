use std::cmp::Ordering;

#[derive(Debug)]
enum Direction {
    Forward(usize),
    Backwards(usize),
}

pub fn cocktail_sort<T: Ord>(items: &mut Vec<T>) {
    let item_count = items.len();

    if item_count == 0 {
        return;
    }

    let mut left = 0;
    let mut right = item_count - 1;
    let mut dir = Direction::Forward(0);

    loop {
        println!("{dir:?}");

        if left >= right - 1 {
            break;
        }

        match dir {
            Direction::Forward(i) => {
                match items[i].cmp(&items[i + 1]) {
                    Ordering::Greater => {
                        items.swap(i, i + 1);
                    }
                    _ => {}
                };

                let is_right_end = i == right - 1;

                if is_right_end {
                    dir = Direction::Backwards(right - 1);
                    left = left + 1;
                } else {
                    dir = Direction::Forward(i + 1);
                }
            }
            Direction::Backwards(i) => {
                match items[i].cmp(&items[i - 1]) {
                    Ordering::Less => {
                        items.swap(i, i - 1);
                    }
                    _ => {}
                };

                let is_left_end = i == left;

                if is_left_end {
                    dir = Direction::Forward(left + 1);
                    right = right - 1;
                } else {
                    dir = Direction::Backwards(i - 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::cocktail_sort;

    #[test]
    fn test_basic() {
        let mut vec = vec![4, 3, 6, 2, 1, 4];

        cocktail_sort(&mut vec);

        assert_eq!(vec, vec![1, 2, 3, 4, 4, 6]);
    }
}
