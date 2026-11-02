# Notfallplan: „Wurde ich gehackt?“ (Incident Response)

**Situation:** Verdacht auf Kompromittierung (Hack) eines IT-Systems.
**Oberstes Gebot:** Nichts eigenmächtig löschen, formatieren oder ausschalten (außer bei akuter Gefahr durch Ransomware). Forensische Spuren (RAM) müssen erhalten bleiben.

## 1. Wie erkenne ich einen Hack? (Indikatoren / IoC)
*   **Performance:** Unerklärliche CPU-/RAM-Auslastung, System reagiert extrem langsam.
*   **Prozesse & Dateien:** Unbekannte Prozesse im Task-Manager, neue Admin-Konten, verschlüsselte Dateien (Ransomware) oder ungewöhnliche Dateiendungen.
*   **Netzwerk:** Ungewöhnlicher ausgehender Datenverkehr (Datenabfluss), Verbindungen zu unbekannten IPs (Command-and-Control-Server), plötzliche Portscans.
*   **Logs & Anmeldungen:** Fehlgeschlagene Login-Versuche (Brute-Force), Anmeldungen zu ungewöhnlichen Zeiten oder von unbekannten Orten.
*   **Antivirus / IDS:** Warnmeldungen des Virenscanners oder Intrusion Detection Systems.

## 2. Wie lege ich den Vorfall fest? (Klassifizierung / Priorisierung)
Sobald der Verdacht bestätigt ist, muss der Vorfall bewertet werden:
*   **Schweregrad:** Handelt es sich um einen isolierten Vorfall (ein PC) oder eine weitreichende Kompromittierung (Server, Domäne)?
*   **Kritikalität:** Welche Systeme sind betroffen? (Produktivserver, Datenbanken, Fileserver, Steuerungssysteme).
*   **Datenabfluss:** Sind personenbezogene Daten (DSGVO-relevant) oder Geschäftsgeheimnisse betroffen?
*   **Eskalation:** Wer muss informiert werden? (IT-Leitung, Geschäftsführung, Datenschutzbeauftragter, ggf. BSI).

## 3. Was danach tun? (Sofortmaßnahmen)
*   **Isolieren:** Betroffenes System sofort vom Netz trennen (LAN-Kabel ziehen). **Nicht** herunterfahren (RAM-Forensik!).
*   **Beweise sichern:** Logs exportieren, RAM-Dump ziehen, Festplatten-Image erstellen (bevor Änderungen vorgenommen werden).
*   **Zugänge sperren:** Betroffene Benutzerkonten deaktivieren, Passwörter zurücksetzen.
*   **Informieren:** Vorgesetzte, IT-Sicherheitsteam, ggf. Datenschutzbeauftragten alarmieren.

## 4. Analyse & Eindämmung
*   **Logs prüfen:** Wann und wie ist der Angreifer eingedrungen? (Phishing, Schwachstelle, gestohlene Zugangsdaten?).
*   **Netzwerksegmentierung:** Ausbreitung verhindern (VLANs trennen, Firewall-Regeln blockieren).
*   **Patchen:** Falls die Schwachstelle bekannt ist, sofort schließen (nach der Analyse).

## 5. Wiederherstellung & Rechtliches
*   **Neuaufsetzen:** System platt machen und aus einem **sauberen Backup** (vor dem Angriff!) wiederherstellen.
*   **DSGVO-Meldefrist:** Bei personenbezogenen Daten muss innerhalb von **72 Stunden** die Aufsichtsbehörde informiert werden (Art. 33 DSGVO).
*   **BSI-Meldung:** Bei kritischer Infrastruktur (KRITIS) ist die Meldung an das BSI verpflichtend.
*   **Betroffene informieren:** Bei hohem Risiko für die Rechte der Personen (Art. 34 DSGVO).

## 6. Nachbereitung
*   **Lessons Learned:** Wie konnte es passieren? Was hat funktioniert? Notfallplan anpassen.
*   **Schulung:** Mitarbeiter sensibilisieren (z.B. gegen Phishing).

## IHK-Merksatz
> „Erkennen (Logs, Performance, Prozesse), Festlegen (Schweregrad, Eskalation), Isolieren (Netz trennen, nicht ausschalten), Beweise sichern, Wiederherstellen. Bei personenbezogenen Daten: 72-Stunden-Meldefrist!“
