#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_secure_wipe() -> io::Result<()> {
        // Crear archivo temporal
        let mut file = NamedTempFile::new()?;
        let path = file.path();
        
        // Escribir datos sensibles
        file.write_all(b"datos ultra secretos")?;
        
        // Probar limpieza
        assert!(secure_wipe(path).is_ok());
        assert!(!path.exists());  // Verificar eliminación
        
        Ok(())
    }
}
