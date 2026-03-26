//! 多指令顺序执行（批量指令串）。

use crate::{Executor, Pose};

#[test]
fn batch_mixed_commands_reaches_expected_pose() {
    // Given: 从原点朝北出发
    let mut executor = Executor::new(0, 0, 'N');

    // When: 执行混合指令串 MMRMMLM
    executor.execute("MMRMMLM");

    // Then: 到达 (2, 3) 且朝北
    assert_eq!(executor.query(), Pose::new(2, 3, 'N'));
}

#[test]
fn turn_left_then_move_advances_along_new_heading() {
    // Given: 从原点朝北出发
    let mut executor = Executor::new(0, 0, 'N');

    // When: 先左转再前进
    executor.execute("LM");

    // Then: 朝西且沿 X 负向移动一格
    assert_eq!(executor.query(), Pose::new(-1, 0, 'W'));
}
