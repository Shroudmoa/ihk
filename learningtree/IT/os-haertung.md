# OS-Härtung

# OS-Härtung – IHK 2026 (Kompakt & Technisch)

---

## 1. Kernprinzip: Angriffsfläche minimieren
**Jeder offene Port / laufender Dienst = potenzieller Einstiegspunkt (OSI L4/L7).**
→ **Least Privilege** + **Minimalismus** = weniger CVEs, weniger Patch-Aufwand, bessere Performance (weniger RAM/CPU für ungenutzte Dienste).

---

## 2. Härtungssäulen (Checkliste)

| Säule | Maßnahme | Techn. Detail / Werkzeug | IHK/DSGVO-Bezug |
|-------|----------|--------------------------|-----------------|
| **Dienste** | Nicht benötigte stoppen & deaktivieren | `systemctl disable --now <dienst>`, `sc config <dienst> start= disabled` | Art. 32 DSGVO (Technische Maßnahmen) |
| **Software** | Unnötige Pakete deinstallieren | `apt purge`, `dnf remove`, Windows Features on Demand | Reduziert Code-Basis → weniger Bugs |
| **Ports** | Host-Firewall: Default-Deny, nur nötige Ports offen | `nftables`/`iptables`/`ufw`/`Windows Defender FW`; **RDP/SSH nur per Quell-IP o. VPN** | Netzwerksegmentierung (VLAN/Zone) |
| **Patchmgmt** | Updates zeitnah einspielen (Test → Prod) | WSUS, SCCM/Intune, `unattended-upgrades`, `dnf-automatic` | Schwachstellen-Management (ISO 27001) |
| **Auth** | Standard-PW ändern/deaktivieren, MFA erzwingen | `passwd -l root`, `pam_faillock`, Azure AD MFA, LAPS | Zugriffskontrolle (Art. 9 DSGVO) |
| **Protokolle** | Alt/unsicher abschalten (TLS 1.0/1.1, SMBv1, NTLMv1) | Registry `SchUseStrongCrypto`, `smb.conf: min protocol = SMB2_10` | Kryptografie-Nachweis (BSI TR-02102) |
| **Firmware** | UEFI/BIOS härten (Secure Boot, TPM, PW, CSM off) | Siehe `BIOS-UEFI.md` | Hardware-Root-of-Trust |
| **Logging** | Zentral, manipulationssicher, alerting | `rsyslog`/`journald` → Graylog/Splunk/ELK; `auditd` für krit. Syscalls | Audit-Trail (Art. 30 DSGVO) |
| **Crypto** | Zertifikate/Keys verwalten, HSM/TPM nutzen | `certbot`, `openssl`, `TPM2.0` für BitLocker/LUKS | Vertraulichkeit & Integrität |

---

## 3. Warum „Frische OS“ unsicher sind (Exam-Antwort, 3 Zeilen)
1. **General-Purpose-Default**: Hersteller aktivieren Dienste (Drucker, Remote-Mgmt, Discovery) für „Out-of-the-Box“-Nutzbarkeit → maximale Angriffsfläche.
2. **Standard-Credentials & offene Ports**: Administrator/admin, Standard-Community-Strings (SNMP), offene RDP/SSH ohne Quellfilterung.
3. **Veraltete Crypto-Defaults**: TLS 1.0/1.1, schwache Cipher-Suites, SMBv1 oft noch aktiv → Compliance-Verstoß (DSGVO/BSI).

---

## 4. Port ↔ Dienst ↔ Firewall – Harte Regel
```
Offener Port + lauschender Prozess (LISTEN) = TCP/UDP-Handshake möglich = Angriffsvektor
```
**Konsequenz**: `ss -tulpen` / `netstat -ano` → **jeder** Eintrag muss durch Change-Management begründet sein.
**Firewall-Regel**: `INGRESS: DENY ALL` → `ALLOW tcp/443 FROM 10.10.10.0/24` (Beispiel Webserver).

---

## 5. Performance-Gewinn durch Härtung (Nebenprodukt)
- Weniger Prozesse → weniger Context-Switches, weniger RAM (Shared Libraries), kleinere Kernel-Angriffsfläche.
- Weniger Log-Rauschen → SIEM effizienter, Storage sparender.

---

## 6. Ergänzende Themen (für „2 Fliegen, 1 Klatsche“)
| Thema | Verbindung zur Härtung | IHK-Prüfungsrelevanz |
|-------|------------------------|----------------------|
| **Netzwerksegmentierung** (VLAN, DMZ, Zero Trust) | Port-Sperren auf Host-Ebene reicht nicht; Defense in Depth | LAN/WAN-Sicherheit, OSI L2/L3 |
| **SIEM / Log-Management** | Härtung ohne Überwachung = Blindflug | DSGVO-Nachweispflicht, Incident Response |
| **Vulnerability Scanning** (OpenVAS, Nessus) | Validiert, ob Härtung greift | Schwachstellen-Management-Prozess |
| **Baseline / CIS Benchmarks** | Standardisierter Härtungszustand (z. B. CIS Level 1) | Audit-Nachweis, Compliance |
| **Application Whitelisting** (AppLocker, SELinux, AppArmor) | Verhindert Ausführung unbekannter Binaries →补丁-Management entlastet | Host-basierte Zugriffskontrolle |
| **Backup & Restore-Test** | Letzte Verteidigungslinie bei fehlgeschlagener Härtung | BSI 200-4, Business Continuity |

---

## 7. Prüfungs-Fertigsätze (Copy-Paste ready)

**„Nennen Sie 5 Härtungsmaßnahmen für einen Windows/Linux-Server.“**
1. Dienste/Features deinstallieren/deaktivieren (Minimalinstallation).
2. Host-Firewall: Default-Deny, explizite Allow-Regeln mit Quell-IP-Restriktion.
3. Patchmanagement: Automatisierte, getestete Updates (WSUS/unattended-upgrades).
4. Authentisierung: Standard-Accounts umbenennen/deaktivieren, MFA, LAPS/SSH-Keys.
5. Krypto-Hygiene: TLS 1.2+, SMB-Signing/Encryption, TLS-Zertifikate via ADCS/ACME.

**„Warum ist ein frisch installiertes OS nicht gehärtet?“**
Hersteller liefern General-Purpose-Defaults mit maximaler Kompatibilität (viele Dienste, offene Ports, Standard-PWs, alte Protokolle) → Angriffsfläche maximal, Compliance minimal.

**„Welche Rolle spielt die Host-Firewall bei offener Port-Liste?“**
Sie ist die **letzte Instanz** auf Layer 4: Selbst wenn ein Dienst unerwartet lauscht (Fehlkonfig, Malware), blockiert die Firewall den Netzwerkzugriff, sofern keine Allow-Regel existiert.

---

## 8. Nächste Schritte für dich
1. `ss -tulpen` / `Get-NetTCPConnection` auf deinem Test-System → **jeden** Eintrag begründen oder killen.
2. CIS Benchmark (PDF) für dein OS herunterladen → Checkliste abarbeiten.
3. `BIOS-UEFI.md` finalisieren (Secure Boot, TPM, Boot-PW, CSM off).
4. Logging auf zentralen Syslog (TLS, Port 6514) + `auditd` für `execve`, `openat` konfigurieren.
5. Patch-Zyklus definieren: **Critical ≤ 72h, High ≤ 7d, Medium ≤ 30d** – dokumentieren.

---

**Savage Realität**: Wenn du „Ports schließen“ schreibst, aber `firewall-cmd --list-all` noch `services: ssh dhcpv6-client` zeigt – hast du **nichts** gehärtet. Beweis führen oder löschen.
