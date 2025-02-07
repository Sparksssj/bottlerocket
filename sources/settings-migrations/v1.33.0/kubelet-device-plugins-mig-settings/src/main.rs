use migration_helpers::common_migrations::AddPrefixesMigration;
use migration_helpers::{migrate, Result};
use std::process;

/// We added new settings for configuring the NVIDIA k8s device plugin.
fn run() -> Result<()> {
    migrate(AddPrefixesMigration(vec![
        "settings.kubelet-device-plugins.nvidia.device-partitioning-strategy",
        "settings.kubelet-device-plugins.nvidia.mig",
        "configuration-files.nvidia-k8s-device-plugin-mig-conf",
    ]))
}

// Returning a Result from main makes it print a Debug representation of the error, but with Snafu
// we have nice Display representations of the error, so we wrap "main" (run) and print any error.
// https://github.com/shepmaster/snafu/issues/110
fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
