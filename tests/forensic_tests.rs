#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_secure_wipe() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path();
        
        std::fs::write(path, "datos sensibles").unwrap();
        secure_wipe(path).unwrap();
        
        assert!(!path.exists());
    }
}
