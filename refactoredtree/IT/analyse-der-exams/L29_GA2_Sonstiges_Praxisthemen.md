# Lektion 29 – GA2: Weitere wiederkehrende Praxisthemen ("Sonstiges")

**Bereich:** GA2 – Analyse und Entwicklung von Netzwerken  
**Hinweis am Anfang:** Diese Themen erschienen jeweils nur 1–2x im Korpus, sind aber gute Punkte-Chancen, weil sie oft übersehen werden. TLS/PKI/VPN siehe eigene Lektion L27 – hier nur kurze Referenz.  
**Lernzeit:** ca. 2h

## 1. Warum diese Thematik wichtig ist

In der GA2-Prüfung (Analyse und Entwicklung von Netzwerken) stehen oft die Kerngebiete IPv4-Subnetting, Routing, Firewall und DNS im Fokus. Die folgenden Themen erscheinen seltener, bieten aber – weil sie oft übersehen werden – gute Chancen, Punkte zu holen, wenn sie drankommen. Alle unten genannten Punkte sind VERIFIZIERT aus dem echten Prüfungskorpus (10 Jahrgänge 2021–2026).

## 2. Themenüberblick mit Jahrgangsangabe

### QoS / Bandbreitenberechnungen (2022W, je 1–2 Aufgabenteile)
- **QoS (Quality of Service):** Priorisierung von Netzwerkverkehr, damit kritische Anwendungen (Sprach-, Video-Streaming) trotz voller Bandbreite funktionieren.
- **VoIP-Codec G.722:** 64 kbit/s + 10 % Overhead → pro Codec-Strecke ca. 72 kbit/s reservieren.
- **Videokonferenz-Bandbrechentabelle:** 
  - Standard-Video (360p): ≈ 1 Mbit/s
  - HD-Video (720p): ≈ 3 Mbit/s
  - Full-HD (1080p): ≈ 5–8 Mbit/s
  - 4K-Video: ≈ 15–25 Mbit/s
- **Berechnung am Beispiel:** 50 VoIP-Telefone × 72 kbit/s = 3,6 Mbit/s Gesamt-Reservierung nötig.
- **Download-Zeit-Berechnung:** MiB → Bit → Zeit. Formel: `Zeit (Sek) = (Dateigröße MiB × 8 × 1024) / Bandbreite bit/s`.
- **Beispiel:** 100 MiB Datei mit 10 Mbit/s Download → `(100 × 8 × 1024) / 10.000.000 × 8 = 81,92 s` ≈ 1,35 min.

### Glasfaser / Kupfer / SFP (2023S, 2023W, 2025W)
- **SFP (Small Form-factor Pluggable):** 1 Gbit/s Uplink/Trunk zwischen Switches. Reichweite bis maximal 150 m (Multimode), bis 20 km (Singlemode).
- **SFP+ (enhanced SFP):** 10 Gbit/s. Core-/Backbone-Uplinks mit höherem Bandbreitenbedarf. Reichweite ähnlich wählbar (Multimode bis 100 m, Singlemode bis 40–80 km).
- **Glasfaser vs. Kupfer-Begründung (2023W):** 
  - **Reichweite:** Glasfaser deutlich weiter (km-range vs. max. 100 m für Kupfer-Ethernet).
  - **EMV-Empfindlichkeit:** Glasfaser ist galvanisch getrennt – kein Elektromagnetische Verträglichkeit (EMV) Störeinfluss.
  - **Brandschutz/Sicherheit:** Glasfaser leitet keinen Strom, kann nicht "anzünden", eavesdropping (Abhören) ist schwieriger als bei Kupfer (optisch vs. elektrisch).
  - **Platzbedarf:** Glasfaserkabel dünner und leichter als Kupfer-Bus-Systeme.
- **SFP vs. SFP+ Entscheidung:** 1 Gbit/s reicht für normale Uplinks, 10 Gbit/s für Core/Backbone, wenn Traffic-Volumen hoch ist.

### SNMP (Simple Network Management Protocol) (2021W)
- **`snmpget`:** Befehl zur Abfrage eines einzelnen OID-Werts von einem Gerät.
- **OID-Struktur:** OID (Object Identifier) identifiziert verwaltete Objekte im MIB (Management Information Base). Beispiel: `1.3.6.1.2.1.1.1` = System-Name (sysDescr).
- **Trap-Mechanismus:** Das Netzwerkgerät sendet proaktiv ein "Trap", wenn ein Ereignis eintritt (z. B. "Gemeinschafts-Spannung verloren", "Fans aus"). Das ist aktiver als das periodische "Polling" (nachfragen).
- **Community-String:** Art "Passwort" für SNMP-Zugriff (oft "public" oder "private" – sollte geändert werden!).
- **Get vs. Get-next:** Iterative OID-Auflösung entlang des Baums.

### MQTT / IoT (2025S)
- **MQTT (Message Queuing Telemetry Transport):** Leichter Publish/Sub-Protokoll für schlechte Netzwerkbedingungen (low bandwidth, hohe Latenz, instabile Verbindungen).
- **TCP-Notwendigkeit:** MQTT basiert auf TCP, um die Zuverlässigkeit von Publish-Nachrichten zu gewährleisten (Paketverlust wird durch TCP-Stack behandelt, Retransmission sichert die Delivery).
- **Bandbreitenberechnung (2025S-Beispiel):** 500 Geräte × 66 Byte / 60 s = 4,4 kbit/s Gesamtbandbreite. (Sehr gering, aber bei Tausenden von Geräten relevant.)
- **Quality of Service (QoS-Level) in MQTT:** 0 (at most once), 1 (at least once), 2 (exactly once) – sicherstellen, dass Nachrichten einmal und nur einmal zugestellt werden.

### E-Mail-Sicherheit: SPF / Spamfilter / Sandbox (2022S, 2023S)
- **SPF (Sender Policy Framework):** DNS-Eintrag, der festlegt, welche Mail-Server berechtigt sind, E-Mails im Namen einer Domain zu versenden. Verhindert "E-Mail-Spoofing" (Fälschen des Absenders).
- **Beispiel-Record:** `v=spf1 ip4:203.0.113.0/24 ~all` → Nur Server aus diesem Netzblock dürfen mails senden.
- **Spamfilter:** Analyse von Header, Body, Sender-Reputation, Bayesianische Filter (Wahrscheinlichkeitsberechnung), Blacklists (RBL). Punkte, die eine Mail als "Spam" bewerten.
- **Sandbox:** E-Mail-Anhänge werden in einer isolierten Umgebung geöffnet und auf Schadcode untersucht, bevor sie dem Empfänger zugestellt werden. Besonders bei ZIP-Dateien, Makros in Office-Dokumenten.
- **Briefgeheimnis bei Mail-Kontrolle (2022S):** Arbeitgeber darf E-Mails nicht allgemein überwachen – nur im Rahmen des sogenannten "Betriebsinteresses" und unter Beachtung der Verhältnismäßigkeit (BAG-Urteil). Auch hier gilt: Betriebliche E-Mail-Adressen sind oft Firmen-Eigentum, private Nutzung muss möglich sein/gemacht werden.

### DSL / Kabel / LTE / Glasfaser-Anschlussarten (2025S, 2025W, 2023W)
- **Geschäftskunden-DSL vs. Privatkunden-DSL (2025S):** 
  - **Privatkunde:** maximal verfügbare Bandbreite oft niedriger, "bis ans Haus" (FTTB/FTTC), asymmetrisch (mehr Downstream als Upstream).
  - **Geschäftskunde:** SLA-verfügbare Bandbreite, symmetrisch oder symmetrisch zuschaltbar, garantierte Reparaturzeiten, oft direktes Glasfaser-Anschluss bis in den Keller (FTTB).
- **GPON (Gigabit Passive Optical Network) "shared medium" (2025S/2025W):**
  - Glasfasernetzwerk, bei dem mehrere Kunden über ein passives Optisches Splitter-Netzwerk einen Glasfaser-Anteil teilen.
  - **Vorteil:** Geringer Aufwand, kein aktiver Switch nötig zwischen Haus undverteiler.
  - **Nachteil:** "Shared medium" → Bandbreite wird unter den Nutzern aufgeteilt, burst-fähig, minimale Latenz-Garantie nicht immer möglich.
- **Carrier-Grade NAT (CGNAT) bei LTE (2023W/2025S):**
  - Aufgrund begrenzter IPv4-Adressen wird auf Ebene des Mobilfunkanbieters NAT eingesetzt.
  - Viele Nutzer teilen sich eine öffentliche IPv4-Adresse.
  - **Nachteil:** Portforwarding ist schwer bis unmöglich (weil die öffentliche IP nicht dem Endgerät, sondern dem Anbieter-Gateway gehört). IPv6 wird als Lösung immer stärker empfohlen.

### Passwort-Policy & MFA (2025W, 2025S, 2026S)
- **Passwort-Policy:** Komplexitätsanforderungen (Mindestlänge ≥ 8, Mischung aus Groß/Klein, Zahl, Sonderzeichen), Ablauf (z. B. alle 90 Tage ändern), Sperre nach fehlgeschlagenen Versuchen (z. B. nach 3 Fehlversuchen 10 min gesperrt).
- **MFA (Multi-Factor-Authentication):** Kombination aus mehreren Faktoren:
  - **Possession (Etwas, das man hat):** Smartphone (App/Code), Hardware-Token, Smartcard.
  - **Inherence (Etwas, das man ist):** Fingerabdruck, Face-ID, Spracherkennung (Biometrie).
  - **Knowledge (Etwas, das man weiß):** Passwort, PIN.
- **Beispiel:** "Bank-Login: Passwort (Wissen) + TOTP-Code per App (Besitz)" → 2-Faktor-Authentifizierung.

### Fax over VoIP-Problem (2022W)
- **Problem:** Sprach-Codecs (z. B. G.711, G.729) sind für Sprachausgabe optimiert und tolerieren Datenverlust/Abbrüche. Fax-Signale und -Daten sind jedoch digital und fehlerempfindlich – ein einziger verlorener Paketteil führt zum kompletten Übertragungsabbruch.
- **Lösungsmöglichkeiten:** 
  - **T38 (Fax over IP):** Spezifisches Protokoll für Fax-Übermittlung über IP-Netze, das Datenpakete zuverlässiger überträgt.
  - **Analog-Fallback:** Faxgerät erkennt Störungen und schaltet auf analogen Modus um (sofern vorhanden).
  - **ISDN/PRI-Beibehaltung:** Wo möglich, Faxe über ISDN-Leitung statt VoIP senden.

### 802.1X: VLAN-Zuordnung (2023S) und WPA2-Enterprise-Baustein (2023W)
- **802.1X:** Authentifizierungsrahmen für Netzwerkzugänge. Basiert auf EAP (Extensible Authentication Protocol).
- **Komponenten:** Supplicant (Client), Authenticator (Switch/AP), Authenticator-Server (Radius).
- **VLAN-Zuordnung (2023S):** Nach erfolgreicher Authentifizierung wird dem Client ein VLAN zugewiesen (z. B. "Mitarbeiter-VLAN" oder "Gast-VLAN"), basierend auf dem authentifizierten Benutzernamen/ -gruppe.
- **WPA2-Enterprise-Bausteine (2023W):** 
  - **AAA-Server (Radius/LDAP):** Zentrale Verwaltung von Benutzern und Schlüsseln.
  - **EAP-TLS:** Nutzung von Zertifikaten für beide Seiten (Client und Server).
  - **Vorteil gegenüber PSK:** Kein gemeinsames Passwort, das weitergegeben werden kann; jeder User hat eigenes Zertifikat/Anmeldedaten.

### DMZ-Konzept: Server-Platzierung begründen (wiederholt 2021W, 2022S, 2023W, 2025S, 2026S)
- **DMZ (Demilitarized Zone):** Netzwerksegment zwischen internem und externem Netz (Internet), in dem server liegen, die von außen erreichbar sein sollen, aber keinen direkten Zugriff auf das interne Netz haben.
- **Platzierung-Begründung:** 
  - **Webserver:** In DMZ, damit Internetnutzer darauf zugreifen können, aber diese keinen direkten Zugriff auf das interne Firmennetz (z. B. Datenbank, File-Server) haben.
  - **Mailserver:** In DMZ, weil SMTP/IMAP/POP3 von außen benötigt werden, aber interne AD/Kontakte isoliert bleiben sollen.
  - **Warum nicht direkt im internen Netz?** → Wenn der Webserver kompromittiert wird, ist der Angreifer bereits im internen Netz (Lateral Movement). DMZ begrenzt die "Blast Radius".
- **Typische Aufbaulösung:** Router → Firewall → (DMZ-Segment mit Web-/Mailserver) → Interner Switch → Firmennetz.

## 3. Übungsaufgabe (im Prüfungsstil, selbst erstellt)

> "Ein Unternehmen betreibt einen Webserver im Internet und einen internen Dateiserver. Warum sollte der Webserver in einer DMZ und nicht direkt im internen Netzwerk platziert werden?"

<details><summary>Lösung (aufklappen)</summary>

**Antwort:** Der Webserver muss von Internetnutzern erreichbar sein, soll aber keinen direkten Zugriff auf interne Ressourcen (Datenbank, File-Server, AD) erhalten. Wäre er im internen Netz, könnte ein Angreifer, der den Webserver kompromittiert, direkt auf das interne Netzzugreifen (Lateral Movement). In der DMZ liegt er abgeschottet; über die Firewall sind nur spezifische Ports/Verbindungen (z. B. Datenbank-Zugriff von Webserver nach außen) erlaubt – der Rest des internen Netzes ist geschützt.

</details>

> "Ein Mobilfunk-Anbieter nutzt Carrier-Grade NAT (CGNAT). Welche Auswirkung hat dies auf einen Kunden, der einen externen Webserver betreibt und Ports 80/443 nach draußen weiterleiten möchte?"

<details><summary>Lösung (aufklappen)</summary>

**Antrag:** Der Kunde kann seine Ports 80/443 nicht einfach nach draußen weiterleiten, weil die öffentliche IPv4-Adresse des Anbieters vielen Kunden gemeinsam zur Verfügung steht (CGNAT). Der eingehende Verkehr kann nicht mehr dem spezifischen Endgerät des Kunden zugeordnet werden. Lösung: Auf IPv6 umstellen oder einen öffentlichen IPv4-Block beim Anbieter buchen (oft gegen Aufpreis).

</details>

## Selbstcheck – kannst du das?

- [ ] Ich kann QoS/Bandbreitenberechnungen (VoIP, Video, Download-Zeit) durchführen.
- [ ] Ich SFP (1 Gbit) vs. SFP+ (10 Gbit) einsortieren und Gründe für Glasfaser vs. Kupfer nennen kann.
- [ ] Ich SNMP (snmpget, OID-Struktur, Trap vs. Polling) erklären kann.
- [ ] Ich MQTT (TCP-Notwendigkeit, Bandbreitenberechnung mit Beispiel) skizzieren kann.
- [ ] Ich SPF (E-Mail-Sicherheit, DNS-Eintrag) und Zweck erklären kann.
- [ ] Ich Spamfilter/ Sandbox-Funktion skizzieren kann.
- [ ] Ich DSL/Kabel/LTE/GPON/CGNAT-Unterschiede und Geschäfts- vs. Privatkunden-Verbindung unterscheiden kann.
- [ ] Ich Passwort-Policy und MFA (Possession/Inherence/Knowledge) auflisten kann.
- [ ] Ich Fax over VoIP-Problem und Lösung (T38, Analog-Fallback) benennen kann.
- [ ] Ich 802.1X (VLAN-Zuordnung, WPA2-Enterprise Bausteine) skizzieren kann.
- [ ] Ich DMZ-Konzept (Begründung der Server-Platzierung) erklären und skizzieren kann.

## Weiterführende Ressourcen

- IHK-Prüfungsaufgaben AP2 (GA2) zu Netzwerktechnik und Netzdiensten
- RFC 1157 (SNMP), RFC 6666 (MQTT), RFC 7208 (SPF)
- CSTA: T38 Fax over IP, Carrier-Grade NAT Analyse
- VoIP-Codecs und Bandbreiten-tabellen