use std::sync::OnceLock;

use zprs::{Node, Progress};

static ROOT: OnceLock<Node> = OnceLock::new();

/// Keep the returned reporter alive until all work and child processes finish.
/// Progress is optional: failure to open a channel must not stop the command.
pub fn init(name: &str) -> Option<Progress> {
    let reporter = Progress::new(name, 0).ok()?;
    let _ = ROOT.set(reporter.root());
    Some(reporter)
}

/// A zero total reports activity without estimating a completion percentage.
pub fn start(name: &str, total: usize) -> Node {
    ROOT.get()
        .map(|root| root.start(name, total as u64))
        .unwrap_or_default()
}

pub fn during<T>(name: &str, work: impl FnOnce() -> T) -> T {
    let _task = start(name, 0);
    work()
}
