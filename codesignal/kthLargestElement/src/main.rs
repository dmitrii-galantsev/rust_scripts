use rand::Rng;

fn kthLargestElement(nums: Vec<i32>, k: i32) -> i32 {
    let mut tmp : Vec<i32> = nums.clone();
    tmp.sort_by(|x, y| y.cmp(x));
    return tmp[k as usize - 1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_element() {
        assert_eq!(1, kthLargestElement([2, 1].to_vec(), 2));
        assert_eq!(2, kthLargestElement([2, 1].to_vec(), 1));
    }

    #[test]
    fn four_element() {
        assert_eq!(3, kthLargestElement([4, 1, 3, 2].to_vec(), 2));
        assert_eq!(4, kthLargestElement([4, 1, 3, 2].to_vec(), 1));
        assert_eq!(2, kthLargestElement([4, 1, 3, 2].to_vec(), 3));
    }

    #[test]
    fn simple_test() {
        let mut rng = rand::thread_rng();
        let result: Vec<i32>;

        // num of tests
        for _ in 0..100 {
            let rand_vec: Vec<i32> = (0..100)            // 100 random vectors
                        .map(|_| rng.gen_range(0, 1000)) // random from 0 to 1000
                        .collect();

            let mut expected: Vec<i32> = rand_vec.clone();
            expected.sort_by(|x, y| y.cmp(x));

            // generate result
            let result: Vec<i32> = (1..101)
                        .map(|i| kthLargestElement(rand_vec.clone(), i))
                        .collect();

            // compare result to expected vector
            assert_eq!(expected, result);
        }
    }
}

fn main() {
    assert_eq!(3, kthLargestElement([4, 1, 3, 2].to_vec(), 2));
    assert_eq!(4, kthLargestElement([4, 1, 3, 2].to_vec(), 1));
    assert_eq!(2, kthLargestElement([4, 1, 3, 2].to_vec(), 3));

    let mut rng = rand::thread_rng();
    let mut vals: Vec<i32>;

    vals = (0..100).map(|_| rng.gen_range(0, 100)).collect();

    println!("arr {:?}", vals);
    vals.sort();
    println!("arr {:?}", vals);

    // print all
    (0..100).zip(500..600)
            .for_each(|(i, v)| println!("i: {:?} v: {:?}", i, v));

    // [0, 50, 1, 51... 5, 55] with temp variable
    let mut my_flat_vec : Vec<i32> = Vec::new();
    (0..5).zip(50..55)
            .for_each(|(i, v)| {my_flat_vec.push(i); my_flat_vec.push(v);});

    println!("my_flat_vec: {:?}", my_flat_vec);

    // [0, 50, 1, 51... 5, 55] without temp variable
    println!("foldr_vec  : {:?}",
        (0..5)
        .zip(50..55)
        .fold(vec![], |mut acc, (i, v)| {
            acc.push(i);
            acc.push(v);
            acc
        }));


    println!("broke");
    for i in 0..5 {
        print!("{}.", i * 10);
    }

    println!();
    println!("woke");
    (0..5).for_each(|i| print!("{}.", i * 10));

    println!();
    println!("bespoke");
    (0..).step_by(10).take(5).for_each(|i| print!("{}.", i));

    println!();
}
