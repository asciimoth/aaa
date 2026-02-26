use argh::FromArgValue;
use rs3a::{Art, Cell};

#[derive(FromArgValue, PartialEq, Debug, Copy, Clone)]
pub enum Effect {
    None,
    RollerUp,
    RollerDown,
    RollerLeft,
    RollerRight,
}

impl Effect {
    pub fn apply(&self, art: &mut Art, frame: usize) {
        match self {
            Effect::None => {}
            Effect::RollerUp => apply_roller_up(art, frame),
            Effect::RollerDown => apply_roller_down(art, frame),
            Effect::RollerLeft => apply_roller_left(art, frame),
            Effect::RollerRight => apply_roller_right(art, frame),
        }
    }
}

impl Default for Effect {
    fn default() -> Self {
        Self::None
    }
}

fn apply_roller_up(art: &mut Art, frame: usize) {
    for _ in 0..art.height() {
        art.dup_frame(frame);
        art.shift_up_frame(frame, 1, Cell::default());
    }
}

fn apply_roller_down(art: &mut Art, frame: usize) {
    for _ in 0..art.height() {
        art.dup_frame(frame);
        art.shift_down_frame(frame, 1, Cell::default());
    }
}

fn apply_roller_left(art: &mut Art, frame: usize) {
    for _ in 0..art.width() {
        art.dup_frame(frame);
        art.shift_left_frame(frame, 1, Cell::default());
    }
}

fn apply_roller_right(art: &mut Art, frame: usize) {
    for _ in 0..art.width() {
        art.dup_frame(frame);
        art.shift_right_frame(frame, 1, Cell::default());
    }
}
