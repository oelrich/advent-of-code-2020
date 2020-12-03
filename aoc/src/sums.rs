pub fn find_sum(sum: i32, values: Vec<i32>) -> Option<(i32, i32)> {
    for value_0 in &values {
        for value_1 in &values {
            if value_0 + value_1 == sum {
                return Some((*value_0, *value_1));
            }
        }
    }
    None
}

pub fn find_triplet_sum(sum: i32, values: Vec<i32>) -> Option<(i32, i32, i32)> {
    for value_0 in &values {
        for value_1 in &values {
            for value_2 in &values {
                if value_0 + value_1 + value_2 == sum {
                    return Some((*value_0, *value_1, *value_2));
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn pairwise() {
        let (a, b) = super::find_sum(2020, vec![1721, 979, 366, 299, 675, 1456]).unwrap();
        assert_eq!(a * b, 514579);
    }

    #[test]
    fn triplet() {
        let (a, b, c) =
            super::find_triplet_sum(2020, vec![1721, 979, 366, 299, 675, 1456]).unwrap();
        assert_eq!(a * b * c, 241861950);
    }
}
