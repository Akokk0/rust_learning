//! 常用章节
//!
//! # Panics
//! 函数可能发生panic的场景
//!
//! # Errors
//! 如果函数返回Result，描述可能的错误种类，以及可导致错误的条件
//!
//! # Safety
//! 如果函数处于unsafe调用，就应该解释函数unsafe的原因，以及调用者确保的使用前提
//!

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = profile_learning::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
///
pub fn add_one(x: i32) -> i32 {
    x + 1
}