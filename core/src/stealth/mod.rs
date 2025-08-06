//! Técnicas de ofuscación y evasión

use sysinfo::{System, SystemExt};

/// Detecta entornos sandbox/VM
pub fn is_sandbox() -> bool {
    let sys = System::new_all();
    sys.cpus().len() <= 2 || sys.total_memory() < 4_000_000_000
}
