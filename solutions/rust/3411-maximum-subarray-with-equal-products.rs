fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}
fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}
fn gcd_all(nums: &[u64]) -> u64 {
    nums.iter().copied().fold(0, gcd)
}
fn lcm_all(nums: &[u64]) -> u64 {
    nums.iter().copied().fold(1, lcm)
}
// Now, let's take the biggest edge case: hundred tens.
// [10,10,10,..,10,10] => prod=10^100; gcd=10; lcm=10
// 10*10 is not 10^100
// To be fair, after that I just used this code:
/*
fn main() {
    let bla: &[u64] = &[1,3,9];
    println!("{:?}", bla);
    println!("{}", bla.iter().copied().product::<u64>());
    println!("{}", gcd_all(bla));
    println!("{}", lcm_all(bla));
}
*/
// And by changing `bla`, found out a few things:
// 1) any pair of numbers will be valid, thus minimum length is 2;
// 2) duplicates in arrays with `length>2` make product bigger than gcd*lcm.
//  Except `1`. Ones don't increase product, thus they can be duplicated in a valid subarray.
// 3) if gcd of any pair of numbers is bigger than 1, product is bigger again.
// Number (3) observation basically means we can't have two items from the same group:
// `2,4,6,8,10`, `3,6,9`, `5,10`
// There's probably some pretty mathematical reason for this,
// but my observations are enough to give an answer.
impl Solution {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        let mut left_iter = nums.iter();
        let mut right_iter = left_iter.clone();
        let current_window = &mut [0; 11];
        current_window[right_iter.next().copied().unwrap() as usize] += 1;
        current_window[right_iter.next().copied().unwrap() as usize] += 1;
        right_iter.fold(2, |max_len, &x| {
            current_window[x as usize] += 1;
            while !is_valid_window(current_window) {
                current_window[left_iter.next().copied().unwrap() as usize] -= 1;
            }

            max_len.max(window_length(current_window))
        })
    }
}

fn window_length(win: &[i32; 11]) -> i32 {
    win.iter().copied().sum()
}

fn is_valid_window(win: &[i32; 11]) -> bool {
    at_most_one_group_2(win)
        && at_most_one_group_3(win)
        && at_most_one_group_5(win)
        && win[2..].iter().all(|&c| c <= 1)
}
fn at_most_one_group_2(win: &[i32; 11]) -> bool {
    [win[2], win[4], win[6], win[8], win[10]]
        .into_iter()
        .sum::<i32>()
        <= 1
}
fn at_most_one_group_3(win: &[i32; 11]) -> bool {
    [win[3], win[6], win[9]].into_iter().sum::<i32>() <= 1
}
fn at_most_one_group_5(win: &[i32; 11]) -> bool {
    win[5] + win[10] <= 1
}
