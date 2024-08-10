fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);
    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {

        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0.clone());

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}


//  There are two solutions here we can clone like above or for performance we can return ownership for performance

fn fill_vec_b(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);
    vec
}

#[cfg(test)]
mod tests2 {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2b() {

        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec_b(vec0);

        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}