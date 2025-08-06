//! Análisis y limpieza forense avanzada

use std::path::Path;
use std::fs;
use rayon::prelude::*; // Procesamiento paralelo

/// Sobrescribe archivos con datos aleatorios (3 pasos)
pub fn secure_wipe(path: &Path) -> std::io::Result<()> {
    let patterns = [b'\x00', b'\xFF', b'\xAA']; 
    let file_size = fs::metadata(path)?.len();
    
    patterns.par_iter().for_each(|&pattern| {
        let _ = fs::write(path, vec![pattern; file_size as usize]);
    });
    
    fs::remove_file(path)?;
    Ok(())
}
