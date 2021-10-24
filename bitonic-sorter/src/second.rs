use super::SortOrder;

fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    if x.len().is_power_of_two() {
        match *order {
            SortOrder::Ascending => do_sort(x, true),
            SortOrder::Descending => do_sort(x, false),
        };
        return Ok(());
    }
    Err(format!("The length of x is not a power of two. (x.len(): {})", x.len()))
}

fn do_sort<T: Ord>(x: &mut [T], up: bool) {
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        do_sort(&mut x[..mid_point], true);
        do_sort(&mut x[mid_point..], false);
        sub_sort(x, up);
    }
}

fn sub_sort<T: Ord>(x: &mut [T], up: bool) {
    if x.len() > 1 {
        compare_and_swap(x, up);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up);
    }
}

fn compare_and_swap<T: Ord>(x: &mut [T], up: bool) {
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        if (x[i] > x[mid_point + i]) == up {
            x.swap(i, mid_point + i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort;
    use parameterized::parameterized;
    use crate::SortOrder;

    #[parameterized(
        input = {
            vec![10, 30, 11, 20, 4, 330, 21, 110],
            vec![10, 30, 11, 20, 4, 330, 21, 110],
        },
        order = {
            SortOrder::Ascending,
            SortOrder::Descending,
        },
        expected = {
            vec![4, 10, 11, 20, 21, 30, 110, 330],
            vec![330, 110, 30, 21, 20, 11, 10, 4],
        }
    )]
    fn 正常系_u32型の配列のソート(input: Vec<u32>, order: SortOrder, expected: Vec<u32>) {
        let mut actual = input;
        assert_eq!(sort(&mut actual, &order), Ok(()));
        assert_eq!(actual, expected);
    }

    #[parameterized(
        input = {
            vec!["Rust", "is", "fast", "and", "memory-effecient", "with", "no", "GC"],
            vec!["Rust", "is", "fast", "and", "memory-effecient", "with", "no", "GC"],
        },
        order = {
            SortOrder::Ascending,
            SortOrder::Descending,
        },
        expected = {
            vec!["GC", "Rust", "and", "fast", "is", "memory-effecient", "no", "with"],
            vec!["with", "no", "memory-effecient", "is", "fast", "and", "Rust", "GC"],
        }
    )]
    fn 正常系_str型の配列のソート(input: Vec<&str>, order: SortOrder, expected: Vec<&str>) {
        let mut actual = input;
        assert_eq!(sort(&mut actual, &order), Ok(()));
        assert_eq!(actual, expected);
    }

    #[test]
    fn 異常系_要素数が2の累乗となっていない() {
        let mut x = vec![10, 30, 11];
        match sort(&mut x, &SortOrder::Ascending) {
            Ok(_) => panic!("sort() must return Err."),
            Err(e) => assert_eq!(e, "The length of x is not a power of two. (x.len(): 3)"),
        }
        assert!(sort(&mut x, &SortOrder::Ascending).is_err());
    }
}
