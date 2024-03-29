#![feature(box_patterns)]
#![feature(is_sorted)]
#![feature(toowned_clone_into)]
#[allow(dead_code)]
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

extern crate core;
extern crate regex;

use std::cell::RefCell;
use std::rc::Rc;

mod buddy;
#[macro_use]
mod macros;
mod add_two_numbers;
mod bleatrix_trotter;
mod broken_calc;
mod bulb_switch;
mod candy;
mod complex_number_multiplication;
mod count_arrangement;
mod delete_node;
mod find_max_consecutive_ones;
mod find_min;
mod flipgame;
mod fraction_addition;
mod help_your_granny;
mod is_alien_sorted;
mod is_valid;
mod largest_perimeter_triangle;
mod last_digit;
mod longest_chunked_palindrome_decomposition;
mod masking_personal_information;
mod max_chunks_to_sorted;
mod max_subarray_sum_circular;
mod min_distance;
mod num_equiv_domino_pairs;
mod num_tile_possibilities;
mod permute;
mod reaching_points;
mod recent_counter;
mod repeated_times;
mod string_compress;
mod student_attendance_record_i;
mod triples_with_bitwise_and_equal_to_zero;

#[cfg(test)]
mod tests {}
