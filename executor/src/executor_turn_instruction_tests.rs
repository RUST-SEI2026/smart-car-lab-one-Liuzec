//! `L` / `R` 转向指令：原地旋转 90°。

use crate::{Executor, Pose};

#[test]
fn four_left_turns_restore_north_without_moving() {
    // Given: 位于原点、朝北
    let mut executor = Executor::new(0, 0, 'N');

    // When: 连续四次左转
    executor.execute("LLLL");

    // Then: 回到朝北，位置不变
    assert_eq!(executor.query(), Pose::new(0, 0, 'N'));
}

#[test]
fn four_right_turns_restore_north_without_moving() {
    // Given: 位于原点、朝北
    let mut executor = Executor::new(0, 0, 'N');

    // When: 连续四次右转
    executor.execute("RRRR");

    // Then: 回到朝北，位置不变
    assert_eq!(executor.query(), Pose::new(0, 0, 'N'));
}

#[test]
fn single_left_turn_from_north_faces_west() {
    // Given: 朝北
    let mut executor = Executor::new(0, 0, 'N');

    // When: 左转一次
    executor.execute("L");

    // Then: 朝西且未位移
    assert_eq!(executor.query(), Pose::new(0, 0, 'W'));
}

#[test]
fn single_right_turn_from_north_faces_east() {
    // Given: 朝北
    let mut executor = Executor::new(0, 0, 'N');

    // When: 右转一次
    executor.execute("R");

    // Then: 朝东且未位移
    assert_eq!(executor.query(), Pose::new(0, 0, 'E'));
}
