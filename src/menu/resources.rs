use bevy::prelude::*;

#[derive(Resource)]
pub struct FixMenuTimer {
    pub timer: Timer,
}
impl Default for FixMenuTimer {
    fn default() -> FixMenuTimer {
        FixMenuTimer {
            timer: Timer::from_seconds(0.1, TimerMode::Once),
        }
    }
}

#[derive(Resource)]
pub struct DrawCordsTracker {
    pub enabled: bool
}

impl Default for DrawCordsTracker {
    fn default() -> Self {
        Self {
            enabled: true,
        }
    }
}

#[derive(Resource)]
pub struct FpsTracker {
    pub enabled: bool,
    pub fps: u32,
    pub frame_time: f32,
    pub frame_count: u32,
}

impl Default for FpsTracker {
    fn default() -> Self {
        Self {
            enabled: true,
            fps: 0,
            frame_time: 0.0,
            frame_count: 0,
        }
    }
}

impl FpsTracker {
    pub fn update(&mut self, time: Res<Time>) {
        self.frame_time += time.delta_seconds();
        self.frame_count += 1;

        if self.frame_time >= 0.5 {
            self.fps = self.frame_count;
            self.frame_time -= 1.0;
            self.frame_count = 0;
        }
    }
}
