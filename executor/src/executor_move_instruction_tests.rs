//! `M` 前进指令：沿当前朝向移动一格。

use crate::{Executor, Pose};

#[test]
fn move_forward_from_north_increments_y() {
    // Given: 位于原点、朝北
    let mut executor = Executor::new(0, 0, 'N');

    // When: 执行一次前进
    executor.execute("M");

    // Then: Y 增加 1，朝向不变
    assert_eq!(executor.query(), Pose::new(0, 1, 'N'));
}

#[test]
fn move_forward_from_east_increments_x() {
    // Given: 位于原点、朝东
    let mut executor = Executor::new(0, 0, 'E');

    // When: 执行一次前进
    executor.execute("M");

    // Then: X 增加 1，朝向不变
    assert_eq!(executor.query(), Pose::new(1, 0, 'E'));
}

#[test]
fn move_forward_from_south_decrements_y() {
    // Given: 位于原点、朝南
    let mut executor = Executor::new(0, 0, 'S');

    // When: 执行一次前进
    executor.execute("M");

    // Then: Y 减少 1，朝向不变
    assert_eq!(executor.query(), Pose::new(0, -1, 'S'));
}

#[test]
fn move_forward_from_west_decrements_x() {
    // Given: 位于原点、朝西
    let mut executor = Executor::new(0, 0, 'W');

    // When: 执行一次前进
    executor.execute("M");

    // Then: X 减少 1，朝向不变
    assert_eq!(executor.query(), Pose::new(-1, 0, 'W'));
}
