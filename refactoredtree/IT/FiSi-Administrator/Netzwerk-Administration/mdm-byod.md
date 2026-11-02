# MDM und BYOD

## In 30 Sekunden

- **MDM** (Mobile Device Management) verwaltet und sichert Smartphones, Tablets und Notebooks **zentral**: Richtlinien, Apps, Verschlüsselung, **Fernsperre (Lock)** und **Fernlöschung (Wipe)**.
- **BYOD** (Bring Your Own Device) = private Geräte werden dienstlich genutzt – Risiko für Firmendaten **und** für die Privatsphäre des Mitarbeiters.
- Schutz privater Daten: **Containerisierung** (Work Profile) und **Selective Wipe** – das MDM verwaltet und löscht nur den Firmenbereich (Prüfung 2025S).
- Bester Schutz bei Verlust/Diebstahl: Geräteverschlüsselung + Sperrcode, dazu **Remote Lock und Remote Wipe**.
- BYOD und MDM brauchen klare Regeln – im Betrieb mit Betriebsrat eine **Betriebsvereinbarung** (Mitbestimmung bei technischer Überwachung, § 87 Abs. 1 Nr. 6 BetrVG).

## Kernwissen

### BYOD (Bring Your Own Device)

**BYOD** bedeutet, dass Mitarbeitende private Endgeräte (Smartphones, Tablets, Laptops) für den dienstlichen Einsatz nutzen.

#### Risiken bei BYOD

- Verlust/Diebstahl des Geräts
- Unbefugter Zugriff auf Firmendaten
- Vermischung privater & geschäftlicher Daten
- Malware-Infektionen
- Schwierigere Kontrolle & Compliance

#### Gegenmaßnahmen

- **Containerisierung/Work Profile:** Trennung von privatem und dienstlichem Bereich
- **MDM-Lösung:** Zentrale Verwaltung
- **VPN-Zugriff:** Verschlüsselter Zugriff aufs Unternehmensnetz
- **Passwortrichtlinien & MFA**
- **Remote Wipe / Selective Wipe**

### MDM (Mobile Device Management)

**MDM** ist eine zentrale Lösung zur Verwaltung, Konfiguration, Überwachung und Absicherung mobiler Endgeräte (iOS, Android, Windows).

#### Aufgaben von MDM

- Gerätekonfiguration & Richtlinienvergabe
- App-Verteilung (Managed Apps)
- Sicherheitsrichtlinien erzwingen
- Verschlüsselung erzwingen
- Remote-Lock & Remote-Wipe
- Geräteinventarisierung

### COPE, CYOD, BYOD im Vergleich

| Konzept | Bedeutung | Erklärung |
|---|---|---|
| **BYOD** | Bring Your Own Device | Privates Gerät wird dienstlich genutzt |
| **CYOD** | Choose Your Own Device | Mitarbeitende wählen aus vorgegebener Geräteauswahl |
| **COPE** | Corporate Owned, Personally Enabled | Firma stellt Gerät, aber private Nutzung teilweise erlaubt |

### Containerisierung und Selective Wipe

| Konzept | Erklärung |
|---|---|
| **Containerisierung / Work Profile** | Trennung in „Business-Container“ und privaten Bereich. Nur der Container wird verwaltet |
| **Full Wipe** | Komplette Löschung des gesamten Geräts (z. B. bei Diebstahl) |
| **Selective Wipe** | Nur Löschung des **dienstlichen Containers/Profiles**. **Private Daten bleiben erhalten** |
| **Compliance Enforcement** | Erzwingung von Passcode, Verschlüsselung, Jailbreak/Root-Erkennung |

### Inhalte einer BYOD-/MDM-Betriebsvereinbarung (Prüfung 2025S)

- zulässige Geräte und Mindest-Betriebssystemversionen
- Umfang der privaten Nutzung, Kostenübernahme (Gerät, Datenvolumen)
- welche Aktionen das MDM ausführen darf (Wipe, Ortung, App-Installation) und welche Daten es **nicht** einsehen darf
- Verbot der Leistungs- und Verhaltenskontrolle
- Pflichten bei Verlust oder Diebstahl (sofortige Meldung, Sperrung, Selective Wipe)
- Vorgehen beim Ausscheiden (Firmencontainer löschen, Zugänge sperren)
- Sicherheitsvorgaben (Sperrcode, Updates, kein Jailbreak/Root)

### Auswahl eines MDM-Systems

Wichtige Kriterien: unterstützte Plattformen (iOS, Android, Windows, macOS), Remote Lock/Wipe und **Selective Wipe**, Containerisierung, App-Verteilung, Compliance-Prüfung (Root/Jailbreak-Erkennung), Anbindung an AD/Entra ID, Serverstandort und AVV, Lizenzmodell (pro Benutzer oder pro Gerät – siehe CAL). Bekannte Produkte: Microsoft Intune, Jamf (Apple), Workspace ONE.

### Kurz gemerkt

- **MDM = zentrale Verwaltung mobiler Geräte**
- **BYOD = privates Gerät im Einsatz**
- **Selective Wipe** ist datenschutzfreundlicher als Full Wipe
- **Container/Work Profile** trennt private & geschäftliche Daten
- **Ziel: Sicherheit + Datenschutz (Gerät gehört dem MA, aber Firmendaten geschützt)**

## Prüfungsfrage

**Frage:** Mitarbeiter sollen ihre privaten Smartphones für dienstliche E-Mails nutzen. Wie stellt ein MDM sicher, dass Firmendaten bei Verlust geschützt sind und private Daten des Mitarbeiters trotzdem unangetastet bleiben?

**Musterantwort:** Das MDM richtet auf dem Gerät einen getrennten, verschlüsselten Firmencontainer bzw. ein Arbeitsprofil ein, in dem Mail-App und Firmendaten liegen; nur diesen Bereich verwaltet das MDM. Es erzwingt einen Sperrcode und die Geräteverschlüsselung. Geht das Gerät verloren, kann die IT es aus der Ferne sperren und per Selective Wipe nur den Firmencontainer löschen, während private Fotos und Apps erhalten bleiben. Die Regeln dazu, insbesondere welche Daten das MDM nicht einsehen darf, werden in einer Betriebsvereinbarung festgehalten.

## Falle

- Bei BYOD einen **Full Wipe** empfehlen → löscht private Daten des Mitarbeiters; richtig ist **Selective Wipe**.
- MDM ohne Datenschutz- und Mitbestimmungshinweis einführen – Ortung und App-Inventar sind Überwachungstechnik.
- Remote Wipe ohne Verschlüsselung als ausreichend sehen → ein offline gehaltenes Gerät erhält den Löschbefehl nie.

## Merksatz

**„Firmendaten in den Container, private Daten bleiben privat – gelöscht wird selektiv.“**

## Verwandt

- [Authentifizierung, Autorisierung und MFA](../Server/authentifizierung-autoritisierung-mfa.md)
- [MFA](../Sicherheit/MFA.md)
- [Least Privilege](../Sicherheit/least-privilege.md)
- [Zero Trust](../Sicherheit/zero-trust.md) – Gerätezustand prüfen
- [OS-Härtung](../Server/os-haertung.md)
- [Patchmanagement](patchmanagement-tu-su-pu.md) – Update vs. Upgrade
- [DSGVO-Grundlagen](../../Management/DSGVO/dsgvo-tom.md)
- [Betriebsrat und JAV](../../../Wirtschaft/betriebsrat-jav.md) – Betriebsvereinbarung
- [VPN](../../nice-to-know/VPN/readme.md)
- [Lektion 28 GA1-Praxisthemen (MDM)](../../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md)
- [Lektion 10 Lizenzierung (CAL)](../../analyse-der-exams/10_Lizenzierung.md)
