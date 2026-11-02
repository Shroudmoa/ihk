# Betriebssystem-Sicherheitsmerkmale

## Die 3 wichtigsten Merkmale für die Tabelle

| Sicherheitsmerkmal | Erläuterung |
| :--- | :--- |
| **Benutzerkontensteuerung (UAC / Rechteverwaltung)** | Vergabe von Lese-/Schreibrechten basierend auf Benutzerrollen (RBAC). Verhindert, dass normale Nutzer Systemänderungen vornehmen oder sensible Daten anderer Nutzer einsehen. |
| **Verschlüsselung (Festplatten-/Dateisystem-Ebene)** | Verschlüsselung von Datenträgern (Data-at-Rest). Schützt Daten bei physischem Diebstahl des Servers oder der Festplatten (z.B. BitLocker, LUKS). |
| **Zentrale Protokollierung (Auditing/Logging)** | Aufzeichnung von Systemereignissen, Anmeldeversuchen und Änderungen. Ermöglicht die forensische Nachverfolgung von Sicherheitsvorfällen und die Erkennung von Angriffen. |

*(Alternative Merkmale falls du variieren willst: Selinux/AppArmor (Zugriffskontrolle), Windows Defender/ClamAV (Malwareschutz), Secure Boot (Manipulationsschutz beim Start).)*

---

## Realistische OS-Beispiele (für Prüfungs-Kontext)

### 1. Windows Server (2022 / 2025)
*   **UAC (User Account Control):** Verhindert, dass Anwendungen ohne explizite Administratorbestätigung Systemänderungen vornehmen.
*   **BitLocker:** Vollverschlüsselung der System- und Datenfestplatten (TPM-Chip nötig).
*   **Windows Defender:** Integrierter Virenscanner und Echtzeitschutz.
*   **Windows Firewall:** Eingehende und ausgehende Regelüberwachung.
*   **SMB-Signing:** Verhindert Man-in-the-Middle-Angriffe auf Netzwerkfreigaben (wichtig für Fileserver).
*   **Active Directory (AD):** Zentrale Benutzer- und Rechteverwaltung für ganze Domänen.

### 2. Linux (Ubuntu Server / Debian / RHEL)
*   **Rechteverwaltung (chmod / chown):** Strikte Trennung von Root-Rechten und normalen User-Rechten (sudo).
*   **LUKS:** Standard-Verschlüsselung für Linux-Festplatten.
*   **SELinux / AppArmor:** Mandatory Access Control (MAC). Selbst wenn ein Dienst gehackt wird, kann er nicht auf Dateien außerhalb seiner Sandbox zugreifen.
*   **iptables / nftables / ufw:** Integrierte Paketfilter (Firewall) im Kernel (Netfilter).
*   **Auditd:** System-Audit-Daemon zur Protokollierung von Systemaufrufen und sicherheitsrelevanten Ereignissen.

### 3. macOS (wenn der Fileserver ein Mac ist)
*   **FileVault:** Festplattenverschlüsselung.
*   **XProtect / Gatekeeper:** Integrierte Malware-Erkennung und Blockierung nicht-signierter Software.
*   **SIP (System Integrity Protection):** Verhindert, dass selbst der Root-User bestimmte Systemdateien ändert.




----------------------------
## 2023 Sommer wurde verlangt 


# IHK-Perspektive: Sicherheitsmaßnahmen auf einem Betriebssystem
## 0. Server Hardware 

*   Reihnfolge von boot
*   Secure-boot
+   Adminpassword auf UEFI



## 1. Grundlegende Betriebssysteminstallation und -konfiguration
*   Servernamen anonymisieren (keine Rückschlüsse auf Zweck/Standort).
*   System-Updates planen und automatisieren.
*   Default-Konten umbenennen oder deaktivieren (z.B. "Administrator").
*   Firewall-Regeln einschränken (Whitelisting statt Blacklisting).
*   Partitionieren und Formatieren der Datenträger (getrennte System-/Datenpartitionen).

## 2. Dienste / Features des Servers
*   Nicht benötigte Dienste deaktivieren (Reduktion der Angriffsfläche).
*   Auto-Start-Programme selektieren und deaktivieren.
*   Nur sichere Programme zulassen und installieren.
*   Unsichere Protokolle abschalten (z.B. Telnet, FTP, SMBv1).
*   Sperren von bestimmten Skripten (z.B. PowerShell/Shell-Skripte einschränken).

## 3. Anmelden am Server
*   Sichere Passwortregeln festlegen (Länge, Komplexität, Ablauf).
*   Anmeldezeiten einschränken (z.B. nur zu Geschäftszeiten).
*   Anmelden nur mit 2FA/MFA zulassen (Zwei-Faktor-Authentifizierung).
*   Remote-Verwaltung / -Zugriff einschränken (z.B. per VPN oder IP-Whitelist).

## 4. Administrieren des Servers
*   Berechtigungen vergeben für:
    *   Ändern von Systemeinstellungen
    *   Installieren von Anwendungen
    *   Zugriff auf Verzeichnisse
    *   Zugriff auf Systemressourcen
    *   Systemeinstellungen sichern
*   Logs maximale Einstellungen aufzeichnen lassen (Auditing/Logging).
