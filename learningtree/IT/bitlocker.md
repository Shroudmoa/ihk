# Bitlocker

### 1. Data‑at‑Rest (DAR) Grundlagen
- **Definition** – AES‑XTS‑128 / AES‑GCM auf Blockgeräteebene.
- **Zweck** – Selbst wenn SSD/HD entnommen wird, sind die Inhalte ohne Schlüssel unlesbar.
- **Wichtigster Punkt** – Der Schlüssel darf niemals auf der Storage selbst verbleiben (DSGVO Art. 32).

### 2. TPM (Trusted Platform Module) – Hardware Root of Trust
- **Definition** – TPM 2.0-Chip auf dem Mainboard, physisch mit dem System verknüpft.
- **Key‑Binding** – Verschlüsselungsschlüssel werden niemals den Chip verlassen; sie werden mit `TPM2_Seal` verknüpft.
- **Boot‑Integrity** – PCR (Platform Configuration Registers) messen Bootloader, Firmware, Kernel → bei Manipulation verweigert TPM Schlüsselfreigabe.

> **Drei‑Zeilen‑Explanationsstil**
> 1️⃣ TPM speichert einen Schlüssel, der mit der Hardware verknüpft ist. 2️⃣ Es überprüft den Bootvorgang und verweigert Zugriff, wenn sich etwas geändert hat. 3️⃣ Ohne TPM gibt es keinen Hardware-basierten Schutz gegen Datenklau.

**Pros**
- Schlüsselspeicherung ist hardwareverifiziert.
- Gemessener Bootvorgang (Vertrauenswürdigkeitsprüfung).

**Cons**
- Portabilitätsprobleme: Wechsel der Hardware → TPM kann einen wichtigen Schlüssel verlieren.
- Kein Alleingangslösung; benötigt OS-weiten Support (BitLocker, LUKS).

### 3. BitLocker – Microsofts Lösung
- **Definition** – AES‑128/256‑XTS‑Verschlüsselung für Windows-System- und-Volume-Verschlüsselung.
- **Integration** – BitLocker kann TPM 2.0 als „Key‑Verbindungs“-Mechanismuse verwenden (`TPM‑Only`, `TPM‑Pin`, `TPM‑USB`).
- **Recovery** – Wiederherstellungs‑Schlüssel in Azure AD, AD‑Userrichtlinie oder einem Passwort‑Manager gespeichert.

> **Drei‑Zeilen‑Explanationsstil**
> 1️⃣ BitLocker verschlüsselt die gesamte Festplatte und umfasst Systemdateien. 2️⃣ Es arbeitet mit TPM zusammen, um den Chiffrierschlüssel auf dem Chip zu speichern; Fallback: Passwort/USB/Recovery-Schlüssel. 3️⃣ Es erfüllt DSGVO-„Verschlüsselungs“-Anforderungen, wenn der Wiederherstellungs-Schlüssel sicher aufbewahrt wird.

**Pros**
- Ein-Klick‑Integration für Windows‑Desktops/Laptops.
- Einfache Verwaltung über Group Policy / Azure AD.

**Cons**
- Windows-zentriert; unsupported auf Linux/macOS.
- Nur eine „Besitz“-Faktor; ohne TPM ist die Sicherheit ein reines Passwort (Schwachpunkt).

### 4. OS‑unabhängige Alternativen
| Lösung | Plattform | Verschlüsselung | Anbindung von Schlüsseln |
|----------|----------|-----------|----------------|
| **LUKS + dm‑crypt** | Linux | AES‑XTS‑128/256 | TPM (tpm2‑luksmapper), Passwort, Schlüsseldatei |
| **FileVault** | macOS | AES‑128‑XTS | ICloud‑Keychain, TPM (optional über FileVault‑TPM) |
| **VeraCrypt** | Cross‑platform | AES‑XTS, Serpent | Passwort, Schlüsseldatei, TPM (über `tcplay`) |
| **Cryptsetup + LUKS2** | Linux | AES‑GCM‑256 (Option) | TPM, YubiKey, HSM |

> **Drei‑Zeilen‑Explanationsstil**
> 1️⃣ LUKS2 speichert einen Chiffrierschlüssel in einer Schlüsseldatei. 2️⃣ `tpm2‑luksmapper` bindet den Schlüssel an TPM, sodass der Schlüssel nie den Chip verlässt. 3️⃣ Die Verwendung von LUKS2 + TPM ist der Linux-Äquivalent von BitLocker + TPM.

**Pros**
- Offen, unabhängig, aufwendigere Schlüsselverwaltung.
- Portabel zwischen Distributionen, Container-Images.

**Cons**
- Einrichtung aufwendiger (`cryptsetup`, LUKS‑Header-Sicherung).
- Geringere Unterstützung durch GUI (im Vergleich zu BitLocker).

### 5. Erweiterte Schlüsselmanagement-Hardware
| Gerät | Typ | Hauptsicherheit |
|--------|------|-----------------|
| **Intel SGX** | CPU-Trust-Domain | Schlüsselspeicherung im Enklave-Speicher (nicht lesbar vom OS) |
| **HSM (z. B. SafeNet)** | Extern, PCI‑e | Schlüsselerzeugung auf Chip, Audit-Protokolle (Syslog/514) |
| **YubiKey 5 NFC / Nitrokey** | USB‑Key | TPM-ähnliche Schlüsselgenerierung, FIDO2 + OTP |

> **Drei‑Zeilen‑Explanationsstil**
> 1️⃣ SGX erstellt eine geschützte Enklave, die den Schlüssel physisch isoliert. 2️⃣ HSMs sind dedizierte Kryptoprozessoren mit strengen Compliance-Audit-Protokollen. 3️⃣ Hardware-Sicherheitsschlüssel bieten einen „Besitz“-Faktor, der viele TPM-Schwachstellen (Portabilität) abdeckt.

### 6. Boot‑Integrity & Measured Boot (TPM + Secure‑Boot)
- **Secure Boot (UEFI)** – Verhindert das Laden nicht signierter Bootloader (Schicht 3‑4).
- **Measured Boot** – Jeder Schritt (Firmware → OS) füllt einen TPM‑PCR; Revisionsliste wird in SIEM (Syslog/514) gespeichert.
- **Linux IMA/EVM** – Integritätsprüfung von geladenen Modulen (Linux‑only, auch mit TPM-PCRs).

> **Drei‑Zeilen‑Explanationsstil**
> 1️⃣ Secure Boot stellt sicher, dass nur Microsoft- oder OEM-Signierte einsteigen. 2️⃣ Gemessener Bootvorgang protokolliert Änderungen in TPM‑PCRs, die mit Syslog/514 exportiert werden können. 3️⃣ Integriert mit DSGVO-„Nachverfolgbarkeit“-Anforderungen.

### 7. DSGVO-Konformität – Checkliste
| Anforderung | Tech-Lösung | Überwachung (Port) |
|-------------|---------------|-------------------|
| Verschlüsselung bei Diebstahl | BitLocker/LUKS/TPM | – |
| Schlüsselrotation | KMS (AWS/Azure) | HTTPS 443, SSH 22 |
| Audit Trail | Syslog (514) / SIEM | Syslog UDP/TCP 514 |
| Zugriffskontrolle | MFA (OTP, YubiKey) | Radius/TACACS+ (1812/1813) |
| Datenlöschung | Cryptographic Erase (Schließung des Schlüssels) | Verwaltungsprotokoll (SNMP 161) |

> **Drei‑Zeilen‑Explanationsstil**
> 1️⃣ Verschlüsselung entspricht Art. 32 „Pseudonymisierung und Verschlüsselung“. 2️⃣ Schlüsselrotation und sichere Speicherung bieten „Rechenschaftspflicht“ (Rechenschaftspflicht). 3️⃣ Syslog/514-Protokollierung stellt sicher, dass jede Schlüsselverwendung aufgezeichnet wird (DSGVO Art. 5(2)).

### 8. Netzwerkseitig – Ports & Protokolle für den sicheren Fernzugriff
| Dienst | Port | Protokoll | Warum es für IHK relevant ist |
|---------|------|----------|--------------------------|
| SSH | 22 | TCP | Authentifizierte Remote-Administration, Schlüsselübertragung |
| VPN (IPsec) | 500 / 4500 | UDP | Verschlüsselte Tunnel (Layer 3) |
| WireGuard | 51820 | UDP | Leichtgewichtig, modernes VPN (IPsec-Äquivalent) |
| TLS/HTTPS | 443 / 8443 | TCP | Webbasierte Verwaltung, Zertifikatsauthentifizierung |
| Syslog | 514 | UDP/TCP | Zentrale Protokollierung (DSGVO-Audit-Trail) |
| SNMP v3 | 161 / 162 | UDP | Netzwerküberwachung, nur mit Authentifizierung |
| RDP | 3389 | TCP | Windows-Remote-Desktop (sicher mit NLA) |
| SMB | 445 | TCP | Dateifreigabe (erfordert Verschlüsselung – SMB‑3) |

> **Drei‑Zeilen‑Explanationsstil**
> 1️⃣ SSH verschlüsselt die Kommandozeile und Transport-Schlüssel (Port 22). 2️⃣ IPsec Tunnel (Ports 500/4500) sichern Layer‑3-Kommunikation für VPN. 3️⃣ Syslog (514) sichert DSGVO-konforme Protokollierung durch sichere Weiterleitung.

### 9. OSI‑Mapping der oben genannten Sicherheitsmaßnahmen
| Schicht | Haupttechnologie | Beispiel-Port |
|-------|----------------|---------------|
| 2 (Datenlink) | VLAN‑Tagging, MAC‑Filtering | – |
| 3 (Netzwerk) | IPsec, WireGuard | 500/4500, 51820 |
| 4 (Transport) | TCP/UDP-Firewall, NAT | 22, 443, 514 |
| 5 (Session) | TLS/SSL | 443, 8443 |
| 6 (Präsentation) | AES‑GCM‑Verschlüsselung (BitLocker/LUKS) | – |
| 7 (Anwendung) | BitLocker-Dienst, LUKS-cryptsetup | – |

> **Drei‑Zeilen‑Explanationsstil**
> 1️⃣ Layer 2 begrenzt den broadcast‑Domänenbereich (VLAN). 2️⃣ Layer 3 leitet verschlüsselte Tunnel (IPsec) um. 3️⃣ Layer 6 kodiert Daten für die Speicherung (AES‑XTS), Layer 7 bietet den OS‑Dienst.

### 10. Kurze Zusammenfassung für die IHK-Prüfung („Double‑Bird“)
- **BitLocker + TPM** = Windows-native, hardwaregebundene Verschlüsselung (WKP 22 / 443 für Wiederherstellung).
- **LUKS + tpm2‑luksmapper** = Linux‑äquivalent mit gleicher hardwarebasierter Schlüsselspeicherung.
- **DSGVO** → Verschlüsselung + sichere Schlüsselspeicherung (Art. 32) + Audit-Protokolle über Syslog/514 (Art. 5/6).
- **Netzwerksicherheit** → Verschlüsselter Remotezugriff (SSH 22, WireGuard 51820, TLS 443) und Überwachung (SNMP 161/162, Syslog 514).

**Zusammenfassung:**
Wenn ein Azubi fragt „Was verhindert Diebstahl von Daten von einem ausgeklemmten Laptop?“, antworten Sie: **Verschlüsselung (BitLocker/LUKS) + hardwarebasierter Schlüssel (TPM/HSM) + sicherer Fernzugriff (SSH/VPN) + auditierte Protokollierung (Syslog/514)**. Das umfasst alle IHK-bezogenen Themen in einem Satz.
