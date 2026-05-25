#![forbid(unsafe_code)]

use std::fs::OpenOptions;
use std::io;
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::time::{Duration, Instant};

use nix::errno::Errno;
use nix::fcntl::{flock, FlockArg};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitOutcome {
    Unlocked,
    Timeout,
}

/// Returns `Ok(true)` when no process holds an exclusive flock on `path`.
/// Returns `Ok(false)` when another process holds `LOCK_EX`.
pub fn try_lock_status(path: &Path) -> io::Result<bool> {
    let file = OpenOptions::new().read(true).write(true).open(path)?;
    let fd = file.as_raw_fd();

    match flock(fd, FlockArg::LockExclusiveNonblock) {
        Ok(()) => {
            flock(fd, FlockArg::Unlock)?;
            Ok(true)
        }
        Err(Errno::EWOULDBLOCK) => Ok(false),
        Err(errno) => Err(errno.into()),
    }
}

/// Polls until the path is unlocked, `timeout` elapses, or an I/O error occurs.
pub fn wait_for_unlock(
    path: &Path,
    timeout: Duration,
    poll: Duration,
) -> io::Result<WaitOutcome> {
    let start = Instant::now();
    loop {
        if try_lock_status(path)? {
            return Ok(WaitOutcome::Unlocked);
        }
        if start.elapsed() >= timeout {
            return Ok(WaitOutcome::Timeout);
        }
        std::thread::sleep(poll);
    }
}
