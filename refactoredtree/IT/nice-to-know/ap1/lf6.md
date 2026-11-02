# Klausurvorbereitung LF6

## Lektion 1 – Servicegrundlagen

### Service
*   Mittel zur Mehrwertlieferung für den Kunden (Kunde erreicht Ziel optimiert, ohne eigene Kosten/Risiken).
*   IT-Unternehmen stellt Ressourcen bereit, um Kundenziele zu erreichen.
*   Service ist ein Prozess (nach Verkauf/Bereitstellung), keine einzelne Dienstleistung.

### Gewährleistung, Garantie, Service, Kulanz
*   **Gewährleistung (gesetzlich):** 2 Jahre Neuware, 12 Monate Gebrauchtware. Deckt Mängel bei Kauf ab. Beweislastumkehr nach 12 Monaten.
*   **Garantie:** Freiwillige Selbstverpflichtung des Herstellers (z.B. 3 Jahre, Bring-In).
*   **Servicedienstleistungen:** Zusätzliche Services (Installation, Schulung, Wartung, Support). Oft kostenpflichtig.
*   **Kulanz:** Freiwillige, kundenorientierte Geste ohne rechtliche Pflicht. Starker Mehrwert, Kundenbindung.

### Pareto-Prinzip (80/20)
*   80% der Ergebnisse durch 20% der Ursachen.
*   Kleine Anzahl von Kunden/Situationen hat signifikanten Einfluss auf den Erfolg (wichtig für Kulanzentscheidungen).

### Gewährleistung B2B vs. B2C
*   **B2C:** Starr, nicht modifizierbar. Verjährungsfrist nicht kürzbar (2 Jahre/12 Monate).
*   **B2B:** Verjährungsfrist auf 1 Jahr senkbar. Gewährleistung auf Gebrauchtware ausschließbar. Mängelrügepflicht (unverzügliche Untersuchung).

### IMAC/R/D-Zyklus
*   **Install:** Konfiguration, Inbetriebnahme, Bereitstellung.
*   **Move:** Verpackung, Transport, Umzug, Neuinstallation.
*   **Add:** Hinzufügen von Hard-/Software, Berechtigungen.
*   **Change:** Update, Upgrade, Neuinstallation, Sicherheitsmaßnahmen.
*   **Remove:** Deinstallation, Datenlöschung, Abbau.
*   **Dispose:** Entsorgung, Recycling, Remarketing, Refresh.

### Kundenzuschnitt von Services
*   Individuelle Anforderungen, effiziente Ressourcennutzung, Kundenzufriedenheit, bessere Kommunikation, Wettbewerbsvorteil, Flexibilität.

### Helpdesk-Vorteile (mit Software)
*   Schnelle/organisierte Hilfe, Ticketsystem (Vorgangsbearbeitung), Überblick über Ressourcen/Verträge, Remote-Support, zentrales Monitoring, Nachverfolgung, Service-Level/Eskalation, automatische Störungsreaktion.

---

## Lektion 2 – Verträge (SLA, Rahmenvertrag)

### SLA vs. IT-Dienstleistungsrahmenvertrag
*   **SLA:** Schriftliche Vereinbarung zur Qualitätssicherung. Regelt Leistungsumfang, Reaktionszeit, Schnelligkeit, Dienstgüte, Eigenleistungen des Kunden.
*   **Rahmenvertrag:** Allgemeine Geschäftsbedingungen, Hauptleistungen des Dienstes.

### Sorgfalt bei Vertragsverhandlung
*   Sicherstellen, dass Leistungen/Bedingungen eingehalten werden können (sonst Vertragsstrafen).
*   Wichtigkeit des Kunden beachten (für Kulanz).

### SLA-Inhalte
*   Verfügbarkeit/Leistungsfähigkeit, Reporting-Pflicht, Störungsbeseitigung, Sanktionen, Laufzeit, Verantwortlichkeiten, Kommunikation, Servicezeiten, Ausnahmen/Flexibilitäten.

### Verfügbarkeitsberechnung
*   **Minimal erwartete Verfügbarkeit:** Gesamtstunden – Max. erlaubte Ausfallzeit.
*   **Max. erlaubte Ausfallzeit:** 100% – Verfügbarkeit in %.
*   **Nicht geschuldete Verfügbarkeit:** Gesamtstunden + Max. Ausfallzeit.

### Reporting & Monitoring
*   Zweck: Offenlegung der Verfügbarkeit, Überprüfung durch Kunden, Strafzahlungen.
*   **Messgrößen:** Antwortzeitverhalten, Datendurchsatz, Paketverzögerung, Round Trip Delay (RTD), Paketverlustrate.

### Sanktionen bei SLA-Verletzung
*   Schadensersatz (auch pauschal), Vertragsstrafen, Kündigung (wichtiger Grund), Vertragsüberprüfung, Rufschädigung.

---

## Lektion 3 – Support-Level & Serviceanliegen

### Support-Level
*   **First-Level:** Entgegennahme, Soforthilfe bei einfachen Problemen, Weiterleitung komplexer Anfragen.
*   **Second-Level:** Komplexere Anfragen, spezifisches Fachwissen, detaillierte Analyse, mehr Berechtigungen.
*   **Third-Level:** Höchste Ebene, hochtechnische/spezifische Probleme, externe Servicemitarbeiter (Hardware/Software/Cloud).

### Serviceanliegen
*   **Incident:** Ungeplante Service-Unterbrechung. Schnelle Reaktion nötig.
*   **Service Request:** Geplante Anfrage (z.B. Passwort-Reset). Keine Betriebsunterbrechung.
*   **Change Request:** Geplante Änderung an Systemen/Prozessen/Services (tiefgreifend).
*   **Major Incident:** Schwerwiegender Incident mit großer Auswirkung.
*   **Problem:** Ursache eines oder mehrerer Incidents (noch unbekannt).
*   **Workaround:** Behelfslösung/Übergangslösung, ermöglicht Betrieb bis zur dauerhaften Lösung.

### Prozessuale Behandlung
*   **Incidents:** Incident-Management, Ticketsystem, Kategorisierung, Priorisierung.
*   **Service Requests:** Request Fulfilment, oft zeitunkritisch, First-Level.
*   **Change Requests:** RfC-Ticket, Eskalation an höhere Level, Change-Management (Standard, Level 1-3).

### Incident-Prozess (Schritte)
1. Meldung/Identifizierung.
2. Protokollierung im Ticketsystem.
3. Kategorisierung/Klassifizierung.
4. Priorisierung.
5. Erste Diagnose (1st Level).
6. Eskalation (2nd/3rd Level).
7. Untersuchung/Diagnose.
8. Lösung/Wiederherstellung.
9. Ticket schließen & Zufriedenheit erfragen.

### Priorisierung
*   Nach **Dringlichkeit** (Wie schnell muss gelöst werden?) und **Auswirkung** (Wie umfangreich? Welcher Schaden?).

### Reaktionszeit (Einflussfaktoren)
*   Rechtlich (Gewährleistung, SLA, Kulanz), sachlich (Art des Anliegens, Priorität), organisatorisch (Ressourcen, Kalender), sonstig (Versicherung, Sicherheit).

### Eskalation
*   Wenn 1st Level nicht allein lösen kann, schwerwiegendes Problem (Expertenwissen), Reaktionszeit nicht einhaltbar, 2nd Level nicht lösen kann.

---

## Lektion 4 – Kommunikation & Arbeitsorganisation

### Kommunikationsarten
*   **Synchron:** Abgestimmt (Chat, Face-to-Face).
*   **Asynchron:** Zeitlich versetzt (E-Mail, Forum).
*   **Verbal:** Sprache.
*   **Nonverbal:** Mimik, Gestik, Haltung.
*   **Hyperkommunikation:** Gemeinsames Arbeiten an Text.
*   **Metakommunikation:** Über die Kommunikation reden.

### Eisenhower-Modell
*   Kategorisierung nach Wichtigkeit (Zielerreichung) und Dringlichkeit (Termin).
*   **Wichtig & dringlich:** Sofort selbst erledigen.
*   **Wichtig, nicht dringlich:** Terminieren, selbst erledigen.
*   **Dringlich, nicht wichtig:** Delegieren.
*   **Weder wichtig noch dringlich:** Ablegen (Papierkorb).
*   *Kritik:* Definition von wichtig/dringlich subjektiv, häuft sich oft an.

### Schulz von Thun (4 Seiten einer Nachricht)
1.  **Sachinhalt:** Worüber informiere ich?
2.  **Selbstoffenbarung:** Was gebe ich von mir kund?
3.  **Beziehung:** Was halte ich vom Empfänger?
4.  **Appell:** Wozu möchte ich den Empfänger veranlassen?

### Sieben-Phasen-Modell (Change Management)
1.  **Schock:** Überraschung, Angst, sinkende Produktivität.
2.  **Verneinung:** Ablehnung, Zusammenhalt gegen Veränderung.
3.  **Rationale Einsicht:** Wandel als notwendig erkennen, oberflächliche Änderungen.
4.  **Emotionale Akzeptanz:** Wendepunkt, Neuorientierung beginnt.
5.  **Ausprobieren/Lernen:** Neugier, Lernen durch Erfolg/Misserfolg.
6.  **Erkenntnis:** Veränderung hat Gutes, Integration in Alltag.
7.  **Integration:** Neue Verhaltensweisen selbstverständlich.

### Kepner-Tregoe (Problemanalyse)
*   Strukturierte Abgrenzung IST / IST-NICHT.
*   Schritte: Problem benennen, System, Vergleichssystem, Wo/Wann/Umfang, Unterschiede, Veränderungen, Ursachenhypothesen, Plausibilität prüfen, Ursache beseitigen.

### Service-Desk-Tipps
*   Gesprächsphasen befolgen (Vorbereitung, Einstieg, Info, Rückfragen, Abschluss).
*   Help-Desk-Fragen: Einzel-/Multiproblem? Logfiles? Gerätetyp? Kunden einbeziehen. Status prüfen lassen.

---

## Lektion 5 – Analyse & Wissensmanagement

### Knowledge Base für Known Errors
*   Wissensdatenbank mit Lösungen/Kategorien. Intern/extern. Known Errors & Workarounds für 1st Level hinterlegen (schnelle Behebung).

### Analysemethoden
*   **5-Warum-Fragen:** Für Probleme ohne bekannte Lösungsstrategie.
*   **Kepner-Tregoe:** Vergleich Problem- vs. Vergleichssystem, Unterschiede, Hypothesen.
*   **DMAIC:** Define, Measure, Analyse, Improve, Control (für aufwendige Problemlösungen).
*   **Problemlösungsmatrix (Ursachen-Wirkungsmatrix):** Systematischer Abgleich IST/SOLL (Was? Wo? Wann? Wie viel?).
*   **Ishikawa-Diagramm (Fischgräte):** Haupt-/Nebeneinflussgrößen (Ursachen) grafisch darstellen. Für umfangreiche Probleme.

### IT-Monitoring-Systeme
*   Überwachung, Störungsanalyse, Prävention, Dashboards, Alarme bei Grenzwertüberschreitung, Daten für Analyse/präventives Agieren.

### Leasing
*   Finanzierungsmethode („Mietkauf“). Leasinggeber bleibt Eigentümer, Leasingnehmer zahlt Raten, nutzt Objekt.
*   **Vorteile:** Geringe Anfangsinvestition, Flexibilität, steuerliche Vorteile, Risikominderung.
*   **Nachteile:** Langfristig höhere Kosten, kein Eigentum, Vertragsbindung.
*   **Formen:** Operativ (Miete, Rückgabe), Finanziell (Ziel Eigentumserwerb).
*   **Berechnung:** Rate = (Anschaffungswert – Restwert) / Laufzeit in Monaten + (Anschaffungswert * Monatszins).

---

## Lektion 6 – Qualitätssicherung & Wartung

### Ziele der Qualitätssicherung
*   Fehlervermeidung, Einsparung von Gewährleistung/Nachbesserung, gute Kundenbeziehung, Imageverbesserung.

### Instandhaltungsvarianten
*   **Periodisch:** Backups, Festplattenüberwachung, Updates.
*   **Präventiv:** Monitoring, Server-/Cloud-/Umgebungsmonitoring.
*   **Prädiktiv:** Kontrollierende/optimierende Maßnahmen, High Performance Monitoring.
*   **Problembasiert (reaktiv):** Nach Auftreten eines Problems.

### 360-Grad-Feedback
*   Feedback von Vorgesetzten, Kollegen, Mitarbeitern, Kunden. Maßnahmen ableiten (z.B. Training, Coaching).
