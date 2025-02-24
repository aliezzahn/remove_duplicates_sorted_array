use remove_duplicates_sorted_array::Solution;

fn main() {
    // Example usage
    let mut nums = vec![1, 1, 2];
    let k = Solution::remove_duplicates(&mut nums);
    println!("Number of unique elements: {}", k); // Output: 2
    println!("Modified array: {:?}", nums); // Output: [1, 2, 2]
}