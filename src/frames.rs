use std::time;

pub struct FrameWithDelay {
    pub frame: String,
    pub delay: time::Duration,
}

pub fn art2frames(art: &rs3a::Art) -> Vec<FrameWithDelay> {
    let mut frames = Vec::with_capacity(art.frames());
    for (i, frame) in art.to_ansi_frames().iter().enumerate() {
        frames.push(FrameWithDelay {
            frame: frame.clone() + &format!("\x1b[{}A", art.height()),
            delay: time::Duration::from_millis(art.get_frame_delay(i) as u64),
        });
    }
    frames
}

