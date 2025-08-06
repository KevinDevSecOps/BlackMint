```rust
/// Sobrescribe archivo con 3 pasos (NIST 800-88)
pub fn secure_wipe(path: &Path) -> Result<(), IoError> { /* ... */ }
```

| Parámetro | Tipo     | Descripción               |
|-----------|----------|---------------------------|
| `path`    | `&Path`  | Ruta al archivo a borrar  |
### **🚀 Despliegue en 3 Pasos**
1. **Compila y prueba**:
   ```bash
   cargo build --release && maturin develop
   ```

2. **Ejecuta tests**:
   ```bash
   pytest py/tests/
   ```

3. **Publica**:
   ```bash
   maturin publish --interpreter python3.10
   ```

---

```bash
# Comando para integrar todo:
git add . && git commit -m "feat: Integración completa Rust-Python"
```
