use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

fn lockwatch_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_lockwatch"))
}

fn lock_holder_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_lock_holder"))
}

#[test]
fn waits_until_sidecar_releases_lock() {
    let path = std::env::temp_dir().join(format!(
        "lockwatch-it-{}-{}.lock",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = std::fs::remove_file(&path);

    let mut holder = Command::new(lock_holder_bin())
        .arg(&path)
        .arg("200")
        .spawn()
        .expect("spawn lock_holder");

    std::thread::sleep(Duration::from_millis(50));

    let status = Command::new(lockwatch_bin())
        .arg(&path)
        .arg("--timeout")
        .arg("5")
        .arg("--poll-ms")
        .arg("50")
        .status()
        .expect("run lockwatch");

    holder.wait().expect("wait lock_holder");

    assert!(status.success(), "expected exit 0, got {:?}", status.code());
    let _ = std::fs::remove_file(&path);
}

#[test]
fn nonexistent_path_exits_2() {
    let path = std::env::temp_dir().join(format!(
        "lockwatch-missing-{}-{}.lock",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    assert!(!path.exists());

    let status = Command::new(lockwatch_bin())
        .arg(&path)
        .status()
        .expect("run lockwatch");

    assert_eq!(status.code(), Some(2));
}
