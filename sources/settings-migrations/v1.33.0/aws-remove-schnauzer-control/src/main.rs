use migration_helpers::common_migrations::RemoveSchnauzerMigration;
use migration_helpers::{migrate, Result};
use std::process;

const OLD_CONTROL_CTR_CMDLINE: &str =
    "schnauzer-v2 render --requires 'aws@v1(helpers=[ecr-prefix])' --template '{{ ecr-prefix settings.aws.region }}/bottlerocket-control:v0.7.20'";

/// We are removing settings.host-containers.control.source setting
/// to populate it from defaults.
fn run() -> Result<()> {
    migrate(RemoveSchnauzerMigration {
        setting: "settings.host-containers.control.source",
        old_cmdline: OLD_CONTROL_CTR_CMDLINE,
    })
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
