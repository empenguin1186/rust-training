use super::SortOrder;

fn sort<T: Ord>(x: &mut [T], order: &SortOrder) {
    match *order {
        SortOrder::Ascending => do_sort(x, true),
        SortOrder::Descending => do_sort(x, false),
    }
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
    fn sort_u32(input: Vec<u32>, order: SortOrder, expected: Vec<u32>) {
        let mut actual = input;
        sort(&mut actual, &order);
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
    fn sort_str(input: Vec<&str>, order: SortOrder, expected: Vec<&str>) {
        let mut actual = input;
        sort(&mut actual, &order);
        assert_eq!(actual, expected);
    }
}
