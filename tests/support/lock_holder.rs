use std::env;
use std::os::unix::io::AsRawFd;
use std::thread;
use std::time::Duration;

use nix::fcntl::{flock, FlockArg};

fn main() {
    let path = env::args().nth(1).expect("usage: lock_holder PATH [HOLD_MS]");
    let hold_ms: u64 = env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(200);

    let file = std::fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&path)
        .expect("open lock file");

    flock(file.as_raw_fd(), FlockArg::LockExclusive).expect("acquire LOCK_EX");
    thread::sleep(Duration::from_millis(hold_ms));
}
