# Projektstrukturplan (PSP)

## In 30 Sekunden

- Der **Projektstrukturplan (PSP, engl. Work Breakdown Structure, WBS)** zerlegt das Projekt **hierarchisch** in Teilprojekte, Teilaufgaben und **Arbeitspakete**.
- Er ist der "Plan der Pläne": Aus ihm entstehen Aufwandsschätzung, Netzplan, Gantt-Diagramm, Ressourcen- und Kostenplan.
- **Arbeitspaket** = kleinste Einheit im PSP, mit klarem Ergebnis, Verantwortlichem, Aufwand, Dauer und Kosten.
- Gliederung **objektorientiert** (nach Bauteilen), **funktionsorientiert** (nach Tätigkeiten), **phasenorientiert** (nach Projektphasen) oder **gemischt**.
- **100-%-Regel:** Der PSP enthält **alle** Arbeiten des Projekts – nicht mehr und nicht weniger.
- Der PSP enthält **keine Zeitachse und keine Reihenfolge**; die kommen erst im Netzplan.

## Kernwissen

### Ebenen und Codierung

| Ebene | Inhalt | Code |
|---|---|---|
| 0 / 1 | Gesamtprojekt | `1` |
| 2 | Teilprojekt oder Phase | `1.2` |
| 3 | Teilaufgabe | `1.2.3` |
| unterste Ebene | Arbeitspaket | `1.2.3.1` |

Die Codierung (PSP-Code) macht jedes Element eindeutig referenzierbar, z. B. in Zeiterfassung, Ticketsystem und Kostenstellen.

### Gliederungsarten

| Art | Gliederung nach | Beispiel Teilprojekte | geeignet für |
|---|---|---|---|
| **objektorientiert** | Komponenten / Ergebnissen | Server, Netzwerk, Clients, Drucker | technische Projekte mit klaren Bauteilen |
| **funktionsorientiert** | Tätigkeiten / Abteilungen | Planen, Beschaffen, Installieren, Testen | Projekte mit klaren Fachfunktionen |
| **phasenorientiert** | zeitlicher Abfolge | Analyse, Konzeption, Umsetzung, Abschluss | wiederkehrende Projekttypen |
| **gemischt** | Ebene 2 z. B. Phase, Ebene 3 Objekt | Umsetzung → Server, Clients | häufigste Form in der Praxis |

### Beispiel: Einführung einer neuen Firewall (phasenorientiert, gemischt)

```
1 Firewall-Einführung
├── 1.1 Analyse
│   ├── 1.1.1 Ist-Aufnahme Netzwerk und Regeln
│   └── 1.1.2 Anforderungen erheben (Lastenheft)
├── 1.2 Konzeption
│   ├── 1.2.1 Zonenkonzept (LAN, DMZ, Gäste)
│   ├── 1.2.2 Angebotsvergleich und Auswahl
│   └── 1.2.3 Regelwerk entwerfen
├── 1.3 Umsetzung
│   ├── 1.3.1 Hardware installieren
│   ├── 1.3.2 Regelwerk konfigurieren
│   └── 1.3.3 VPN-Zugänge einrichten
├── 1.4 Test und Inbetriebnahme
│   ├── 1.4.1 Funktions- und Sicherheitstest
│   └── 1.4.2 Umschaltung (Wartungsfenster)
└── 1.5 Abschluss
    ├── 1.5.1 Dokumentation
    ├── 1.5.2 Einweisung der Administratoren
    └── 1.5.3 Abnahme und Projektabschlussbericht
```

### Arbeitspaketbeschreibung

| Feld | Beispiel für 1.3.2 |
|---|---|
| PSP-Code, Name | 1.3.2 Regelwerk konfigurieren |
| Verantwortlich | M. Kurz (Netzwerk) |
| Ziel/Ergebnis | Regelwerk gemäß Konzept 1.2.3 aktiv, exportiert und gesichert |
| Voraussetzungen | 1.3.1 abgeschlossen, Regelwerkentwurf freigegeben |
| Aufwand / Dauer | 24 h / 3 Tage |
| Kosten | 24 h × 50,23 € = 1.205,52 € |
| Abnahmekriterium | Testfälle aus 1.4.1 bestanden |

### Vorgehen beim Erstellen

- **Top-down:** vom Gesamtprojekt schrittweise verfeinern (üblich bei bekannten Projekttypen).
- **Bottom-up:** Tätigkeiten sammeln (Brainstorming) und dann gruppieren (bei neuartigen Projekten).
- Zerlegen, bis ein Arbeitspaket **eindeutig verantwortet, schätzbar und kontrollierbar** ist (Faustregel: wenige Tage bis wenige Wochen).

## Prüfungsfrage

**Frage:** Sie sollen für die Umstellung von 40 Arbeitsplätzen auf neue Notebooks einen Projektstrukturplan erstellen. Erläutern Sie den Zweck eines PSP, nennen Sie zwei Gliederungsarten und geben Sie drei Arbeitspakete an.

**Musterantwort:** Der PSP zerlegt das Projekt vollständig und überschneidungsfrei in überschaubare Teilaufgaben und Arbeitspakete. Er schafft Klarheit über den Leistungsumfang und ist Grundlage für Aufwandsschätzung, Terminplanung (Netzplan, Gantt), Ressourcen- und Kostenplanung sowie die Zuordnung von Verantwortlichkeiten. Gliederungsarten sind die **objektorientierte** Gliederung (z. B. Hardware, Software, Daten) und die **phasenorientierte** Gliederung (z. B. Planung, Beschaffung, Rollout, Abschluss). Arbeitspakete: "Image mit Standardsoftware erstellen und testen", "Benutzerdaten der Altgeräte sichern und migrieren", "Altgeräte zertifiziert löschen und entsorgen".

## Falle

- **PSP mit Reihenfolge oder Terminen:** Der PSP zeigt **was**, nicht **wann**.
- **Arbeitspakete zu groß:** "Rollout" ist kein Arbeitspaket, sondern ein Teilprojekt.
- **Doppelungen:** Dieselbe Tätigkeit in zwei Ästen verletzt die Überschneidungsfreiheit.
- **Projektmanagement vergessen:** Auch Planung, Dokumentation, Abnahme und Projektleitung sind Arbeit und gehören hinein (100-%-Regel).
- **PSP mit Organigramm verwechselt:** Das Organigramm zeigt Personen und Stellen, der PSP Aufgaben.

## Merksatz

**"Erst zerlegen, dann planen."** Der PSP enthält alles, was getan werden muss – und das Arbeitspaket ist die kleinste Einheit mit einem Verantwortlichen.

## Verwandt

- [Projektmanagement-Grundlagen](projektmanagement-grundlagen.md) – Phasen, Ziele, Organisation
- [Aufwandsschätzung](aufwandsschaetzung.md) – Aufwand je Arbeitspaket
- [Netzplan](netzplan.md) – Reihenfolge und kritischer Pfad
- [Gantt-Diagramm](gantt-diagramm.md) – Arbeitspakete auf der Zeitachse
- [Ressourcenplan](ressourcenplan.md)
- [Lastenheft & Pflichtenheft](lastenheft-pflichtenheft.md) – Umfang, aus dem der PSP abgeleitet wird
