# Stakeholder- und Risikoanalyse

## In 30 Sekunden

- **Stakeholder** sind alle Personen und Gruppen, die am Projekt **beteiligt**, davon **betroffen** oder daran **interessiert** sind – intern (Geschäftsführung, Anwender, Betriebsrat, Datenschutzbeauftragter) wie extern (Kunden, Lieferanten, Behörden).
- **Stakeholderanalyse:** identifizieren → Einfluss und Einstellung bewerten → Maßnahmen planen (Einfluss-Interesse-Matrix).
- **Risiko** = mögliches Ereignis mit negativer Wirkung auf Ziele. **Risikowert = Eintrittswahrscheinlichkeit × Schadensausmaß**.
- **Risikomatrix** ordnet Risiken nach Wahrscheinlichkeit und Auswirkung (grün/gelb/rot).
- Strategien: **vermeiden, vermindern, übertragen (versichern, Vertrag), akzeptieren** – plus Notfallplan.
- Risikomanagement ist ein **Kreislauf**: identifizieren → bewerten → steuern → überwachen.

## Kernwissen

### Stakeholder identifizieren (Beispiel: Einführung eines neuen ERP-Systems)

| Stakeholder | Interesse / Erwartung | Einfluss | Einstellung |
|---|---|---|---|
| Geschäftsführung | Kostensenkung, Termin | hoch | positiv |
| Buchhaltung (Anwender) | einfache Bedienung, keine Mehrarbeit | mittel | skeptisch |
| Betriebsrat | Leistungs- und Verhaltenskontrolle ausgeschlossen (§ 87 BetrVG) | hoch | neutral |
| Datenschutzbeauftragter | DSGVO-Konformität, AV-Vertrag | mittel | neutral |
| IT-Abteilung | Betreibbarkeit, Schnittstellen | mittel | positiv |
| Softwarelieferant | Auftrag, Referenzkunde | mittel | positiv |
| Steuerberater, Finanzamt | GoBD-konforme Buchführung | gering | neutral |

### Einfluss-Interesse-Matrix

| | geringes Interesse | hohes Interesse |
|---|---|---|
| **hoher Einfluss** | **zufriedenstellen** (regelmäßig informieren, nicht überlasten) | **eng einbinden** (Schlüsselakteure, aktiv beteiligen) |
| **geringer Einfluss** | **beobachten** (minimaler Aufwand) | **informieren** (Newsletter, Infoveranstaltung) |

Maßnahmen für Skeptiker: frühe Einbindung (Key User), Schulungen, Pilotphase, transparente Kommunikation, Feedbackkanäle.

### Risikobewertung

Skala jeweils 1 (gering) bis 5 (sehr hoch).

| Nr. | Risiko | Wahrscheinlichkeit | Auswirkung | Risikowert | Maßnahme (Strategie) |
|---|---|---:|---:|---:|---|
| R1 | Datenverlust bei der Migration | 2 | 5 | **10** | Vollbackup und Testmigration vorab (vermindern) |
| R2 | Lieferverzug der Hardware | 3 | 3 | **9** | früh bestellen, Vertragsstrafe vereinbaren (vermindern/übertragen) |
| R3 | Schlüsselperson fällt aus | 2 | 4 | **8** | Wissen dokumentieren, Vertretung einarbeiten (vermindern) |
| R4 | geringe Akzeptanz der Anwender | 4 | 3 | **12** | Key User, Schulung, Pilotgruppe (vermindern) |
| R5 | kleiner Darstellungsfehler im Bericht | 3 | 1 | **3** | nach Go-Live beheben (akzeptieren) |

**Priorität:** R4 (12) → R1 (10) → R2 (9) → R3 (8) → R5 (3).

### Risikomatrix

| Auswirkung ↓ / Wahrscheinlichkeit → | 1–2 (gering) | 3 (mittel) | 4–5 (hoch) |
|---|---|---|---|
| **4–5 (hoch)** | gelb: R1, R3 | rot | rot |
| **3 (mittel)** | grün | gelb: R2 | rot: R4 |
| **1–2 (gering)** | grün | grün: R5 | gelb |

### Strategien

| Strategie | Bedeutung | IT-Beispiel |
|---|---|---|
| **vermeiden** | Ursache beseitigen, auf riskante Aktivität verzichten | keine Migration im Jahresabschluss |
| **vermindern** | Wahrscheinlichkeit oder Auswirkung senken | Backup, Redundanz, Tests, Schulung |
| **übertragen** | Risiko auf Dritte verlagern | Versicherung, SLA mit Vertragsstrafe, Outsourcing |
| **akzeptieren** | bewusst tragen, ggf. Rückstellung/Puffer | geringe Risiken mit kleinem Schaden |

### Weitere Werkzeuge

- **SWOT-Analyse:** Stärken, Schwächen (intern), Chancen, Risiken (extern).
- **Risikoregister:** Liste aller Risiken mit Verantwortlichem, Status, Frühwarnindikator.
- **FMEA** (Fehlermöglichkeits- und Einflussanalyse): Risikoprioritätszahl = Auftreten × Bedeutung × Entdeckbarkeit.

## Prüfungsfrage

**Frage:** Bei der Umstellung des Mailservers auf eine Cloud-Lösung sind folgende Risiken bekannt: (1) Mails gehen während der Umstellung verloren (Wahrscheinlichkeit 2, Auswirkung 5), (2) Anwender finden Funktionen nicht (4, 2), (3) Internetanbindung fällt aus (2, 4). Bewerten Sie die Risiken, bilden Sie eine Rangfolge und nennen Sie je eine Maßnahme.

**Musterantwort:** Risikowerte: (1) 2 × 5 = **10**, (2) 4 × 2 = **8**, (3) 2 × 4 = **8**. Rangfolge: (1) vor (2) und (3). Maßnahmen: (1) **vermindern** durch Umstellung am Wochenende, niedrige TTL der MX-Records vorab, Vollsicherung der Postfächer und Parallelbetrieb mit Weiterleitung; (2) **vermindern** durch Kurzschulung und Anleitung vor dem Go-Live; (3) **vermindern** durch eine zweite Internetleitung eines anderen Providers (Redundanz) bzw. **übertragen** über ein SLA mit Entstörzeit.

## Falle

- **Nur interne Stakeholder nennen:** Betriebsrat, Datenschutzbeauftragter, Kunden und Behörden werden oft vergessen.
- **Risiko und Problem verwechseln:** Ein Risiko kann eintreten; ein eingetretenes Risiko ist ein Problem (Issue).
- **Risikowert addieren statt multiplizieren.**
- **"Risiko vermeiden" für alles:** Nicht jedes Risiko lässt sich vermeiden; Gegenmaßnahmen müssen wirtschaftlich sein.
- **Analyse nur zu Projektbeginn:** Risiken ändern sich, das Register muss regelmäßig aktualisiert werden.

## Merksatz

**"Wer ist betroffen, was kann schiefgehen, wie schlimm, wie wahrscheinlich – und was tun wir dagegen?"** Risiko = Wahrscheinlichkeit × Auswirkung.

## Verwandt

- [Projektmanagement-Grundlagen](projektmanagement-grundlagen.md)
- [Soll-Ist-Vergleich](soll-ist-vergleich.md) – Abweichungen als Frühwarnung
- [Qualitätsmanagement](qualitaetsmanagement.md) – FMEA, PDCA
- [ITIL: Incident, Problem, Change](itil-incident-problem-change.md) – Risiken im Change-Prozess bewerten
- [Schutzziele](../FiSi-Administrator/Sicherheit/schutzziele.md) – IT-Risiken nach Vertraulichkeit, Integrität, Verfügbarkeit
- [DSGVO & TOM](DSGVO/dsgvo-tom.md) – Datenschutzrisiken
- [Betriebsrat & JAV](../../Wirtschaft/Mitbestimmung/betriebsrat-jav.md) – Mitbestimmung als Stakeholder-Thema
