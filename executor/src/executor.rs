#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pose {
    pub x: i32,
    pub y: i32,
    pub heading: char,
}

impl Pose {
    pub fn new(x: i32, y: i32, heading: char) -> Self {
        Pose { x, y, heading }
    }
}

impl Default for Pose {
    fn default() -> Self {
        Pose {
            x: 0,
            y: 0,
            heading: 'N',
        }
    }
}

pub struct Executor {
    pose: Pose,
}

impl Default for Executor {
    fn default() -> Self {
        Self {
            pose: Pose::default(),
        }
    }
}

impl Executor {
    /// 使用坐标与朝向初始化执行器。
    pub fn new(x: i32, y: i32, heading: char) -> Self {
        Self {
            pose: Pose::new(x, y, heading),
        }
    }

    pub fn with_pose(pose: Pose) -> Self {
        Executor { pose }
    }

    /// 按顺序执行指令；当前支持 `M` 前进一格。
    pub fn execute(&mut self, cmds: &str) {
        for c in cmds.chars() {
            if c == 'M' {
                self.move_forward();
            }
        }
    }

    pub fn query(&self) -> Pose {
        self.pose
    }

    fn move_forward(&mut self) {
        match self.pose.heading {
            'N' => self.pose.y += 1,
            'S' => self.pose.y -= 1,
            'E' => self.pose.x += 1,
            'W' => self.pose.x -= 1,
            _ => {}
        }
    }
}
