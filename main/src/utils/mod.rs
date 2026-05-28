/*
pub mod math;
pub mod string;
has to use with
    let sum = utils::math::add(10, 20);
    println!("Sum: {}", sum);

    let upper = utils::string::to_uppercase("rust");
    println!("Uppercase: {}", upper);
*/

pub mod helper;
pub mod math;
pub mod string;
use crate::kinds::*;

pub use math::add;
pub use string::to_uppercase;

/// 混合两个基础颜色，返回一个二级颜色。
/// 这里目前是学习模块示例，后续可以把完整颜色匹配逻辑补齐。
/// ```rust
/// use learning_notes::kinds::{PrimaryColor, SecondaryColor};
/// use learning_notes::utils::mix;
///
/// assert!(matches!(mix(PrimaryColor::Yellow, PrimaryColor::Blue), SecondaryColor::Green));
/// ```
pub fn mix(_c1: PrimaryColor, _c2: PrimaryColor) -> SecondaryColor {
    SecondaryColor::Green
}
