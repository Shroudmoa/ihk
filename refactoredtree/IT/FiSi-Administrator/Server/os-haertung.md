# OS-Härtung (Betriebssystem absichern)

## In 30 Sekunden

- **Härtung** heißt: die **Angriffsfläche minimieren**. Was nicht läuft, nicht installiert ist und nicht erreichbar ist, kann nicht angegriffen werden.
- Kernmaßnahmen: **unnötige Dienste und Software entfernen**, **Standardkonten und -passwörter ändern**, **Updates einspielen**, **Host-Firewall mit Default-Deny**, **Least Privilege**, **unsichere Protokolle abschalten** (SMBv1, Telnet, TLS 1.0/1.1), **Protokollierung**.
- Dazu gehört die Firmware: UEFI-Passwort, Secure Boot, kein Boot von USB → [BIOS/UEFI](bios-uefi.md).
- Ein frisch installiertes OS ist **nicht** gehärtet: Es ist auf Kompatibilität ausgelegt (viele Dienste, Standardkonten, alte Protokolle).
- Geprüft: 2022S, 2023S (Serverkonfiguration in 4 Bereichen), 2025S, 2026S. Ein wiederkehrendes Thema.

## Kernwissen

### Härtungsmaßnahmen

| Bereich | Maßnahme | Beispiel / Werkzeug |
|---|---|---|
| **Dienste** | nicht benötigte Dienste stoppen und deaktivieren | `systemctl disable --now cups`, Windows: Dienst „Druckwarteschlange“ auf einem Server ohne Druckfunktion deaktivieren |
| **Software/Rollen** | Minimalinstallation, nur benötigte Rollen und Features (Server Core) | `apt purge`, Server-Manager |
| **Konten** | Standardkonten umbenennen/deaktivieren, starke Passwortrichtlinie, Kontosperrung nach Fehlversuchen, MFA für Admins | `passwd -l root`, LAPS, Account-Lockout-Policy |
| **Rechte** | **Least Privilege**, getrennte Admin- und Benutzerkonten, Dienstkonten mit minimalen Rechten | Tier-Modell, `sudo` statt root |
| **Updates** | Patchmanagement: testen, zeitnah verteilen, Erfolg prüfen | WSUS, Intune, `unattended-upgrades` |
| **Netzwerk** | Host-Firewall mit Default-Deny, nur nötige Ports, Fernwartung nur über VPN/Jump-Host | Windows Defender Firewall, `nftables`, `ufw` |
| **Protokolle** | unsichere Protokolle abschalten | SMBv1, Telnet, FTP, TLS 1.0/1.1, NTLMv1 |
| **Ausführung** | nur erlaubte Programme starten | AppLocker, SELinux, AppArmor |
| **Protokollierung** | Anmeldungen und Änderungen loggen, zentral sammeln | Ereignisanzeige, `auditd`, Syslog → SIEM |
| **Firmware** | UEFI-Passwort, Secure Boot, CSM aus, Boot von externen Medien sperren | → [BIOS/UEFI](bios-uefi.md) |
| **Daten** | Datenträger verschlüsseln | BitLocker mit TPM, LUKS |
| **Prüfen** | Ergebnis kontrollieren | `ss -tulpn` / `netstat -ano` (offene Ports), Schwachstellenscan (OpenVAS, Nessus), CIS-Benchmark |

### Serverkonfiguration in Bereichen (Muster 2023S)

| Bereich | zwei Beispiel-Einstellungen |
|---|---|
| Server-Hardware/Firmware | UEFI-Passwort setzen, Boot von USB sperren, Secure Boot aktivieren |
| Betriebssystem | nur benötigte Rollen installieren, Updates sofort einspielen |
| Dienste und Features | unnötige Dienste deaktivieren, nur benötigte Ports in der Firewall öffnen |
| Anmelden am Server | Kontosperrung nach 5 Fehlversuchen, Passwortkomplexität, MFA für Admins |
| Administration | Fernwartung nur über VPN/Jump-Host, Admin- und Alltagskonto trennen, Audit-Logging |

### Warum ein frisches OS unsicher ist

1. **Allzweck-Voreinstellungen:** Dienste wie Druck, Remote-Verwaltung und Netzwerkerkennung sind aktiv, damit alles sofort funktioniert.
2. **Standardzugänge:** bekannte Kontonamen (Administrator, root), Standardpasswörter bei Appliances, SNMP-Community „public“.
3. **Kompatibilität vor Sicherheit:** alte Protokolle und schwache Verschlüsselung bleiben teils aktiv.
4. **Patchstand:** Das Installationsmedium ist oft Monate alt.

### Defense in Depth

Härtung ist eine Schicht von mehreren: Netzwerksegmentierung (VLAN, DMZ), zentrale Firewall, Monitoring/SIEM, Backups mit Restore-Test. Fällt eine Schicht, greift die nächste.

## Prüfungsfrage

**Frage:** Ein neuer Windows-Server soll vor der Inbetriebnahme gehärtet werden. Nennen und erläutern Sie zwei Maßnahmen.

**Musterantwort:** Erstens werden alle nicht benötigten Dienste, Rollen und Programme deaktiviert bzw. deinstalliert, z. B. die Druckwarteschlange auf einem reinen Datenbankserver oder das veraltete SMBv1-Protokoll. Jeder laufende Dienst ist ein möglicher Angriffspunkt, das Entfernen verkleinert die Angriffsfläche und den Patchaufwand. Zweitens wird das Standard-Administratorkonto umbenannt oder deaktiviert, und für die Verwaltung wird ein persönliches Admin-Konto mit starkem Passwort und MFA genutzt, getrennt vom Alltagskonto. Dazu kommt eine Kontosperrung nach mehreren Fehlversuchen. Das erschwert Brute-Force-Angriffe auf bekannte Kontonamen und setzt das Least-Privilege-Prinzip um.

## Falle

- Nur „Firewall einschalten“ nennen. Sinnvoll ist Default-Deny **plus** nur benötigte Ports.
- Maßnahme nennen, aber **nicht begründen**. Gefragt ist fast immer „nennen **und erläutern**“.
- Virenscanner als einzige Härtung. Er erkennt Bekanntes, die Angriffsfläche bleibt.
- Härtung einmalig sehen. Neue Software und Updates können Einstellungen zurücksetzen, deshalb regelmäßig prüfen (Baseline, Scan).
- Admin-Rechte für den täglichen Gebrauch behalten „weil es bequemer ist“.

## Merksatz

**„Weg, was nicht gebraucht wird. Zu, was nicht offen sein muss. Aktuell, was bleibt. Protokolliert, was passiert.“**

## Verwandt

- [BIOS/UEFI – Firmware und Härtung](bios-uefi.md)
- [BitLocker](bitlocker.md) – Datenträgerverschlüsselung mit TPM
- [Linux-Grundlagen](linux-grundlagen.md)
- [Monitoring](monitoring.md) – Überwachung und Logs
- [Least Privilege](../Sicherheit/least-privilege.md)
- [Patchmanagement](../Netzwerk-Administration/patchmanagement-tu-su-pu.md)
- [Firewall](../Sicherheit/firewall/firewall.md)
- [AAA & SIEM](../Sicherheit/aaa-siem.md)
- [OS-Sicherheitsmerkmale](../../nice-to-know/OS-sicherheitsmerkmale.md)
- [Lektion L28 GA1 Praxisthemen](../../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md)
