#!/bin/bash
echo "[+] Hardening Linux Baseline"
# 1. Deshabilitar servicios innecesarios
sudo systemctl disable ftpd telnetd
# 2. Asegurar SSH
sudo sed -i 's/PermitRootLogin yes/PermitRootLogin no/' /etc/ssh/sshd_config
# 3. Habilitar firewall (UFW)
sudo ufw enable
echo "[+] Done. Revisa /var/log/secure para auditoría."