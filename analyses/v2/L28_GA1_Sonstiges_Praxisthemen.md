# Lektion 28 – GA1: Weitere wiederkehrende Praxisthemen ("Sonstiges")

**Bereich:** GA1 – Konzeption und Administration von IT-Systemen  
**Hinweis am Anfang:** Diese Themen erschienen jeweils nur 1–2x im Korpus, sind aber gute Punkte-Chancen, weil sie oft übersehen werden.  
**Lernzeit:** ca. 2h

## 1. Warum diese Thematik wichtig ist

In der GA1-Prüfung (Analyse und Administration von IT-Systemen) stehen oft die Kerngebiete Cloud Computing, Programmierung, Datenbanken und UML im Fokus. Die folgenden Themen erscheinen seltener, bieten aber – weil sie oft übersehen werden – gute Chancen, Punkte zu holen, wenn sie drankommen. Alle unten genannten Punkte sind VERIFIZIERT aus dem echten Prüfungskorpus (10 Jahrgänge 2021–2026).

## 2. Themenüberblick mit Jahrgangsangabe

### Cluster-Size/Zuordnungseinheit (2021W, 2023W – identische Frage)
- Die **Cluster-Size** (oder Zuordnungseinheit) gibt an, wie viele Bytes einem Cluster zugeordnet sind – also die kleinste Speichereinheit auf einer Festplatte.
- **Kleine Cluster** (z. B. 512 B) eignen sich für viele kleine Dateien (viele Einträge, geringe interne Fragmentierung pro Datei).
- **Große Cluster** (z. B. 65.536 B) eignen sich für wenige große Dateien (weniger Einträge, aber größere Einheiten).
- **Frage-Format:** „Welche Cluster-Size verwenden Sie, wenn … viele kleine Dateien? / … wenige große Dateien?“

### Datendeduplizierung & Komprimierung mit Beispielen (2021W ausführlich, 2024W englischer Quelltext)
- **Datendeduplizierung:** Wiederholte Datenblöcke werden einmal gespeichert, Referenzen darauf gesetzt. Reduktion: bis zu 30x bei geeigneten Daten (z. B. Server-Logs, virtuelle Maschinen).
- **Komprimierung:** Algorithmen (z. B. ZIP, gzip) reduzieren die Größe von Daten, indem redundante Information entfernt wird. Reduktion: je nach Datenart 10–30x.
- **Prüfungsfokus:** Unterschied zwischen Duplizierung (physische Reduktion) und Komprimierung (algorithmische Reduktion), Beispielrechnung kann gefordert werden.

### Latenz: Definition, Ursachen, Auswirkung auf Cloud-DB-Zugriff (2022W)
- **Latenz:** Verzögerung zwischen Anfrage und Antwort. In Netzen oft durch physische Distanz, Netzwerküberlastung oder Verarbeitungszeit verursacht.
- **Ursachen bei Cloud-DB-Zugriff:** Entfernung zum Rechenzentrum, mehrere Netzwerk-Hops, Lastausgleich, Verschlüsselungs-overhead.
- **Auswirkung:** Schlechtere Performance für transaktionale Anwendungen, Notwendigkeit von Caching oder lokalen Zwischenspeichern.

### Horizontale (Scale-out) vs. vertikale (Scale-up) Skalierung (2023S)
- **Horizontale Skalierung (Scale-out):** Zusätzliche Knoten/Server zum bestehenden Cluster hinzufügen. Vorteil: Lineare Skalierbarkeit, Redundanz. Nachteil: Koordination, Dateninkonsistenz möglich.
- **Vertikale Skalierung (Scale-up):** Existing Server leistungsfähiger austauschen (mehr CPU, mehr RAM). Vorteil: Keine Änderungen an der Architektur nötig. Nachteil: Hardware-Obergrenze, Single-Point-of-Failure.
- **Frage-Format:** „Welche Skalierungsart wählen Sie für eine hochverfügbare Web-App mit erwartetem starken Wachstum?“

### Blue-Green-Deployment mit Schaubild (2023S)
- **Blue-Green-Deployment:** Zwei identische Umgebungsumgebungen (Blue = produktiv, Green = neue Version). Traffic wird schrittweise auf Green umgelenkt.
- **Vorteile:** Keine Ausfallzeit bei Deployment, einfache Rollback-Möglichkeit (auf Blue zurückwechseln), Risiko minimiert.
- **Schaubild:** Blaues Rechteck (alt) ↔ Grünes Rechteck (neu) mit Pfeil „Traffic umleiten“.

### USV-Typen Online/Offline/Line-Interactive = VFI/VFD/VI (2023S – kurz erwähnt, Details in Lektion 03)
- **Online-USV (VFI – Voltage/Frequency Independent):** Immer Netzspannung am Verbraucher, Batterie laufend geladen. Kein Umschaltzeitraum (RTO = 0). Teuer, aber höchste Verfügbarkeit.
- **Offline-USV (VFD – Voltage/Frequency Dependent):** Netzspannung direkt am Verbraucher, bei Netzausfall über Batterie umschalten (RTO ≈ 200–400 ms). Günstiger, geeignet für PC/Workstation.
- **Line-Interactive-USV:** Netzspannung wird stabilisiert (Autotransformer), Batterie nur bei tieferem Spannungsabfall zugeschaltet. Mittelklasse, geeignet für Server, Netzwerkgeräte.
- **Prüfungsrelevant:** RTO (Recovery Time Objective) und PTO (Peak Time Objective) unterscheiden.

### Windows Task-Scheduler: `SCHTASKS /Create /tn CPUMON /tr ... /sc DAILY /st 16:10` (2023S)
- **Aufgabe:** Automatisierte Überwachung oder Wartung zu einer bestimmten Zeit.
- **Beispiel-Befehl:** `SCHTASKS /Create /tn CPUMON /tr "C:\Skripte\cpu_monitor.bat" /sc DAILY /st 16:10`
- **Bedeutung:** Erstellt eine Aufgabe, die täglich um 16:10 Uhr die Batch-Datei `cpu_monitor.bat` ausführt.
- **Parameter:** `/tn` = Task-Name, `/tr` = Auszuführendes Programm/Script, `/sc` = Zeitplan (DAILY = täglich), `/st` = Startzeit (Stunde:Minute).

### Unix-Rechte: `chmod`, `ls -la`, oktale Notation (2023W)
- **`ls -la`:** Listet Dateien mitberechtigungen und Eigentümern auf (z. B. `-rwxr-xr-- 1 user group 1234 Mai 15 dateiname`).
- **Oktale Notation:** `rwx` = 7, `r-x` = 5, `r--` = 4. Kombination: `chmod 664 datei` → Owner: lesen/schreiben (6), Gruppe: lesen/schreiben (6), Andere: lesen (4).
- **Bedeutung der Buchstaben:** `r` = read (4), `w` = write (2), `x` = execute (1).
- **Beispiel:** `chmod 755 sketch.sh` → Owner volle Rechte (7 = 4+2+1), Gruppe und andere können lesen und ausführen (5 = 4+1).

### Patch-Management: zentrale Verteilung, Patch vs. Upgrade (2023W)
- **Patch:** Kleine Software-Aktualisierung, die Sicherheitslücken oder Bugs schließt. Meist kumulativ (alle vorherigen Patches enthalten).
- **Upgrade:** Große Versionsänderung (z. B. Windows 10 → Windows 11, Server 2019 → Server 2022). Neue Funktionen, oft neuere Systemvoraussetzungen.
- **Zentrale Verteilung:** Einsatz von WSUS, SCCM oder dritten Tools (Ansible, Jenkins), um Patches auf allen Systemen gleichzeitig auszuspielen.
- **Prüfungsfokus:** Unterschied Patch/Upgrade erkennen und Gründen („Patch = Sicherheitsfix, Upgrade = neue Version mit Funktionen“).

### CSV-Dateiformat: Vor-/Nachteile (2024W)
- **Vorteil:** Plattformunabhängig, einfach lesbar, kann von fast jedem Programm (Excel, Datenbanken, Skripte) geöffnet werden. Gut für den Datenaustausch.
- **Nachteil:** Kein festes Schema (Spaltenreihenfolge kann variieren), keine Typ-Sicherheit (alles ist Text), bei großen Datenmengen Performance-Einbußen, keine Verschlüsselung/Integritätssicherheit von Haus aus.
- **Frage-Format:** „Nenne zwei Vor- und zwei Nachteile von CSV-Dateien für den Datenaustausch.“

### Stromkosten-/Leistungsberechnung: Server-Cluster Stromkosten/Jahr (2023W)
- **Rechenbeispiel (2023W):** Server-Cluster mit unterschiedlichen Leistungsaufnahmen berechnen.
- **Formel:** Leistungsaufnahme (W) × Betriebsstunden/Jahr ÷ Wirkungsgrad = Stromkosten.
- **Beispielrechnung (vereinfacht):** 10 Server je 150 W Dauerbetrieb → 10 × 150 × 24 × 365 = 13.140 kWh/Jahr → bei 0,30 €/kWh → 3.942 €/Jahr.
- **Netzteil-Wirkungsgrad:** Echte Leistungsaufnahme = Netzleistahl / Wirkungsgrad (typisch 0,85–0,95).

### Serverauswahl anhand Spezifikationstabellen-Vergleich (2024W Mailserver, 2025W DB- vs. Virtualisierungsserver)
- **Aufgabe:** Aus gegebenen Tabellen mit CPU, RAM, Speicher, Stromverbrauch, Preis das passende Modell auswählen.
- **Begründungskriterien:** Auslastungsszenario (Mailserver braucht viel I/O, DB-Server braucht viel RAM/CPU, Virtualisierungsserver braucht viele Kerne).
- **Beispiel:** „Für einen Mailserver mit vielen gleichzeitigen POP3/IMAP-Verbindungen wählen Sie das Modell mit hoher E/A-Leistung und vielen Kernen, nicht das mit höchster Roh-CPU-Geschwindigkeit.“

### Non-relationale Datenbanktypen: Dokumenten-, Key-Value-, Graphen-DB (2022W)
- **Dokumenten-DB (Document Store):** Speichert Daten als JSON-/BSON-Dokumente (Beispiel: MongoDB). Flexisches Schema, gut für Inhalte mit variabler Struktur.
- **Key-Value-DB:** Einfaches Mapping aus Schlüssel zu Wert (Beispiel: Redis, DynamoDB). Sehr schnelle Lookups, eignet sich für Caching, Session-Speicherung.
- **Graphen-DB:** Speichert Knoten und Kanten mit Eigenschaften (Beispiel: Neo4j). Ausgezeichnet für vernetzte Daten (Social Network, Empfehlungs-Motoren), Durchläufe (Traversals).

### White-Box- vs. Black-Box-Testverfahren (2025W)
- **White-Box-Test:** Der Tester hat vollen Zugriff auf Quellcode, Architektur und Implementierungsdetails. Basis: Code-Abdeckung, Unit-Tests.
- **Black-Box-Test:** Der Tester hat keinen Zugriff auf den Quellcode. Die Software wird wie eine „ schwarze Box „ behandelt – Eingaben werden gegeben, Ausgaben geprüft. Basis: Funktionalität, Usability, Acceptance Testing.
- **Grau-Box-Test:** Teilweise Zugriff auf Spezifikationen, Architektur, aber nicht auf Quellcode.

### Mobile Device Management (MDM/MDMS): Anbietervergleich, BYOD, Betriebsvereinbarung (2025S)
- **MDM (Mobile Device Management):** Zentrale Verwaltung von Firmengeräten (iOS, Android). Funktionen: App-Kontrolle, Sicherheitsrichtlinien, Fernlöschung, Inventarisierung.
- **BYOD (Bring Your Own Device):** Mitarbeiter nutzen eigene Geräte für dienstliche Zwecke. Erfordert klare Betriebsvereinbarung.
- **Betriebsvereinbarung:** Muss BYOD regeln (Kostenübernahme, Datenschutz, Erstattung von Telefonie/Internet, Sicherheitsanforderungen). Ohne Vereinbarung ist BYOD rechtlich oft unbeschränkt möglich – mit Vereinbarung können Begriffe wie „Berufsgeheimnis“, „Sonderausstattung“ festgehalten werden.
- **Anbietervergleich:** AirWatch (VMware), Intune (Microsoft), Jamf (Apple) – jeweils Stärken bei unterschiedlichen Ökosystemen.

### Least-Privilege-Prinzip & Zero-Trust-Prinzip als moderne Sicherheitskonzepte (2025S)
- **Least-Privilege-Prinzip:** Jeder User / Prozess erhält nur die Berechtigungen, die er für seine konkrete Aufgabe zwingend benötigt. Mehr nicht. reduziert Angriffsfläche.
- **Zero-Trust-Prinzip:** „Never trust, always verify“. Kein Vertrauensaussprech nach Netzwerkzugehörigkeit (auch nicht im internen Netz). Jeder Zugriff wird authentifiziert und autorisiert, egal von wo aus.
- **Unterschied:** Least-Privilege beschränkt die Breite der Berechtigungen; Zero-Trust hinterfragt die Legitimität jeder Verbindung.

### OS-Härtung: unnötige Dienste deaktivieren, Standardkonten umbenennen, Secure Boot, BIOS-Passwort (2022S, 2023S, 2025S, 2026S – wiederkehrend!)
- **Maßnahme 1:** Dienste deaktivieren, die nicht benötigt werden (z. B. Druckerdienst auf Server ohne Drucker, Telnet durch SSH ersetzen).
- **Maßnahme 2:** Standardkonten umbenennen (z. B. Administrator → Admin123 oder einen eigenen Namen vergeben), Passwort ändern.
- **Maßnahme 3:** **Secure Boot** aktivieren (UEFI-Feature, verhindert das Laden nicht signierter BIOS/UEFI-Malware beim Booten).
- **Maßnahme 4:** **BIOS-Passwort** setzen, um unbefugten Zugriff auf Boot-Reihenfolge und Hardware zu verhindern.
- **Prüfungsbezug:** Diese Maßnahmen werden oft in Kombination gefordert („Nennen Sie vier Maßnahmen zur OS-Härtung“).

### Load Balancer (Round Robin/Least Connections/IP-Hashing) & DDoS-Angriffe (2026S)
- **Load Balancer:** Verteilt eingehenden Traffic auf mehrere Server, um Last auszugleichen und Ausfallssicherheit zu gewährleisten.
- **Round Robin:** Jeder Server nacheinander bedient wird. Einfach, gleichmäßige Verteilung bei gleichbleibender Last.
- **Least Connections:** Anfrage geht an den Server mit geringsten aktuellen Verbindungen. Bessere Lastverteilung bei ungleicher Last.
- **IP-Hashing:** Same Client IP → Same Server. Sicherstellt, dass Session-Daten eines Clients immer am selben Standort liegen.
- **DDoS-Angriffe (Distributed Denial of Service):** Viele compromised Systeme überfluten Ziel mit Traffic, um es lahmzulegen. Gegenmaßnahmen: Firewall-Regeln, Load-Balancer-Kapazitätserweiterung, DDoS-Schutz-Services (Cloudflare, Akamai).

### GRUB-Bootloader (Linux) und Fehlerbehebung (2026S, neu/erstmalig)
- **GRand Unified Bootloader (GRUB):** Verwaltung des Boot-Prozesses von Linux. Wählt das Kernel-Image und übergibt Parameter an den Kernel.
- **Konfigurationsdatei:** `/etc/default/grub` und `/boot/grub/grub.cfg`.
- **Typische Fehler:**
  - Beschädigte `grub.cfg` → System startet nicht (schwarzer Bildschirm).
  - Falsche Kernel-Parameter → Boot-Schleife oder fehlende Treiber.
  - **Reparatur:** Live-System verwenden, `grub-install /dev/sda`, `update-grub` ausführen.
- **Rescue-Mode:** System von Rettungs-CD/USB starten, Reparaturbefehle ausführen.

### Bit-Speicherbedarfsberechnung Beispiel: wie viele Bit für 16 TiB in KiB (2021W)
- **Aufgabe:** 16 TiB Speicherplatz in KiB einer `int`-Variablen zuweisen.
- **Rechnung:**
  - 1 TiB = 1024 GiB
  - 1 GiB = 1024 MiB
  - 1 MiB = 1024 KiB
  - Also: 16 TiB = 16 × 1024 × 1024 × 1024 KiB = 17.179.869.184 KiB
- **Wert in Bit:** 17.179.869.184 KiB × 8 Bit/KiB = 137.438.953.472 Bit
- **`int` max:** 2.147.483.647 Bit → reicht bei weitem nicht → **mind. 64 Bit (`long`)** nötig.
- **Prüfungsfokus:** Verständnis für Einheiten-Ketten (Bit → Byte → KiB → MiB → GiB → TiB) und Bereichsberechnung.

### Videoüberwachung/Webcam-Auswahl: Bildauflösung, Datenmengenberechnung, DSGVO/Persönlichkeitsrecht/Löschfristen (2022S)
- **Bildauflösung:** Beispiel 1920 × 1080 Pixel (Full HD). Farbtiefe meist 24 Bit (RGB).
- **Datenmenge pro Bild:** 1920 × 1080 × 24 Bit = 49.766.400 Bit ≈ 6,22 MB pro Bild.
- **Datenmenge pro Sekunde (30 Bilder/s):** 6,22 MB × 30 ≈ 186,6 MB/s ≈ 1,5 Gbit/s.
- **DSGVO/Persönlichkeitsrecht:** Videoüberwachung ist nur unter strengen Bedingungen zulässig (Zweckbindung, Datenminimierung, Information der Betroffenen, Löschfristen). Kameraaufnahmen dürfen nicht länger aufbewahrt werden, als für den Zweck nötig.
- **Löschfristen:** In vielen Unternehmen z. B. 30–72 Stunden, maximal 1 Monat, bevor Aufnahmen automatisch gelöscht werden müssen.

## 3. Übungsaufgabe (im Prüfungsstil, selbst erstellt)

> Wähle die richtige Cluster-Size: Für einen Dateiserver mit vielen kleinen Textdateien (≈ 1 KB pro Datei) soll die Speichereffizienz maximiert werden. Sollte man eher 512 B oder 65.536 B wählen?

<details><summary>Lösung (aufklappen)</summary>

**Antwort: 512 B.**
- Bei vielen kleinen Dateien sorgt eine kleine Cluster-Size dafür, dass jede Datei nah an ihrer eigentlichen Größe liegt (wenig internes Fragmentierung).
- Bei 65.536 B würde jede Datei von 1 KB auf 65.536 KB "aufgerundet" – der Speicherplatz-Verbrauch wäre immens sinnlos.

</details>

> Rechne aus: Ein Cluster hat Size 4 KiB, auf ihm liegen jedoch nur Dateien mit jeweils 500 Byte. Wie viel Speicherplatzverschwendung (in %) entsteht pro Datei, und wie viele Dateien passern auf einen Cluster?

<details><summary>Lösung (aufklappen)</summary>

**Pro Datei:** 4 KiB − 500 Byte = 4096 − 500 = 3596 Byte Verschwendung.
**Prozentual:** (3596 / 4096) × 100 ≈ 87,8 % Verschwendung pro Datei.
**Dateien pro Cluster:** Da jede Datei nur 500 Byte belegt, passen 4096 / 500 ≈ 8,1 → also **8 Dateien** pro Cluster (die 9. Datei würde den Cluster überfüllen).

</details>

## Selbstcheck – kannst du das?

- [ ] Ich erkläre, was Cluster-Size (Zuordnungseinheit) ist und wann kleine/large Choice ist.
- [ ] Ich differenziere zwischen Datendeduplizierung und Komprimierung.
- [ ] Ich nenne Ursachen für Latenz im Cloud-Zugriff.
- [ ] Ich unterscheide horizontale und vertikale Skalierung mit Vor-/Nachteilen.
- [ ] Ich Blue-Green-Deployment und dessen Vorteile erläutern.
- [ ] Ich kann USV-Typen (Online/Offline/Line-Interactive) zuordnen und RTO unterscheiden.
- [ ] Ich kann `SCHTASKS /Create` Befehl lesen und verstehen.
- [ ] Ich Unix-Rechte (`chmod`, `ls -la`, oktale Notation) zuordnen.
- [ ] Ich Patch vs. Upgrade unterscheiden und zentrale Verteilung nennen.
- [ ] Ich CSV-Dateiformat Vor- und Nachteile nennen.
- [ ] Ich Stromkosten/Leistungsberechnung für Server berechnen kann.
- [ ] Ich Serverauswahl anhand Spezifikationstabellen begründen kann.
- [ ] Ich non-relationale Datenbanktypen (Dokumenten/Key-Value/Graphen) zuordne.
- [ ] Ich White-Box- vs. Black-Box-Testverfahren unterscheiden kann.
- [ ] Ich MDM/BYOD/Operatsvereinbarung skizzieren kann.
- [ ] Ich Least-Privilege- und Zero-Trust-Prinzip erläutern kann.
- [ ] Ich OS-Härtungsmaßnahmen (Dienste deaktivieren, Secure Boot, BIOS-Passwort) nennen kann.
- [ ] Ich Load Balancer-Algorithmen (Round Robin/Least Connections/IP-Hashing) unterscheiden kann.
- [ ] Ich DDoS-Angriffe und Gegenmaßnahmen benennt.
- [ ] Ich GRUB-Bootloader und Fehlerbehebung beschreibt.
- [ ] Ich Bit-Speicherbedarfsberechnung (Einheiten-Ketten) durchführen kann.
- [ ] Ich Videoüberwachungs-Datenmengen und DSGVO-Bestimmungen berechnen/erklären kann.

## Weiterführende Ressourcen

- IHK-Prüfungsaufgaben AP2 (GA1) zu Administration und Konfiguration
- Microsoft Docs: Windows Taskplaner
- Linux Documentation Project: GRUB
- DGUV V3: Elektrische Arbeitsmittelprüfung