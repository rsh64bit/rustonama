/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 *
 * https://leetcode.com/problems/two-sum/description/
 *
 * algorithms
 * Easy (45.28%)
 * Total Accepted:    2.8M
 * Total Submissions: 6.2M
 * Testcase Example:  '[2,7,11,15]\n9'
 *
 * Given an array of integers nums and an integer target, return indices of the
 * two numbers such that they add up to target.
 * You may assume that each input would have exactly one solution, and you may
 * not use the same element twice.
*/

/*sliding window will not work since numbers are not in order & although window is of 2,
the window is not contigious

When to Use Sliding Window?

Use it when:

Problem involves subarrays / substrings
You need contiguous elements
Asked for:
max/min sum
longest/shortest substring
count of valid windows

sliding window will NOT work if the elements are not contiguous

Now, if below vectors are sorted, then can use two pointers

So this case we have to use hashmaps :)
*/

use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sum_hash = HashMap::new();
    let mut pair: Vec<i32> = Vec::new();
    for (i, val) in nums.iter().enumerate() {
        if sum_hash.contains_key(&(target - val)) {
            match sum_hash.get(&(target - val)) {
                Some(v) => {
                    pair.push(i.try_into().unwrap());
                    pair.push(*v);
                }
                None => {
                    println!("wtf");
                }
            }
        } else {
            sum_hash.insert(val, i.try_into().unwrap());
        }
    }
    pair
}
