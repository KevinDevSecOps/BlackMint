use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::Path,
};

/// Patrones de sobrescritura (NIST 800-88 + Gutmann)
const WIPE_PATTERNS: [u8; 7] = [
    0x00,  // Null bytes
    0xFF,  // 1's
    0xAA,  // 0101
    0x55,  // 1010
    0xDB,  // 1101
    0x6D,  // 0110
    0x24,  // Random
];

/// Sobrescribe un archivo 7 veces y lo elimina
pub fn secure_wipe(path: &Path) -> io::Result<()> {
    let file_size = File::open(path)?.metadata()?.len();
    
    // Sobrescritura múltiple
    for &pattern in &WIPE_PATTERNS {
        let mut file = OpenOptions::new().write(true).open(path)?;
        file.write_all(&vec![pattern; file_size as usize])?;
        file.sync_all()?;  // Forzar escritura a disco
    }
    
    // Eliminación segura (FS-dependent)
    fs::remove_file(path)?;
    
    Ok(())
}
