# Remove Duplicates from Sorted Array

This is a Rust implementation of the "Remove Duplicates from Sorted Array" problem

## Problem Description

Given an integer array `nums` sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in `nums`.

## Solution

The solution uses a two-pointer approach:

- One pointer (`i`) iterates through the array.
- Another pointer (`k`) places the next unique element.
- The algorithm runs in O(n) time and uses O(1) extra space.

## Usage

To use the solution, call the `remove_duplicates` function with a mutable reference to a sorted vector:

```rust
let mut nums = vec![1, 1, 2];
let k = Solution::remove_duplicates(&mut nums);
println!("Unique elements: {}", k); // Output: 2
println!("Modified array: {:?}", nums); // Output: [1, 2, 2]
```
