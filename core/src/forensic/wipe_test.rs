#[test]
fn test_secure_wipe() {
    let test_file = tempfile::NamedTempFile::new().unwrap();
    let path = test_file.path();
    
    std::fs::write(path, "datos sensibles").unwrap();
    assert!(secure_wipe(path).is_ok());
    assert!(!path.exists());
}
