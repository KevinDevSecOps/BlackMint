import hashlib
import os

def check_malware(file_path, known_hashes):
    with open(file_path, 'rb') as f:
        file_hash = hashlib.sha256(f.read()).hexdigest()
    return file_hash in known_hashes

if __name__ == "__main__":
    known_hashes = ["a1b2c3...", "d4e5f6..."]  # Hashes de malware conocidos
    target_file = input("Ruta del archivo a analizar: ")
    if check_malware(target_file, known_hashes):
        print("[!] ¡Posible malware detectado!")
    else:
        print("[+] Archivo limpio.")