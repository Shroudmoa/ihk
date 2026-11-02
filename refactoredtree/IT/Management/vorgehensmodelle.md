# Vorgehensmodelle: Wasserfall, V-Modell, Scrum, Kanban

## In 30 Sekunden

- **Vorgehensmodelle** legen fest, in welchen Schritten ein (IT-)Projekt abläuft.
- **Klassisch / plangetrieben:** **Wasserfall** (Phasen nacheinander) und **V-Modell** (jeder Entwicklungsphase steht eine Teststufe gegenüber). Gut bei stabilen, klaren Anforderungen.
- **Agil:** **Scrum** (feste Sprints, Rollen, Events, Artefakte) und **Kanban** (kontinuierlicher Fluss, WIP-Limits). Gut bei unklaren oder sich ändernden Anforderungen.
- **Iterativ-inkrementell:** Ergebnis wächst in Schritten; Feedback fließt in den nächsten Schritt (auch Spiralmodell, Prototyping).
- **Hybrid:** klassischer Rahmen (Meilensteine, Budget) + agile Umsetzung im Team.

## Kernwissen

### Wasserfallmodell

```
Analyse → Entwurf → Implementierung → Test → Einführung → Betrieb/Wartung
```

- Jede Phase endet mit einem Dokument und wird abgeschlossen, bevor die nächste beginnt.
- Rücksprünge nur zur direkt vorherigen Phase (erweitertes Modell).
- **Vorteile:** klare Struktur, gut planbar, Kosten und Termine früh bekannt, gute Dokumentation.
- **Nachteile:** späte Fehlererkennung, Änderungen teuer, Kunde sieht das Ergebnis erst am Ende.

### V-Modell

```
Anforderungsdefinition ─────────────────────── Abnahmetest
    Systementwurf ──────────────────────── Systemtest
        Architekturentwurf ─────────── Integrationstest
            Feinentwurf ───────── Komponententest (Unit)
                     Implementierung
```

- Linker Ast: Spezifikation, rechter Ast: Test. Jede Ebene wird gegen die gegenüberliegende Spezifikation geprüft (**Verifikation** = richtig gebaut; **Validierung** = das Richtige gebaut).
- Für Behörden: **V-Modell XT** (verbindlich in Bundesprojekten).
- **Vorteil:** Qualitätssicherung ist von Anfang an eingeplant. **Nachteil:** ebenso unflexibel wie der Wasserfall.

### Scrum

| Bestandteil | Inhalt |
|---|---|
| **Rollen (Accountabilities)** | **Product Owner** (verantwortet Produktwert, pflegt und priorisiert das Product Backlog), **Scrum Master** (Coach, beseitigt Hindernisse, sorgt für Einhaltung von Scrum), **Developers** (setzen um, organisieren sich selbst, typischerweise 10 Personen oder weniger im Scrum Team) |
| **Events** | **Sprint** (1–4 Wochen, feste Länge), **Sprint Planning**, **Daily Scrum** (15 Minuten täglich), **Sprint Review** (Ergebnis mit Stakeholdern prüfen), **Sprint Retrospective** (Zusammenarbeit verbessern) |
| **Artefakte** | **Product Backlog** (priorisierte Anforderungen, z. B. User Stories), **Sprint Backlog** (für den Sprint ausgewählte Einträge + Plan), **Increment** (nutzbares Ergebnis, erfüllt die Definition of Done) |

- **User Story:** "Als *Rolle* möchte ich *Funktion*, damit *Nutzen*." Beispiel: "Als Mitarbeiter möchte ich mein Passwort selbst zurücksetzen, damit ich nicht auf den Service Desk warten muss."
- Schätzung in **Story Points** (Planning Poker), Fortschritt im **Burndown-Chart**, **Velocity** = erledigte Story Points pro Sprint.

### Kanban

- Visualisierung auf einem **Kanban-Board** mit Spalten, z. B. *Backlog | To Do | In Arbeit | Test | Erledigt*.
- **WIP-Limit** (Work in Progress): maximale Anzahl Karten pro Spalte → Engpässe werden sichtbar, Fokus auf Fertigstellen.
- **Pull-Prinzip:** Neue Arbeit wird gezogen, wenn Kapazität frei ist.
- Keine festen Rollen oder Sprints; Kennzahlen **Durchlaufzeit (Lead Time)** und **Durchsatz**.
- Sehr gut geeignet für **IT-Betrieb und Service Desk** mit ständig neu eintreffenden Aufgaben.

### Vergleich

| Kriterium | Wasserfall / V-Modell | Scrum | Kanban |
|---|---|---|---|
| Anforderungen | vollständig zu Beginn | entwickeln sich, Product Backlog | laufend, Board |
| Planung | einmal, detailliert | pro Sprint | kontinuierlich |
| Lieferung | am Ende | nach jedem Sprint (Increment) | fortlaufend |
| Änderungen | teuer, Change Request | zwischen Sprints einfach | jederzeit |
| Rollen | Projektleitung, Team | PO, Scrum Master, Developers | keine vorgeschrieben |
| Kundeneinbindung | Anfang und Abnahme | jedes Sprint Review | laufend |
| geeignet für | Infrastrukturprojekte mit fester Vorgabe, regulierte Umgebungen | Softwareentwicklung, neue Produkte | Betrieb, Support, Wartung |

### Agiles Manifest (2001) – vier Werte

Individuen und Interaktionen **über** Prozesse und Werkzeuge · funktionierende Software **über** umfassende Dokumentation · Zusammenarbeit mit dem Kunden **über** Vertragsverhandlung · Reagieren auf Veränderung **über** das Befolgen eines Plans. (Die rechten Punkte sind nicht wertlos, die linken werden höher gewichtet.)

## Prüfungsfrage

**Frage:** Ihr Unternehmen soll (a) ein Rechenzentrum mit exakt vorgegebener Hardware nach Ausschreibung aufbauen und (b) für einen Kunden eine Self-Service-App entwickeln, deren Funktionen sich erst im Laufe des Projekts klären. Empfehlen Sie jeweils ein Vorgehensmodell und begründen Sie.

**Musterantwort:** Für (a) eignet sich ein **klassisches Modell wie Wasserfall oder V-Modell**: Die Anforderungen stehen durch die Ausschreibung fest, Abhängigkeiten (Strom, Klima, Rack, Verkabelung) erfordern eine feste Reihenfolge, Budget und Termine müssen verbindlich geplant werden, und das V-Modell sichert über Teststufen die Abnahme gegen die Spezifikation. Für (b) eignet sich **Scrum**: Die Anforderungen sind unklar und ändern sich, durch kurze Sprints entsteht regelmäßig ein lauffähiges Increment, das im Sprint Review mit dem Kunden geprüft wird; neue Erkenntnisse fließen über das priorisierte Product Backlog in den nächsten Sprint ein.

## Falle

- **Scrum Master = Projektleiter:** Er ist Coach und Hindernisbeseitiger, kein Weisungsgeber.
- **Product Owner = Kunde:** Der PO vertritt die Kundensicht im Team, ist aber eine eigene Rolle mit Entscheidungsbefugnis.
- **Agil = ohne Plan und Dokumentation:** Auch agile Teams planen (pro Sprint) und dokumentieren, was nötig ist.
- **Kanban mit Sprints:** Kanban kennt keine festen Iterationen.
- **V-Modell mit Wasserfall gleichsetzen:** Das V-Modell ordnet jeder Entwurfsphase eine Teststufe zu.

## Merksatz

**"Feste Anforderungen → Wasserfall/V-Modell, unklare Anforderungen → Scrum, ständiger Strom an Aufgaben → Kanban."**

## Verwandt

- [Projektmanagement-Grundlagen](projektmanagement-grundlagen.md)
- [Lastenheft & Pflichtenheft](lastenheft-pflichtenheft.md) – klassische Anforderungsdokumente
- [Aufwandsschätzung](aufwandsschaetzung.md) – Planning Poker, Story Points
- [Softwaretest: White-Box & Black-Box](../UML-Code-SQL/Pseudocode/softwaretest-white-black-box.md) – Teststufen im V-Modell
- [Qualitätsmanagement](qualitaetsmanagement.md) – PDCA als Verbesserungszyklus
- [Ticketsystem, SLA & Eskalation](ticketsystem-sla-eskalation.md) – Kanban im Support
- [Blue-Green-Deployment](../FiSi-Administrator/Server/blue-green-deployment.md) – häufige Auslieferung ohne Ausfall
