# Abnahme, Übergabe und Dokumentation

## In 30 Sekunden

- Die **Abnahme** ist die förmliche Bestätigung des Auftraggebers, dass das Ergebnis **vertragsgemäß** ist (Werkvertrag, § 640 BGB). Grundlage ist das **Pflichtenheft** mit den vereinbarten Abnahmekriterien.
- Folgen der Abnahme: **Fälligkeit der Vergütung**, Beginn der **Gewährleistungsfrist** (Verjährung der Mängelansprüche, § 634a BGB), **Gefahrübergang**, Umkehr der Beweislast für Mängel.
- **Abnahmeprotokoll:** Gegenstand, Datum, Teilnehmer, durchgeführte Tests, festgestellte Mängel (wesentlich/unwesentlich), Fristen zur Beseitigung, Unterschriften.
- **Übergabe:** System, Zugangsdaten, Dokumentation und Verantwortung gehen an den Betrieb bzw. Kunden; Einweisung/Schulung der Anwender und Administratoren.
- **Dokumentation:** Projektdokumentation, technische Dokumentation (Netzplan, Konfiguration), Betriebshandbuch, Benutzerhandbuch, Notfallhandbuch.
- Projektabschluss mit **Abschlussbericht** und **Lessons Learned**.

## Kernwissen

### Ablauf der Abnahme

1. **Abnahmebereitschaft** melden (alle Arbeitspakete erledigt, interne Tests bestanden).
2. **Abnahmetest** gemeinsam mit dem Auftraggeber anhand vorher vereinbarter Testfälle.
3. Mängel erfassen und einstufen:
   - **wesentlicher Mangel** → Abnahme kann verweigert werden
   - **unwesentlicher Mangel** → Abnahme unter Vorbehalt, Mängelliste mit Frist
4. **Abnahmeprotokoll** unterschreiben.
5. Nachbesserung offener Punkte, ggf. Nachabnahme.

Eine Abnahme kann auch **stillschweigend** erfolgen (z. B. produktive Nutzung ohne Beanstandung) oder **fiktiv**, wenn der Auftraggeber eine gesetzte Frist zur Abnahme verstreichen lässt, ohne Mängel zu nennen (§ 640 Abs. 2 BGB).

### Inhalt eines Abnahmeprotokolls

| Punkt | Beispiel |
|---|---|
| Projekt / Vertrag | Einführung WLAN Hotel Seeblick, Vertrag Nr. 2025-117 |
| Datum, Ort, Teilnehmer | 14.07., Empfang, Hr. Lang (Hotel), Fr. Klein (IT-Dienstleister) |
| Abnahmegegenstand | 24 Access Points, Controller, Gast-Portal, Dokumentation |
| geprüfte Kriterien | Ausleuchtung ≥ −67 dBm in allen Zimmern; Gast-VLAN isoliert; 100 Mbit/s je Client |
| Ergebnis | abgenommen mit Vorbehalt |
| Mängel | Zimmer 312: −72 dBm (unwesentlich), Behebung bis 21.07. |
| Unterschriften | Auftraggeber und Auftragnehmer |

### Übergabe an den Betrieb

- Dokumentation und Zugangsdaten (sicher, z. B. im Passwort-Tresor, nicht per unverschlüsselter Mail)
- Einweisung der Administratoren, Schulung der Key User und Anwender
- Übergabe an Support (Ticketkategorien, Wissensdatenbank, Eskalationswege)
- Wartungs- und Supportvertrag bzw. SLA aktivieren
- Monitoring und Backup eingerichtet und getestet
- Altsystem kontrolliert abschalten, Daten löschen oder archivieren

### Arten der Dokumentation

| Dokument | Zielgruppe | Inhalt |
|---|---|---|
| **Projektdokumentation** | Auftraggeber, Prüfungsausschuss | Ausgangslage, Ziele, Planung, Durchführung, Soll-Ist-Vergleich, Fazit |
| **technische Dokumentation / Systemdokumentation** | Administratoren | Netzplan, IP-Konzept, VLANs, Konfigurationen, Versionen, Lizenzen |
| **Betriebshandbuch** | Betrieb, Support | Routineaufgaben, Backup/Restore, Patchprozess, Monitoring |
| **Benutzerhandbuch / Kurzanleitung** | Anwender | Bedienung, FAQ, Ansprechpartner |
| **Notfallhandbuch** | Betrieb, Management | Wiederanlauf, Kontaktliste, Prioritäten (RTO/RPO) |
| **Abnahme- und Testprotokolle** | Auftraggeber, Auditoren | Nachweis der Vertragserfüllung |

**Anforderungen an gute Dokumentation:** aktuell (Versionsnummer, Datum, Autor), vollständig, zielgruppengerecht, auffindbar, geschützt (vertrauliche Daten wie Passwörter getrennt).

### Projektabschluss

- **Abschlussbericht:** Zielerreichung, Termine, Kosten (Soll-Ist), offene Punkte
- **Lessons Learned:** Was lief gut, was schlecht, was machen wir künftig anders?
- **Entlastung** der Projektleitung, Auflösung des Projektteams, Archivierung der Unterlagen
- Projektkostenabrechnung und ggf. Rechnungsstellung

## Prüfungsfrage

**Frage:** Nach der Installation eines neuen Serversystems beim Kunden steht die Abnahme an. Erläutern Sie drei rechtliche Folgen der Abnahme und nennen Sie vier Inhalte eines Abnahmeprotokolls.

**Musterantwort:** Mit der Abnahme wird (1) die **Vergütung fällig**, der Auftragnehmer kann die Schlussrechnung stellen; (2) beginnt die **Gewährleistungsfrist** für Mängelansprüche; (3) geht die **Gefahr** des zufälligen Untergangs auf den Auftraggeber über, und für spätere Mängel muss der Auftraggeber beweisen, dass sie schon bei der Abnahme vorlagen. Ein Abnahmeprotokoll enthält u. a. **Datum und Teilnehmer**, den **Abnahmegegenstand**, die **durchgeführten Prüfungen mit Ergebnis**, eine **Liste festgestellter Mängel mit Frist zur Beseitigung** sowie die **Unterschriften** beider Parteien.

## Falle

- **Abnahme = Lieferung:** Die Lieferung ist der physische Übergang; die Abnahme ist die Billigung als vertragsgemäß.
- **Abnahme wegen jedes Kleinmangels verweigern:** Bei unwesentlichen Mängeln darf die Abnahme nicht verweigert werden (§ 640 Abs. 1 BGB).
- **Passwörter in der Dokumentation:** Zugangsdaten gehören in einen geschützten Passwortspeicher.
- **Dokumentation erst am Ende:** Begleitend dokumentieren, sonst fehlen Details.
- **Lessons Learned weglassen:** gehört zu jedem Projektabschluss (und zur Projektdokumentation der AP2).

## Merksatz

**"Ohne Protokoll keine Abnahme, ohne Abnahme kein Geld, ohne Dokumentation kein Betrieb."**

## Verwandt

- [Lastenheft & Pflichtenheft](lastenheft-pflichtenheft.md) – Abnahmekriterien
- [Softwaretest: White-Box & Black-Box](../UML-Code-SQL/Pseudocode/softwaretest-white-black-box.md) – Abnahmetest
- [Projektmanagement-Grundlagen](projektmanagement-grundlagen.md) – Abschlussphase
- [Soll-Ist-Vergleich](soll-ist-vergleich.md) – im Abschlussbericht
- [Qualitätsmanagement](qualitaetsmanagement.md) – Lessons Learned, KVP
- [Ticketsystem, SLA & Eskalation](ticketsystem-sla-eskalation.md) – Übergabe an den Support
- [Lektion 23: Recht & Steuern](../analyse-der-exams/L23_WISO_Recht_Steuern.md) – Vertragsarten, Gewährleistung
