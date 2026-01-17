use std::process::Command;

fn main() {
    // Rerun build script when:
    // - Git index changes (commits, staging) - to update/clear dirty status
    // - Source files change - to detect modifications for dirty status
    // - Git HEAD changes (branch switches) - to update commit hash
    println!("cargo::rerun-if-changed=.git/index");
    println!("cargo::rerun-if-changed=.git/HEAD");
    println!("cargo::rerun-if-changed=src");

    // Git commit hash (short)
    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map_or_else(|| "unknown".to_owned(), |s| s.trim().to_owned());

    // Git commit date (ISO 8601)
    let git_date = Command::new("git")
        .args(["log", "-1", "--format=%ci"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map_or_else(|| "unknown".to_owned(), |s| s.trim().to_owned());

    // Check if working directory has uncommitted changes to tracked files
    // Uses diff-index to ignore untracked files (like Cargo.lock on first build)
    let is_dirty = Command::new("git")
        .args(["diff-index", "--quiet", "HEAD", "--"])
        .status()
        .map(|s| !s.success()) // exit code 1 = differences found
        .unwrap_or(false);

    let dirty_suffix = if is_dirty { "-dirty" } else { "" };

    // Build timestamp (UTC)
    let build_timestamp = chrono::Utc::now()
        .format("%Y-%m-%d %H:%M:%S UTC")
        .to_string();

    // Export as compile-time environment variables
    println!("cargo::rustc-env=BUILD_GIT_HASH={git_hash}{dirty_suffix}");
    println!("cargo::rustc-env=BUILD_GIT_DATE={git_date}");
    println!("cargo::rustc-env=BUILD_TIMESTAMP={build_timestamp}");
}
