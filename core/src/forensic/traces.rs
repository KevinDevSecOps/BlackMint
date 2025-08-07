use std::path::Path;
use std::fs::{self, OpenOptions};
use std::io::Write;

/// Sobrescribe archivo 3 veces (NIST 800-88)
pub fn secure_wipe(path: &Path) -> std::io::Result<()> {
    let data = vec![0x00, 0xFF, 0xAA]; // Patrones de borrado
    let file_size = fs::metadata(path)?.len();
    
    for byte in data {
        let mut file = OpenOptions::new().write(true).open(path)?;
        file.write_all(&vec![byte; file_size as usize])?;
    }
    
    fs::remove_file(path)?;
    Ok(())
}
