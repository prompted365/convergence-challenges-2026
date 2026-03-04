//! Command execution abstraction.
//!
//! The `Sys` trait wraps shell command execution the same way `Fs` wraps
//! filesystem access. Live runs use `std::process::Command`; tests can
//! inject a mock that returns canned output without requiring the target
//! OS, root access, or network connectivity.

use anyhow::Result;

// ─── Command result ──────────────────────────────────────────────────────────

/// Output of a command execution.
#[derive(Debug, Clone)]
pub struct CmdResult {
    pub stdout: String,
    pub stderr: String,
    pub code: Option<i32>,
}

impl CmdResult {
    /// Truncated summary suitable for report notes.
    pub fn summarize(&self, max_bytes: usize) -> String {
        let combined = format!(
            "[exit={}] stdout({} bytes) stderr({} bytes)\n{}",
            self.code.map(|c| c.to_string()).unwrap_or_else(|| "?".to_string()),
            self.stdout.len(),
            self.stderr.len(),
            &self.stdout,
        );
        if combined.len() <= max_bytes {
            combined
        } else {
            format!("{}…[truncated]", &combined[..max_bytes])
        }
    }

    /// True when the command exited with code 0.
    pub fn success(&self) -> bool {
        self.code == Some(0)
    }
}

// ─── Sys trait ───────────────────────────────────────────────────────────────

/// Abstraction over command execution.
pub trait Sys: Send + Sync {
    /// Run a command with the given arguments and capture output.
    fn run(&self, cmd: &str, args: &[&str]) -> Result<CmdResult>;
}

// ─── Live implementation ─────────────────────────────────────────────────────

/// Real command execution via `std::process::Command`.
pub struct LiveSys;

impl Sys for LiveSys {
    fn run(&self, cmd: &str, args: &[&str]) -> Result<CmdResult> {
        let output = std::process::Command::new(cmd)
            .args(args)
            .output()?;

        Ok(CmdResult {
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            code: output.status.code(),
        })
    }
}

// ─── Mock implementation ─────────────────────────────────────────────────────

/// Mock command execution for deterministic testing.
/// Register expected commands and their outputs before use.
pub struct MockSys {
    responses: std::collections::HashMap<String, CmdResult>,
}

impl MockSys {
    pub fn new() -> Self {
        Self {
            responses: std::collections::HashMap::new(),
        }
    }

    /// Register a canned response for `cmd args...`.
    /// Key format: `"cmd arg1 arg2"` (space-joined).
    pub fn register(&mut self, key: &str, result: CmdResult) {
        self.responses.insert(key.to_string(), result);
    }
}

impl Sys for MockSys {
    fn run(&self, cmd: &str, args: &[&str]) -> Result<CmdResult> {
        let key = if args.is_empty() {
            cmd.to_string()
        } else {
            format!("{} {}", cmd, args.join(" "))
        };

        self.responses
            .get(&key)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("mock: no response registered for: {key}"))
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_sys_returns_registered() {
        let mut sys = MockSys::new();
        sys.register(
            "uname -a",
            CmdResult {
                stdout: "Linux testhost 5.4.0".to_string(),
                stderr: String::new(),
                code: Some(0),
            },
        );
        let r = sys.run("uname", &["-a"]).unwrap();
        assert!(r.success());
        assert!(r.stdout.contains("Linux"));
    }

    #[test]
    fn mock_sys_missing_command_errors() {
        let sys = MockSys::new();
        assert!(sys.run("nonexistent", &[]).is_err());
    }

    #[test]
    fn cmd_result_summarize_truncates() {
        let r = CmdResult {
            stdout: "a".repeat(5000),
            stderr: String::new(),
            code: Some(0),
        };
        let s = r.summarize(100);
        assert!(s.len() < 200);
        assert!(s.contains("truncated"));
    }
}
