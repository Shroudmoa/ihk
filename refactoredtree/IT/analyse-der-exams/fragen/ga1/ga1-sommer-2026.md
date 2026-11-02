## 1. Aufgabe (Hardware, Härtung & Linux-Boot)

### aa) Netzteil-Leistungsbedarf berechnen
- **Leistungsaufnahme aller Komponenten:** Mainboard 1 × 40 W = 40 W; CPU 2 × 90 W = 180 W; RAM 8 × 4 W = 32 W; HDD 4 × 10 W = 40 W; NIC 2 × 6 W = 12 W. Summe = 40 + 180 + 32 + 40 + 12 = **304 W**.
- **Puffer 20 %:** 304 W × 1,20 = **364,8 W** (Puffer auf die reine Komponentenlast).
- **Wirkungsgrad 90 %:** Das Netzteil muss am Eingang mehr liefern, um am Ausgang 364,8 W bereitzustellen. Benötigte Netzteil-Nennleistung = 364,8 W / 0,90 = **405,33 W**.
- **Marktübliche Größe (50-W-Raster):** 405,33 W → nächstgrößer verfügbar ist **450 W**.

### ab) UEFI/BIOS-Härtung – zwei Einstellungen
- **Einstellung 1: Boot-Reihenfolge fixieren und Boot von externen Medien deaktivieren.**  
  *Erläuterung:* Verhindert Booten von USB/CD durch Angreifer (z.B. Live-Linux zum Umgehen von Passwörtern) und reduziert Angriffsfläche auf Firmware-Ebene.
- **Einstellung 2: BIOS/UEFI-Administrator-Passwort und Secure Boot aktivieren.**  
  *Erläuterung:* Admin-Passwort verhindert unbefugte Änderungen der Firmware-Einstellungen; Secure Boot erlaubt nur signierte Bootloader, schützt vor Bootkits/Rootkits.

### ac) Zwei weitere TOMs (Technisch-Organisatorische Maßnahmen)
- **Beispiel 1: Verschlüsselung der Festplatten (BitLocker/LUKS) und Netzwerkverschlüsselung (TLS/IPsec).**  
  *Erläuterung:* Schützt Vertraulichkeit (Confidentiality) bei Diebstahl oder Abhören; Daten sind ohne Schlüssel unlesbar.
- **Beispiel 2: Zutrittskontrolle + Videoüberwachung im Serverraum (physische Sicherheit).**  
  *Erläuterung:* Verhindert unbefugten physischen Zugriff, dokumentiert Vorfälle, schützt vor Manipulation und Diebstahl von Hardware (Availability/Integrity).

### ba) GRUB – Funktion und Zweck
- **Funktion:** GRUB ist ein Bootmanager/Bootloader. Er lädt den Kernel des Betriebssystems in den Speicher und übergibt die Kontrolle an das OS.
- **Zweck:** Auswahl zwischen mehreren Betriebssystemen oder Kernel-Versionen (Multiboot). GRUB kann auch Nicht-Linux-OS (z.B. Windows) starten und findet den Linux-Kernel anhand von Laufwerks-, Partitionsnummer und Dateiname.
- **Analogie:** GRUB ist der "Türsteher" zwischen Firmware (UEFI/BIOS) und OS-Kernel.

### bb) GRUB-Fehler „file not found“ – Ursache & Behebung
- **Ursache:** GRUB findet die Kernel-Datei oder das GRUB-Konfigurationsverzeichnis nicht. Häufige Gründe: beschädigtes Dateisystem, geänderte Partitionierung (UUID verschoben), fehlendes `/boot`-Verzeichnis, fehlgeschlagenes Update oder falscher Pfad in `grub.cfg`.
- **Fehlerbehebung:** Mit einem Live-System booten, Partition mounten, `grub-install` und `update-grub` (bzw. `grub2-mkconfig`) ausführen. Alternativ in der GRUB-Rescue-Konsole `set root=(hdX,gptY)`, `set prefix=...`, `insmod normal`, `normal` ausführen und GRUB neu installieren.

---

## 2. Aufgabe (Webserver, Load Balancer & DDoS)

### aa) Zwei Aufgaben einer Certificate Authority (CA)
- **Identitätsprüfung/Validierung:** Die CA prüft die Identität des Antragstellers (Domain-Inhaberschaft, Organisation) bevor sie ein Zertifikat ausstellt. Verhindert Missbrauch (Spoofing).
- **Signatur und Vertrauenskette:** Die CA signiert den öffentlichen Schlüssel mit ihrem privaten Schlüssel und stellt das Zertifikat aus. Clients vertrauen der CA über den Root-Store; die Chain of Trust wird aufgebaut.
- **Weitere:** Widerrufsmanagement (CRL/OCSP), Ausstellung und Erneuerung von Zertifikaten.

### ba) Funktionsweise Load Balancer
- **Prinzip:** Ein Load Balancer verteilt eingehende Client-Anfragen auf mehrere Backend-Server (Server-Pool). Er tritt als virtueller Endpunkt auf (VIP) und sorgt für Lastverteilung, Ausfallsicherheit (Health Checks) und Skalierbarkeit.

### bb) Load-Balancing-Methoden
- **Round Robin:** Anfragen werden reihum gleichmäßig auf die Server verteilt (Server 1, 2, 3, 1, 2, 3…). Einfach, aber ignoriert aktuelle Serverlast und -kapazität.
- **Least Connections:** Die Anfrage geht an den Server mit den aktuell wenigsten aktiven Verbindungen. Gut für heterogene Lasten und langlaufende Sessions.
- **IP-Hashing:** Die Client-IP wird gehasht und einem Server zugeordnet. Sorgt für Session-Persistenz (gleicher Client → gleicher Server), aber ungleichmäßige Verteilung möglich.

### c) DDoS-Angriff (Distributed Denial of Service)
- **Prinzip:** Viele verteilte Systeme (Botnet) senden massenhaft Anfragen an ein Ziel (z.B. Webserver) oder überlasten die Bandbreite. Ziel ist die Verfügbarkeit (Availability) zu brechen; legitime Clients werden blockiert. Abwehr: Rate Limiting, CDN/DDoS-Protection, Traffic-Scrubbing, Anycast.

---

## 3. Aufgabe (Cloud-Modelle & Verfügbarkeit)

### aa) Drei Cloud-Servicemodelle
- **IaaS (Infrastructure as a Service):** Bereitstellung von virtuellen Maschinen, Storage, Netzwerk. Kunde verwaltet OS und Anwendungen. *Beispiel:* AWS EC2, Azure Virtual Machines.
- **PaaS (Platform as a Service):** Bereitstellung einer Entwicklungs-/Laufzeitumgebung (Runtime, Middleware, DB). Kunde deployt nur die Anwendung. *Beispiel:* Google App Engine, Azure App Service.
- **SaaS (Software as a Service):** Fertige Anwendung über das Internet, Kunde nutzt nur die Software. *Beispiel:* Microsoft 365, Salesforce, Google Workspace.

### ab) Deployment-Modell – Empfehlung
- **Empfehlung:** **Private Cloud** (oder Hybrid Cloud). Begründung: Strenge deutsche Sicherheits-/Compliance-Anforderungen (DSGVO, BDSG) erfordern volle Kontrolle über Datenstandort und Zugriff.
- **Vorteil:** Hohe Kontrolle, Datenhoheit, individuelle Compliance, Isolation.
- **Nachteil:** Hohe Kosten (Hardware, Betrieb, Personal), geringere Skalierbarkeit/Elastizität als Public Cloud.

### b) RTO & RPO
- **RTO (Recovery Time Objective):** Maximal akzeptable **Ausfallzeit** bis zur Wiederherstellung des Dienstes. Beispiel: „Webshop muss nach 2 Stunden wieder online sein."
- **RPO (Recovery Point Objective):** Maximal akzeptabler **Datenverlust** in Zeit gemessen. Definiert das maximale Alter des letzten Backups. Beispiel: „RPO = 15 Minuten" → maximal 15 Minuten Datenverlust erlaubt.

---

## 4. Aufgabe (Programmierung – Task-Scheduling)

### aa) Schreibtischtest – Array `tasksPriol = {5,7,4,8,3,7,3}`
- **Initial:** `indexMaxPriol = 0`, `maxPriorität = 10`.
- **Schritt 1 (i=0, Wert 5):** 5 < 10 → maxPriorität=5, indexMaxPriol=0.
- **Schritt 2 (i=1, Wert 7):** 7 < 5? Nein → keine Änderung.
- **Schritt 3 (i=2, Wert 4):** 4 < 5? Ja → maxPriorität=4, indexMaxPriol=2.
- **Schritt 4 (i=3, Wert 8):** 8 < 4? Nein → keine Änderung.
- **Schritt 5 (i=4, Wert 3):** 3 < 4? Ja → maxPriorität=3, indexMaxPriol=4.
- **Schritt 6 (i=5, Wert 7):** 7 < 3? Nein → keine Änderung.
- **Schritt 7 (i=6, Wert 3):** 3 < 3? Nein (nicht kleiner, gleich) → keine Änderung. **Wichtig:** Der Code prüft `<`, nicht `<=`; dadurch bleibt der erste Index bei Gleichstand.

| Schritt | indexMaxPriol | maxPriorität |
|---|---|---|
| Start | 0 | 10 |
| 1 | 0 | 5 |
| 2 | 0 | 5 |
| 3 | 2 | 4 |
| 4 | 2 | 4 |
| 5 | 4 | 3 |
| 6 | 4 | 3 |
| 7 | 4 | 3 |

- **Ergebnis:** `indexMaxPriol = 4` → `taskNummer = 5`. Task 5 hat die höchste Priorität (Wert 3), und bei Gleichstand (Task 7, Wert 3) gewinnt der kleinere Index.

### ab) Zweck der Zeile 12
- **Zeile 12:** `int taskNummer = indexMaxPriol + 1;`
- **Zweck:** Umrechnung vom 0-basierten Array-Index auf die 1-basierte Task-Nummer. Der Index 4 entspricht Task 5. Rückgabe der Task-Nummer für die weitere Verarbeitung.

### b) Zwei Merkmale/Eigenschaften von Arrays
- **Vorteil – Direkter Zugriff (Random Access):** Zugriff über Index in O(1); sehr schnell für wahlfreien Zugriff. Fester zusammenhängender Speicherbereich, cache-freundlich.
- **Nachteil – Statische Größe:** Länge ist bei der Deklaration festgelegt und kann nicht dynamisch geändert werden. Vergrößerung erfordert neues Array und Kopieren (wie in Prüfung 2025).
- **Weitere:** Homogener Datentyp (nur ein Typ pro Array); Einfügen/Löschen in der Mitte ist O(n) wegen Verschiebeaufwand.

### c) Programm im laufenden Betrieb beenden
- **Möglichkeit:** Task-Manager (Windows) oder `kill`/`kill -9` (Linux) mit der Prozess-ID (PID). Unter Windows: `taskkill /PID <PID> /F`. Unter Linux: `ps aux` zur PID-Ermittlung, dann `kill -SIGKILL <PID>`.
- **Hinweis:** SIGKILL beendet den Prozess sofort, ohne Aufräumen; Ressourcen werden freigegeben. Alternativ SIGTERM für sauberes Beenden.

---

**Merksatz:** IHK-Aufgaben sind **Kausalitätsketten** – wer Netzteil-Wirkungsgrad, GRUB-Chain-of-Trust, Load-Balancing-Algorithmen und RTO/RPO nicht als Hard-Facts sitzen hat, verliert Punkte an der Oberfläche. Lerne Standards als Vokabeln, nicht als Prosa.
