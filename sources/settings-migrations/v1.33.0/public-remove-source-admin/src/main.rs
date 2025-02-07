use migration_helpers::common_migrations::RemoveMatchingString;
use migration_helpers::{migrate, Result};
use std::process;

const OLD_ADMIN_CTR: &str = "public.ecr.aws/bottlerocket/bottlerocket-admin:v0.11.16";

/// We are removing settings.host-containers.admin.source setting
/// to populate it from defaults.
fn run() -> Result<()> {
    migrate(RemoveMatchingString {
        setting: "settings.host-containers.admin.source",
        old_val: OLD_ADMIN_CTR,
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
