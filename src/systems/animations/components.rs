use std::time::Duration;
//NEW
#[derive(Clone, Copy)]
pub enum AnimationDuration {
    /// Total time for one full pass through all frames.
    PerCycle(Duration),
    /// Fixed time each individual frame is held.
    PerFrame(Duration),
}

impl AnimationDuration {
    pub(crate) fn frame_interval(&self, frame_count: usize) -> Duration {
        match self {
            AnimationDuration::PerFrame(d) => *d,
            AnimationDuration::PerCycle(d) => *d / frame_count as u32,
        }
    }
}
