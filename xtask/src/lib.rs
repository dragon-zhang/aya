use anyhow::{anyhow, Context as _, Result};
use std::process::Command;

pub const AYA_BUILD_INTEGRATION_BPF: &str = "AYA_BUILD_INTEGRATION_BPF";
pub const LIBBPF_DIR: &str = "xtask/libbpf";

pub fn exec(cmd: &mut Command) -> Result<()> {
    let status = cmd
        .status()
        .with_context(|| format!("failed to run {cmd:?}"))?;
    match status.code() {
        Some(code) => match code {
            0 => Ok(()),
            code => Err(anyhow!("{cmd:?} exited with code {code}")),
        },
        None => Err(anyhow!("{cmd:?} terminated by signal")),
    }
}
