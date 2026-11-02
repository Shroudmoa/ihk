# WLAN (IEEE 802.11)

## In 30 Sekunden

- WLAN ist ein **geteiltes Medium (Halbduplex)**: Alle Clients einer Funkzelle teilen sich die Bandbreite – real kommt etwa die Hälfte der Bruttorate an.
- **2,4 GHz** = mehr Reichweite, aber nur **3 überlappungsfreie Kanäle (1, 6, 11)** und viele Störer; **5/6 GHz** = mehr Kanäle und Durchsatz, weniger Reichweite.
- Sicherheit: **WPA2/WPA3-Personal** (ein gemeinsamer Schlüssel/PSK) vs. **WPA2/WPA3-Enterprise** (802.1X + RADIUS, individuelle Anmeldung).
- Gäste/BYOD gehören in ein **eigenes SSID + VLAN** mit **Captive Portal/Voucher** und Client-Isolation.
- Prüfungsklassiker: WLAN-Scan deuten (SSID, BSSID, Kanal, Breite, Security, Signal), PSK-Nachteile, AAA/RADIUS, Gast-WLAN (2021W, 2022S, 2023W, 2024S, 2025S).

## Kernwissen

### Standards

| IEEE | Wi-Fi | Band | max. brutto | Bemerkung |
|---|---|---|---:|---|
| 802.11b | – | 2,4 GHz | 11 Mbit/s | veraltet |
| 802.11a | – | 5 GHz | 54 Mbit/s | veraltet |
| 802.11g | – | 2,4 GHz | 54 Mbit/s | veraltet |
| 802.11n | Wi-Fi 4 | 2,4 / 5 GHz | 600 Mbit/s | MIMO, 40 MHz |
| 802.11ac | Wi-Fi 5 | 5 GHz | ~6,9 Gbit/s | 80/160 MHz, MU-MIMO (Downlink) |
| 802.11ax | Wi-Fi 6 / **6E** | 2,4 / 5 (6E: + 6 GHz) | ~9,6 Gbit/s | OFDMA, viele Clients, WPA3 Pflicht für Zertifizierung |
| 802.11be | Wi-Fi 7 | 2,4 / 5 / 6 GHz | ~46 Gbit/s | 320 MHz, Multi-Link Operation |

### Frequenzbänder und Kanäle

| | 2,4 GHz | 5 GHz | 6 GHz |
|---|---|---|---|
| Reichweite / Wanddurchdringung | hoch | mittel | gering |
| Kanäle (EU) | 1–13, **überlappungsfrei 1, 6, 11** (bei 20 MHz) | viele (36–140), teils mit **DFS** (Radar-Ausweichen) | sehr viele, nur Wi-Fi 6E/7 |
| Störer | Bluetooth, Mikrowelle, Nachbar-WLANs, IoT | weniger | kaum |
| Typische Geräte | ältere IoT-Geräte, Kameras, Sensoren | Notebooks, Smartphones | neueste Endgeräte |

**Kanalbreite (BW):** 20/40/80/160 MHz – breiter = schneller, aber weniger freie Kanäle und störanfälliger. In 2,4 GHz nur 20 MHz verwenden.

### WLAN-Scan lesen (GA2 Sommer 2024)

| Spalte | Bedeutung | Bewertung |
|---|---|---|
| **SSID** | Name des Funknetzes | `<hidden>` = SSID-Broadcast deaktiviert |
| **BSSID** | MAC-Adresse der Funkzelle des APs | mehrere BSSIDs mit gleicher SSID = mehrere APs/Bänder |
| **CHANNEL** | genutzter Kanal | viele Netze auf Kanal 1 → **Gleichkanalstörung** |
| **BW [MHz]** | Kanalbreite | 40 MHz in 2,4 GHz belegt zwei Drittel des Bandes |
| **SECURITY** | Verfahren | Open, WPA2-Personal, WPA2-/WPA3-Enterprise |
| **SIGNAL [dBm]** | Empfangsstärke (negativ, logarithmisch) | **−50 sehr gut**, −67 Minimum für VoIP, −75 schwach, **−85 kaum nutzbar** |

Faustregel: **3 dB weniger = halbe Leistung**; −51 dBm ist also deutlich besser als −85 dBm.

### Sicherheitsverfahren

| Verfahren | Verschlüsselung | Bewertung |
|---|---|---|
| Open | keine | nur mit Captive Portal + **OWE (Enhanced Open)** sinnvoll |
| WEP | RC4 | gebrochen – nicht verwenden |
| WPA | TKIP | veraltet |
| **WPA2** | AES-**CCMP** | Standard; Personal (PSK) anfällig für Offline-Wörterbuchangriffe auf den mitgeschnittenen Handshake |
| **WPA3** | AES-CCMP/GCMP | Personal mit **SAE** (Schutz vor Offline-Brute-Force, Forward Secrecy), Enterprise-192-Bit-Modus, PMF (geschützte Management-Frames) Pflicht |

**Authentifizierung vs. Vertraulichkeit (GA2 Sommer 2025):** Authentifizierung = Wer darf ins WLAN? (PSK/SAE oder 802.1X/EAP). Vertraulichkeit = Verschlüsselung der Funkstrecke (CCMP/GCMP).

### Personal (PSK) vs. Enterprise (802.1X)

| | Personal (PSK/SAE) | Enterprise (802.1X/RADIUS) |
|---|---|---|
| Zugangsdaten | ein Schlüssel für alle | individuell: AD-Konto, Zertifikat (EAP-TLS), PEAP |
| Mitarbeiter verlässt Firma | Schlüssel muss überall geändert werden | Konto sperren – fertig |
| Weitergabe | Schlüssel kann weitergegeben werden | personengebunden |
| Nachvollziehbarkeit | keine | **Accounting** pro Benutzer |
| VLAN-Zuweisung | je SSID fest | dynamisch je Benutzer/Gruppe |
| Aufwand | gering | RADIUS-Server, ggf. PKI |

**Nachteile WPA2-Personal (2024S, „nenne drei“):** gemeinsamer Schlüssel wird weitergegeben; bei Austritt/Geräteverlust muss der PSK auf allen Geräten geändert werden; Handshake kann mitgeschnitten und offline per Brute-Force angegriffen werden; keine Zuordnung von Aktivitäten zu Personen; wer den PSK kennt, kann bei WPA2 den Verkehr anderer Clients entschlüsseln.

### AAA mit RADIUS (GA2 Sommer 2022, Winter 2023)

| | Frage | Beispiel |
|---|---|---|
| **Authentication** | Wer bist du? | Benutzername/Passwort, Zertifikat über EAP |
| **Authorization** | Was darfst du? | VLAN, Zeitfenster, Bandbreite |
| **Accounting** | Was hast du getan? | Anmeldezeit, Dauer, Datenvolumen protokollieren |

Rollen: **Supplicant** (Client) → **Authenticator** (AP/Controller) → **Authentication Server** (RADIUS, z. B. Microsoft NPS, fragt das AD).

### Architektur

| Komponente | Aufgabe |
|---|---|
| **Access Point (AP)** | Funkschnittstelle: sendet Beacons, verschlüsselt die Funkstrecke, verbindet WLAN-Clients mit dem LAN (Bridge) |
| **WLAN-Controller (WLC)** | zentrale Verwaltung vieler APs: SSIDs, Kanal-/Sendeleistungsplanung, Firmware, Roaming, RADIUS-Anbindung, Monitoring, Rogue-AP-Erkennung (2021W) |
| Cloud-Management | Controller als Cloud-Dienst |
| Mesh | APs verbinden sich per Funk untereinander (wenn kein Kabel möglich) |

**Roaming:** Client wechselt zwischen APs derselben SSID; schnell mit **802.11r** (Fast Transition), unterstützt durch **802.11k/v**.

### Gast-WLAN / BYOD

| Maßnahme | Begründung |
|---|---|
| **Eigene SSID** | Gäste sehen/nutzen nicht das interne Netz |
| **Eigenes VLAN + Subnetz** | Isolation auf L2/L3, Firewall: nur Internet, kein Zugriff aufs LAN |
| **Captive Portal mit Voucher** | Zugangskontrolle, Zustimmung zu Nutzungsbedingungen (Haftung), zeitlich/volumenmäßig begrenzt |
| **Client-Isolation** | Gäste können sich nicht gegenseitig angreifen |
| **Bandbreitenlimit / QoS** | Geschäftsverkehr bleibt priorisiert |

**Voucher-Möglichkeiten (2024S, „nenne drei“):** Gültigkeitsdauer, Datenvolumen, Bandbreite, Anzahl Geräte pro Voucher, Zeitfenster, Anzeige von AGB/Infoseite, Protokollierung.

### Planung (Site Survey, GA2 Sommer 2022, Winter 2023)

- **Ausleuchtung** messen (Heatmap), Dämpfung durch Beton, Metall, Glas, Wasser berücksichtigen.
- **Kanalplanung:** benachbarte APs auf unterschiedliche Kanäle (2,4 GHz: 1/6/11), Sendeleistung anpassen.
- **Kapazität:** Clients pro AP, Bandbreitenbedarf (VoIP, Video), Zellgröße eher klein halten.
- **Infrastruktur:** PoE-Budget, Uplinks, Montageorte, Controller.
- **Störquellen** und **Nachbarnetze** erfassen.

### Hidden SSID und MAC-Filter

Beides ist **keine echte Sicherheitsmaßnahme**: Die SSID taucht in Probe-Requests/Associations im Klartext auf, MAC-Adressen sind fälschbar. Wirkung nur: Netz ist für Laien unsichtbar.

## Prüfungsfrage

**Frage (angelehnt an GA2 Sommer 2024):** Nach dem Austausch der APs verbinden sich die Überwachungskameras (802.11n, nur 2,4 GHz, WPA2) nicht mehr. Außerdem klagen Mitarbeiter über langsames WLAN; der Scan zeigt die SSIDs „MEDIA-AGENTUR“, „Hotspot1“, „iPhone von Thomas“ und eine versteckte SSID – alle auf Kanal 1. Nennen Sie zwei mögliche Ursachen für das Kamera-Problem und erläutern Sie das Performance-Problem.

**Musterantwort:** Kameras: (1) Die neuen APs senden die Kamera-SSID nur im **5-GHz-Band** oder das 2,4-GHz-Band ist deaktiviert – die Kameras können nur 2,4 GHz. (2) Die neuen APs verlangen **WPA3** bzw. nur 802.11ax („Wi-Fi-6-only“) – die Kameras unterstützen nur WPA2/802.11n. Auch ein geänderter SSID-Name/Schlüssel kommt in Frage. Performance: Alle Netze nutzen **denselben Kanal 1**. Da WLAN ein geteiltes Medium ist, müssen alle Geräte auf diesem Kanal abwechselnd senden (Gleichkanalstörung), der Durchsatz sinkt, die Latenz steigt. Lösung: eigenen AP auf Kanal 6 oder 11 bzw. ins 5-GHz-Band legen.

## Falle

- „Hidden SSID macht das WLAN sicher“ → nein.
- „WPA2-Enterprise nutzt einen PSK“ → nein, **802.1X/RADIUS** mit individuellen Zugangsdaten.
- 2,4-GHz-Kanäle 1, 2, 3 als „verschiedene Kanäle“ empfehlen → sie **überlappen**; nur 1/6/11.
- Bruttorate als Nutzrate verkaufen („Wi-Fi 6 = 9,6 Gbit/s für jeden Client“).
- dBm falsch herum lesen: **−50 ist besser als −80**.
- Gast-WLAN nur als eigene SSID ohne eigenes VLAN/Firewallregeln → keine echte Trennung.

## Merksatz

**„2,4 weit und voll, 5 schnell und kurz – Enterprise für Mitarbeiter, Voucher für Gäste, eins-sechs-elf für Ruhe.“**

## Verwandt

- [Port-Security & 802.1X](port-security-8021x.md) – 802.1X, EAP, RADIUS im Detail
- [VLAN](vlan.md) – SSID ↔ VLAN, Gast-VLAN
- [PoE](poe.md) – Stromversorgung der Access Points
- [Netzwerkgeräte](netzwerkgeraete.md) – AP und WLAN-Controller
- [AAA & SIEM](../Sicherheit/aaa-siem.md) – RADIUS/TACACS+ und Protokollierung
- [MDM & BYOD](mdm-byod.md) – private Geräte im Firmennetz
- [Netzwerktypen](../netzwerktypen.md) – WLAN als LAN-Technik
- [Lektion 09 – WLAN](../../analyse-der-exams/09_WLAN.md)
