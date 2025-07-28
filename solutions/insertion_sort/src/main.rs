// How to avoid that students use sort without creating a program to
// restrict functions in the code??
// Just use a grep?
// Implement the insertion sort algorithm but implement it in a step
// by step so we can test that each step is corresponding with the algorithm

// Insertion sort algorithmAlgorithm

// To sort an array of size n in ascending order:

// 1: Iterate from arr[1] to arr[n] over the array.

// 2: Compare the current element (key) to its predecessor.

// 3: If the key element is smaller than its predecessor, compare it
// to the elements before. Move the greater elements one position up
// to make space for the swapped element.

use insertion_sort::*;

fn main() {
    let mut target = [5, 3, 7, 2, 1, 6, 8, 4];
    insertion_sort(&mut target, 1);
    println!("{:?}", target);

    let mut target = [5, 3, 7, 2, 1, 6, 8, 4];
    let len = target.len();
    insertion_sort(&mut target, len - 1);
    println!("{:?}", target);
}

