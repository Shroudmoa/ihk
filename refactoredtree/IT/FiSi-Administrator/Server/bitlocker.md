# BitLocker und Datenträgerverschlüsselung (TPM)

## In 30 Sekunden

- **Data at Rest:** Gespeicherte Daten werden verschlüsselt. Wer Notebook, Festplatte oder USB-Stick stiehlt, kann ohne Schlüssel nichts lesen.
- **BitLocker** ist die Vollverschlüsselung von Windows (Pro/Enterprise/Server) mit **AES-XTS 128 oder 256 Bit**. **BitLocker To Go** verschlüsselt Wechseldatenträger.
- Der Schlüssel liegt geschützt im **TPM 2.0**. Das TPM gibt ihn nur frei, wenn der Bootvorgang unverändert ist (**Measured Boot**, PCR-Werte).
- Schutzstufen: **TPM only** (bequem), **TPM + PIN** (Pre-Boot-Authentifizierung, empfohlen für Notebooks), **TPM + USB-Schlüssel**.
- Ohne gesicherten **Wiederherstellungsschlüssel** (48 Ziffern in AD/Entra ID) sind die Daten bei einem Defekt verloren.
- Linux-Gegenstück: **LUKS** (dm-crypt), macOS: **FileVault**.
- Geprüft: OS-Sicherheitsmerkmale 2022S, Schutz bei Notebook-Diebstahl 2024S, Verschlüsselung externer Datenträger (AES vs. DES vs. RSA) 2023W.

## Kernwissen

### Bausteine

| Baustein | Aufgabe |
|---|---|
| **Volume Master Key (VMK)** / **Full Volume Encryption Key (FVEK)** | der eigentliche Datenschlüssel (FVEK) wird vom VMK geschützt, der VMK von den Schutzmechanismen (Protektoren) |
| **TPM 2.0** | Hardware-Chip (oder Firmware-TPM), speichert Schlüssel versiegelt, misst Firmware, Bootloader und Konfiguration in PCR-Registern |
| **Protektoren** | TPM, TPM + PIN, TPM + USB, Passwort (Datenlaufwerke), Wiederherstellungsschlüssel |
| **Secure Boot** | stellt sicher, dass nur signierter Bootcode läuft → [BIOS/UEFI](bios-uefi.md) |
| **Wiederherstellungsschlüssel** | 48-stelliger Notfallschlüssel, wird bei geänderter Hardware/Firmware, vergessener PIN oder Plattenumzug abgefragt |

### Ablauf beim Start

1. UEFI und Bootloader werden gemessen, die Messwerte landen in den PCRs des TPM.
2. Stimmen die Werte mit dem Zustand beim Versiegeln überein (und ist ggf. die PIN korrekt), gibt das TPM den VMK frei.
3. Windows entschlüsselt das Volume transparent im laufenden Betrieb.
4. Wurde etwas manipuliert (anderer Bootloader, Platte in fremdem Rechner), verweigert das TPM die Freigabe → Wiederherstellungsschlüssel nötig.

### Vergleich der Lösungen

| Lösung | Plattform | Besonderheit |
|---|---|---|
| **BitLocker** | Windows Pro/Enterprise/Server | TPM-Integration, zentrale Verwaltung per GPO/Intune, Schlüssel-Backup in AD/Entra ID |
| **Geräteverschlüsselung** | Windows Home (unterstützte Geräte) | vereinfachte BitLocker-Variante |
| **LUKS** | Linux | Standard für Vollverschlüsselung, TPM-Bindung z. B. mit `systemd-cryptenroll` |
| **FileVault** | macOS | Schlüssel in der Secure Enclave |
| **VeraCrypt** | plattformübergreifend | Open Source, Container und Partitionen |
| **Self-Encrypting Drive (SED)** | Hardware | Laufwerk verschlüsselt selbst (Opal), Vertrauen in die Firmware nötig |

### Warum AES? (2023W)

| Verfahren | Eignung für Datenträger |
|---|---|
| **AES-256** | symmetrisch, schnell, hardwarebeschleunigt (AES-NI), sicher → **erste Wahl** für große Datenmengen |
| **DES** | 56-Bit-Schlüssel, per Brute Force gebrochen → **nicht geeignet** |
| **RSA** | asymmetrisch, sehr langsam für große Datenmengen → nur für Schlüsselaustausch und Signaturen (hybride Verfahren) |

### Grenzen

- Schützt nur, wenn das Gerät **ausgeschaltet** ist oder im Ruhezustand (Hibernate). Im laufenden, entsperrten Betrieb sind die Daten lesbar.
- **TPM only** ohne PIN: Das Gerät startet bis zum Windows-Login automatisch. Bei hohem Schutzbedarf deshalb TPM + PIN.
- Kein Schutz vor Malware oder Ransomware im laufenden System, kein Ersatz für Backup.

## Prüfungsfrage

**Frage:** Nennen Sie zwei Maßnahmen, die die Unternehmensdaten bei Diebstahl eines Notebooks schützen, und erläutern Sie die Rolle des TPM.

**Musterantwort:** Erstens wird die Festplatte vollständig mit BitLocker (AES-256) verschlüsselt, mit Pre-Boot-Authentifizierung per TPM und PIN. Ohne PIN und ohne das passende TPM sind die Daten auch nach Ausbau der Festplatte unlesbar. Zweitens wird das Notebook per Mobile Device Management verwaltet, sodass es bei Verlust gesperrt oder aus der Ferne gelöscht werden kann. Das TPM ist ein Sicherheitschip, der den Schlüssel zum Entschlüsseln geschützt speichert. Es gibt ihn nur frei, wenn der gemessene Bootvorgang unverändert ist und die PIN stimmt. So kann ein Angreifer die Platte weder in einem anderen Rechner noch mit einem manipulierten Bootloader öffnen.

## Falle

- BitLocker mit Secure Boot verwechseln. Secure Boot prüft Signaturen, BitLocker verschlüsselt.
- Den Wiederherstellungsschlüssel auf dem verschlüsselten Laufwerk speichern.
- Verschlüsselung als Schutz im laufenden Betrieb verkaufen.
- RSA für die Verschlüsselung ganzer Datenträger vorschlagen.
- Ein BIOS-Passwort als gleichwertig darstellen. Ausgebaut ist die Platte ohne Verschlüsselung lesbar.

## Merksatz

**„AES verschlüsselt, das TPM bewacht den Schlüssel, die PIN öffnet die Tür, und der Recovery-Key liegt woanders.“**

## Verwandt

- [BIOS/UEFI – Firmware und Härtung](bios-uefi.md) – Secure Boot, TPM aktivieren
- [OS-Härtung](os-haertung.md)
- [Symmetrische und asymmetrische Verschlüsselung](../Sicherheit/Verschluesselung/symmetrische-und-asymmetrische.md)
- [MDM & BYOD](../Netzwerk-Administration/mdm-byod.md) – Fernlöschung
- [DSGVO-TOM](../../Management/DSGVO/dsgvo-tom.md) – Verschlüsselung als TOM (Art. 32)
- [OS-Sicherheitsmerkmale](../../nice-to-know/OS-sicherheitsmerkmale.md)
- [Lektion L27 PKI, TLS, Verschlüsselung](../../analyse-der-exams/L27_PKI_TLS_Verschluesselung_VPN.md)
