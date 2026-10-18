# Themenliste – Alle Prüfungsthemen aus den GA1/GA2/WISO-Altklausuren (Sommer/Winter 2021–2026)

Diese Liste basiert auf der Durchsicht aller im Repo vorhandenen Prüfungen und Musterlösungen. Sortiert nach Prüfungsteil. Häufigkeit = grobe Einschätzung, wie oft/regelmäßig das Thema in den vorliegenden Jahrgängen vorkommt.

**Legende:** 🔴 = kommt fast in jedem Termin vor · 🟡 = kommt regelmäßig vor · 🟢 = vereinzelt/möglich

---

## GA1 – Konzeption und Administration von IT-Systemen

### Speicher & Storage
- 🔴 RAID-Level (0,1,5,6,10,50,60) – Eigenschaften, Ausfallsicherheit, Nettokapazität berechnen
- 🔴 Festplattenanzahl/-kapazität berechnen (Erweiterung, Umstieg zwischen RAID-Leveln)
- 🟡 MTBF/MTTF – Berechnung und Unterschied (Austauschteile vs. reparierte Systeme)
- 🟡 Speicherhierarchien, Einheiten-Umrechnung (KiB/MiB/GiB/TiB/PiB, Bit-Berechnung für Variablengröße)
- 🟡 Backup-Strategien: Voll-/Differenziell-/Inkrementell-Backup, Großvater-Vater-Sohn-Prinzip, Bandrotation
- 🟡 3-2-1-Backup-Regel
- 🟢 Disk-to-Disk-to-Tape (D2D2T)
- 🟢 NAS/SAN-Konzepte, Benchmark-Tests (Testbedingungen für objektive Ergebnisse)

### Virtualisierung
- 🔴 Hypervisor Typ 1 (bare-metal) vs. Typ 2 (gehostet) – Erläuterung, Einsatzbeispiel, marktgängiges Produkt
- 🔴 Vor-/Nachteile der Virtualisierung
- 🟢 Container vs. VM (in neueren Jahrgängen zunehmend relevant)

### IT-Sicherheit & Datenschutz
- 🔴 Symmetrische vs. asymmetrische Verschlüsselung (AES, RSA)
- 🔴 Hashverfahren (MD5, SHA-256) – Unterschied zu Verschlüsselung, Sicherheitsstatus
- 🟡 Betriebssystemhärtung, Sicherheitsmerkmale von Betriebssystemen (Firewall, Updates, Rechteverwaltung)
- 🟡 Informationspflichten bei Datenschutzverletzung/unbefugtem Zugriff
- 🟡 Warum Formatieren/Löschen datenschutzrechtlich nicht ausreicht; sicheres Löschen/Entsorgung von Datenträgern
- 🟡 Pseudonymisierung vs. Anonymisierung
- 🟢 Schutz mobiler Endgeräte (Notebook-Diebstahl, Verschlüsselung, Remote-Wipe)
- 🟢 Zertifikate/CA, Kollisionsangriffe, Zertifikatswiderruf (teils auch in GA2)

### Verfügbarkeit & Ausfallsicherheit
- 🟡 Technische Maßnahmen für hohe Ausfallsicherheit (redundante Netzteile, RAID, USV, redundante NICs/CPUs)
- 🟡 USV-Berechnung (Ladezustand, Überbrückungszeit, Umschaltung auf Notstromaggregat)
- 🟢 Monitoring/Alarmierung von Systemzuständen (SNMP-Traps, Schwellwerte)

### Programmierung & Datenmodellierung
- 🔴 Pseudocode / einfache Programmlogik lesen, Fehler finden (Off-by-one, Array-Grenzen, Endlosschleifen)
- 🔴 SQL-Grundlagen (SELECT, COUNT, WHERE, Sortierung/TOP-N-Abfragen)
- 🟡 ER-Modell / Datenbankdesign, Kardinalitäten, referentielle Integrität
- 🟡 UML-Diagramme: Aktivitätsdiagramm (häufig als Zeichenaufgabe), Klassendiagramm, weitere (Objekt-, Sequenz-, Zustandsdiagramm) – statische vs. dynamische Sicht
- 🟡 Datentypen & Speicherbedarf berechnen (Bit-Berechnung für Wertebereiche)
- 🟢 Zugriff auf Arrays/Structs (Objektzugriff mit Punktoperator)
- 🟢 Testverfahren: White-Box- vs. Black-Box-Test

### Projektmanagement / Vorgehensmodelle
- 🟢 SCRUM, Kanban (eher in AP1, kommt aber auch in AP2-Kontext vor)
- 🟢 Qualitätsmanagement / Softwaretest-Grundlagen

---

## GA2 – Analyse und Entwicklung von Netzwerken

### IP-Adressierung & Subnetting
- 🔴 IPv4-Subnetting: Netzadresse, Broadcast-Adresse, nutzbare Hosts berechnen
- 🔴 VLSM – ungleich große Subnetze aus einem Adressband bilden
- 🔴 Subnetzmasken in CIDR- und Dezimalschreibweise
- 🟡 Private vs. öffentliche IP-Adressbereiche (RFC1918)
- 🟡 IPv6-Grundlagen: Adressraum, Subnetting (Hex-Rechnung), SLAAC (zustandslose Autokonfiguration), Router Advertisement
- 🟡 Warum IPv6 eingeführt wurde (Adressknappheit von IPv4), Vorteile von IPv6 (Sicherheit/IPsec, QoS, kein NAT nötig)
- 🟢 Dual Stack (IPv4/IPv6 parallel)

### Netzwerktechnik & -infrastruktur
- 🔴 VLANs: Zweck, Trunk-/Access-Port, Tagging, VLAN-Konfiguration auf Switches
- 🔴 Routing zwischen VLANs (Router-on-a-Stick, Subinterfaces)
- 🔴 Routing-Tabellen vervollständigen (Netz-ID, Subnetzmaske, Next-Hop/Interface), Default Route
- 🟡 Spanning Tree Protocol (Schleifenvermeidung, geblockte Ports)
- 🟡 Link Aggregation (Bündelung physischer Verbindungen)
- 🟡 Unicast, Multicast, Broadcast – Unterschiede mit Beispielen
- 🟡 FHRP (VRRP/HSRP) – Gateway-Redundanz
- 🟢 Redundante Internetanbindungen, Anbindungstechnologien (DSL, Glasfaser/FTTH, GPON Shared Medium, LTE/CGN, Satellit) inkl. Verfügbarkeitsberechnung
- 🟢 QoS / VoIP-Netztrennung (eigenes VLAN für Telefonie)
- 🟢 Monitoring-Port (SPAN/Mirror-Port)

### Fehlersuche & Diagnose
- 🔴 Troubleshooting anhand von ipconfig/ping/traceroute/nslookup-Ausgaben
- 🟡 DNS-Fehleranalyse (falscher/interner DNS-Server, "nicht autorisierende Antwort")
- 🟡 DHCP-Mitschnitt auswerten (IP, Subnetzmaske, Lease Time, DHCP-/DNS-Server-IP)
- 🟢 ICMP-Filterung durch Firewalls als Fehlerursache bei Ping/Traceroute

### Namens- & Adressdienste
- 🔴 DHCP (Discover/Offer/Request/Ack – Broadcast-Prinzip)
- 🔴 DNS (Auflösung, interner vs. öffentlicher DNS-Server, Split-Horizon-DNS)
- 🟢 Reverse Proxy vs. einfache Portweiterleitung

### Sicherheit im Netzwerk
- 🔴 Firewall-Grundlagen: Aufgaben (Paketfilter, NAT/PAT, VPN-Endpunkt), Grenzen des Schutzes (Social Engineering, Insider, USB-Malware)
- 🔴 Firewall-Regeln lesen/erstellen (Tabellen mit Quelle/Ziel/Port/Protokoll/Richtung)
- 🟡 Stateful Packet Inspection (SPI) vs. Next Generation Firewall (NGFW, Layer-7-Funktionen)
- 🟡 WLAN-Sicherheit: SSID-Broadcast, Authentifizierung vs. Verschlüsselung, WPA/WPA2/WPA3-Beispiele
- 🟡 ARP-Poisoning / Man-in-the-Middle-Angriffe, Erkennung im Paketmitschnitt
- 🟡 Port-Security (statische/sticky MAC-Adressen, Violation Mode)
- 🟢 TLS/Zertifikate: Zertifikatssignatur (Hash statt Volltext), Kollisionsangriffe, CA-Kompromittierung und Konsequenzen, Diffie-Hellman-Schlüsselaustausch, Rolle asymmetrischer Verschlüsselung bei TLS 1.3

### Berechnungen (Netzwerk-Kontext)
- 🟡 Verfügbarkeitsberechnung (z. B. 99%/99,9%, Ausfallzeit in Stunden pro Jahr)
- 🟡 Redundanzberechnung (Ausfallwahrscheinlichkeit bei parallelen Anbindungen)

---

## WISO – Wirtschafts- und Sozialkunde

### Ausbildung & Berufsbildungsgesetz (BBiG)
- 🔴 Ende des Ausbildungsverhältnisses nach bestandener Prüfung, automatische Weiterbeschäftigung (§21/§24 BBiG)
- 🔴 Kündigung während der Ausbildung: Form, Fristen, Kündigungsgründe, Minderjährigkeit (Unterschrift der Erziehungsberechtigten)
- 🟡 Zeugnisarten (einfaches vs. qualifiziertes Ausbildungszeugnis) – welche Formulierung macht es "qualifiziert"
- 🟡 Jugendarbeitsschutzgesetz (JArbSchG): Ruhepausen, Ruhezeiten, wöchentliche Arbeitszeit für Jugendliche
- 🟢 Europass (Zweck, Bestandteile)
- 🟢 Berufliche Fortbildung vs. Umschulung (Abgrenzung von Bildungsmaßnahmen)

### Arbeitsrecht (allgemein)
- 🔴 Kündigungsfristen nach § 622 BGB (Grundfrist, gestaffelte Arbeitgeberfristen nach Betriebszugehörigkeit), Probezeit-Kündigungsfrist
- 🟡 Kündigungsschutz (Betriebsratsmitglieder, Schwerbehinderte, werdende Mütter – wer NICHT besonders geschützt ist)
- 🟡 Tarifvertrag: Mantel- vs. Lohn-/Gehaltstarifvertrag, wer verhandelt (Arbeitgeberverband/Gewerkschaft), Arbeitskampf/Friedenspflicht
- 🟢 Arbeitsgerichtsbarkeit vs. Sozialgerichtsbarkeit (welches Gericht ist zuständig)

### Betriebsverfassung & Mitbestimmung
- 🔴 Betriebsrat: Wahlvoraussetzungen, Interessenvertretung, Mitbestimmungsrechte (z. B. Urlaubsplan), Anhörungspflicht bei Kündigung
- 🟡 Jugend- und Auszubildendenvertretung (JAV) – Rechte und Aufgaben
- 🟡 Betriebsvereinbarung – was darf/kann darüber geregelt werden

### Sozialversicherung
- 🔴 Die 5 Zweige der Sozialversicherung (Kranken-, Renten-, Pflege-, Arbeitslosen-, Unfallversicherung) – was gehört NICHT dazu (z. B. Lebensversicherung)
- 🔴 Beitragssatzberechnung (Arbeitnehmeranteil an Krankenversicherung/Rentenversicherung berechnen)
- 🟡 Beitragsbemessungsgrenze – Bedeutung und Auswirkung

### Steuern
- 🟡 Direkte vs. indirekte Steuern, Lohnsteuerklassen
- 🟢 Steuerarten im Unternehmenskontext

### Unternehmensorganisation
- 🔴 Leitungssysteme (Einlinien-, Mehrlinien-, Stab-Linien-, Matrix-, Spartenorganisation) anhand Organigramm erkennen
- 🟡 Stabsstellen vs. Linienstellen, Weisungsbefugnis
- 🟢 Wirtschaftssektoren (primär/sekundär/tertiär) – Beispiele zuordnen

### Recht & Verträge
- 🟡 Kaufvertragsrecht (Angebot/Annahme, Gewährleistung, Mängelrechte)
- 🟢 Gesellschaftsformen (GmbH, KG, etc.) – Grundlagen
- 🟢 Kartellrecht (anmeldepflichtige vs. erlaubte Kartellarten)
- 🟢 Gerichtszuständigkeiten je nach Streitfall

### Kosten- & Wirtschaftlichkeitsrechnung
- 🔴 Wirtschaftlichkeit berechnen (Ertrag/Aufwand-Verhältnis, welcher Auftrag am wirtschaftlichsten ist)
- 🟡 Eigenkapitalrentabilität berechnen (Reingewinn / Eigenkapital × 100)
- 🟡 Fixe/variable Kosten, Deckungsbeitragsrechnung
- 🟢 Businessplan – Zweck und Bestandteile (v. a. bei Existenzgründung)

### Marketing & Absatz
- 🟢 Marketing-Grundlagen (Marketing-Mix: Produkt, Preis, Kommunikation, Distribution)
- 🟢 Marktformen (Angebot/Nachfrage-Konstellationen)

---

## Wie du diese Liste nutzen solltest

1. **Priorisiere 🔴-Themen zuerst** – die kommen am zuverlässigsten dran und bringen die meisten Punkte
2. **Häng dich bei 🟢-Themen nicht zu lange auf** – lieber breit über alle Themen einen Überblick haben als bei Randthemen perfektionieren
3. Nutze diese Liste als Checkliste: Häng ✅ dran, sobald wir eine Lektion zu dem Thema gemacht haben
4. Sag mir einfach "mach weiter mit [Thema]" – ich erstelle dann die nächste Lektion im gleichen Format wie die ersten drei

---
*Erstellt durch Auswertung aller GA1_SI_*, GA2_SI_* und WISO_* Dateien (Sommer/Winter 2021 bis Sommer 2026) im Repository.*
