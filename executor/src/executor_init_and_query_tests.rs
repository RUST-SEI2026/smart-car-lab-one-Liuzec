//! Executor 初始化与姿态查询相关测试。

use crate::{Executor, Pose};

#[test]
fn default_executor_reports_origin_facing_north() {
    // Given: 使用默认构造的 Executor（对应默认姿态 0,0,N）
    let executor = Executor::default();

    // When: 查询当前姿态
    let pose = executor.query();

    // Then: 坐标为原点且朝向为北
    assert_eq!(pose, Pose::new(0, 0, 'N'));
}

#[test]
fn new_executor_reports_configured_pose() {
    // Given: 使用 new 指定坐标与朝向
    let executor = Executor::new(3, -2, 'E');

    // When: 查询当前姿态
    let pose = executor.query();

    // Then: 与构造参数一致
    assert_eq!(pose, Pose::new(3, -2, 'E'));
}

#[test]
fn with_pose_executor_reports_same_pose() {
    // Given: 使用 Pose 构造 Executor
    let initial = Pose::new(10, 20, 'S');
    let executor = Executor::with_pose(initial);

    // When: 查询当前姿态
    let pose = executor.query();

    // Then: 与给定 Pose 相同
    assert_eq!(pose, initial);
}
