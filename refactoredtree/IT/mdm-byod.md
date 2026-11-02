# MDM, BYOD & Mobile Device Management

## BYOD (Bring Your Own Device)

**BYOD** bedeutet, dass Mitarbeitende private Endgeräte (Smartphones, Tablets, Laptops) für den dienstlichen Einsatz nutzen.

### Risiken bei BYOD

- Verlust/Diebstahl des Geräts
- Unbefugter Zugriff auf Firmendaten
- Vermischung privater & geschäftlicher Daten
- Malware-Infektionen
- Schwierigere Kontrolle & Compliance

### Gegenmaßnahmen

- **Containerisierung/Work Profile:** Trennung von privatem und dienstlichem Bereich
- **MDM-Lösung:** Zentrale Verwaltung
- **VPN-Zugriff:** Verschlüsselter Zugriff aufs Unternehmensnetz
- **Passwortrichtlinien & MFA**
- **Remote Wipe / Selective Wipe**

## MDM (Mobile Device Management)

**MDM** ist eine zentrale Lösung zur Verwaltung, Konfiguration, Überwachung und Absicherung mobiler Endgeräte (iOS, Android, Windows).

### Aufgaben von MDM

- Gerätekonfiguration & Richtlinienvergabe
- App-Verteilung (Managed Apps)
- Sicherheitsrichtlinien erzwingen
- Verschlüsselung erzwingen
- Remote-Lock & Remote-Wipe
- Geräteinventarisierung

## COPE, CYOD, BYOD im Vergleich

| Konzept | Bedeutung | Erklärung |
|---|---|---|
| **BYOD** | Bring Your Own Device | Privates Gerät wird dienstlich genutzt |
| **CYOD** | Choose Your Own Device | Mitarbeitende wählen aus vorgegebener Geräteauswahl |
| **COPE** | Corporate Owned, Personally Enabled | Firma stellt Gerät, aber private Nutzung teilweise erlaubt |

## Containerisierung & Selective Wipe

| Konzept | Erklärung |
|---|---|
| **Containerisierung / Work Profile** | Trennung in „Business-Container“ und privaten Bereich. Nur der Container wird verwaltet |
| **Full Wipe** | Komplette Löschung des gesamten Geräts (z. B. bei Diebstahl) |
| **Selective Wipe** | Nur Löschung des **dienstlichen Containers/Profiles**. **Private Daten bleiben erhalten** |
| **Compliance Enforcement** | Erzwingung von Passcode, Verschlüsselung, Jailbreak/Root-Erkennung |

## IHK-Merksätze

- **MDM = zentrale Verwaltung mobiler Geräte**
- **BYOD = privates Gerät im Einsatz**
- **Selective Wipe** ist datenschutzfreundlicher als Full Wipe
- **Container/Work Profile** trennt private & geschäftliche Daten
- **Ziel: Sicherheit + Datenschutz (Gerät gehört dem MA, aber Firmendaten geschützt)**

## Verwandt

- [Authentifizierung-Autoritisierung-MFA](authentifizierung-autoritisierung-mfa.md)
- [Least Privilege](least-privlage.md)
- [OS-Härtung](os-haertung.md)
- [DSGVO & TOM](dsgvo-tom.md)
- [VPN](vpn.md)
