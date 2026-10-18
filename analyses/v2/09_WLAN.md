# 9. WLAN

*Quelle: GA2 Aufgabe 4 (2024 Sommer) und weitere Jahrgänge*

## 9.1 Verschlüsselung: WPA2 Personal (PSK) – Nachteile

Nenne 3:
- PSK kann an Dritte weitergegeben werden
- Anfällig für **Brute-Force**
- Bei Personalaustritt weiterhin Zugriff, bis PSK geändert wird
- Gestohlenes Endgerät = sofortiger Zugriff
- Datenverkehr abhörbar, wenn Schlüssel bekannt

**Verbesserung:**
- **WPA2/WPA3-Enterprise** (802.1X, individuelle Anmeldung/Zertifikate)
- oder WPA3 (Simultaneous Authentication of Equals)

## 9.2 Gast-/BYOD-Netz trennen

Sinn (nenne 3):
- Kompromittierung des internen Netzes durch unverwaltete Geräte verhindern
- Kein Zugriff auf interne Daten/Anwendungen
- Internetverkehr priorisieren/steuern
- Zeitlich begrenzter Zugang möglich

## 9.3 WLAN-Scan bewerten

Begriffe erklären können:
- **SSID:** Name des WLAN
- **BSSID:** MAC des Access Points
- **CHANNEL:** genutzter Kanal
- **BW[MHz]:** Kanalbreite (20/40/80)
- **SECURITY:** Verschlüsselung (WPA2-Personal, WPA2-Enterprise, Open…)
- **SIGNAL:** Empfangsstärke (dBm, z. B. -51 besser als -85)

**Performance-Problem:** Zwei Netze auf **demselben Kanal** (z. B. Kanal 1)
→ gegenseitige Störung.

**`<hidden>`:** SSID-Broadcast am AP deaktiviert (Name nicht sichtbar).

## 9.4 WLAN-Ticketsystem / Voucher (Gast-WLAN)

Gast erhält **Voucher** mit Zugangsdaten, gibt diese im **Captive Portal** ein. Möglichkeiten für Betreiber (nenne 3):
- Zeitliche Begrenzung
- Volumenbegrenzung
- Zustimmung zu Nutzungsbedingungen einholen
- Dienste freischalten (je nach Ticket)
- Werbe-/Infoseite anzeigen

### AAA (Authentication, Authorization, Accounting) – RADIUS

- **Authentication:** Überprüfung der Anmeldedaten am RADIUS-Server (z. B. Active Directory, LDAP)
- **Authorization:** Nach erfolgreichem Login wird das Zugangsniveau (SSID, VLAN, Rechte) zugewiesen
- **Accounting:** Detaillierte Protokollierung der Verbindungsdauer, verbrauchtes Volumen, Zugriffszeiten
- **Komponenten:** NAS (Access Point), RADIUS-Server, Client (Gerät)
- **Vorteil gegenüber PSK:** Zentrales User-Management, sofortiger Zugangsentzug bei Austritt, individuelle Profile

## 9.5 2,4 GHz vs. 5 GHz

Kameras (2,4 GHz) verbinden nicht nach AP-Tausch →
- Neue APs nur **5 GHz**
- Falsche Kanalwahl
- Reichweite/Signal zu schwach
- Falsche Konfiguration

## 9.6 Site Survey / Funkstellenuntersuchung

Eine **Site Survey** ist die physische Vermessung des Funkfeldes, um optimale AP-Positionen zu ermitteln. Typische Inhalte (2023W, 2024S):

- **Signalstärke-Karte:** Darstellung der Empfangsstärke an verschiedenen Orten
- **Kanalanalyse:** Ermittlung geeigneter Kanäle, um Störungen zu vermeiden
- **AP-Positionierung:** Optimale Höhe und Platzierung für flächendeckende Abdeckung
- **Interferenzquellen:** Identifikation anderer Funkquellen (Mikrowellen, Bluetooth, benachbarte WLANs)
- **Ergebnis:** Empfehlung für AP-Typen, Kanaleinstellung, Leistungsbereich

**Lerninhalte aus echten Prüfungen:**
- 2024S: WLAN-Scan auswerten (SSID/BSSID/Channel/BW/Security/Signal)
- 2023W: AAA/RADIUS als Enterprise-Alternative zu PSK
- 2022S: Gast-WLAN mit Voucher und Captive Portal

## Typische Prüfungsaufgabe

> „Erläutern Sie drei Begriffe aus dem WLAN-Scan."

→ Immer **Abkürzung + Bedeutung + (Bezug zur Ausgabe)**.

> „Nennen Sie drei Vorteile von WLAN-Enterprise gegenüber WLAN-PSK."

→ **Zentrales User-Management, sofortiger Zugangsentzug, individuelle Profile durch RADIUS-Authentifizierung, keine PSK-Weitergabe möglich.**

## Selbstcheck – kannst du das?

- [ ] Ich kann drei Begriffe aus dem WLAN-Scan erklären (SSID, BSSID, Channel, BW, Security, Signal).
- [ ] Ich unterscheide WPA2-Personal (PSK) und WPA2/WPA3-Enterprise (802.1X/RADIUS).
- [ ] Ich nenne drei Nachteile von WPA2-Personal.
- [ ] Ich erkläre, was bei einer Site Survey / Funkstellenuntersuchung zu beachten ist.
- [ ] Ich kann Voucher-Einlösungsprozesse für Gast-WLAN beschreiben.

## Weiterführende Ressourcen

- IHK-Prüfungsaufgaben AP2 zu Netzdiensten
- Cisco: WLAN Konfiguration und Site Survey
- RFC 2865 (RADIUS)