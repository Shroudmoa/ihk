# Anwendungsfalldiagramm / Use-Case-Diagramm

> Statisches UML-Diagramm zur Darstellung von funktionalen Anforderungen aus Sicht des Anwenders

## Zweck

Das **Use-Case-Diagramm** zeigt, welche Funktionen (Use Cases) ein System für welche Benutzer (Akteure) bietet. Es wird vor allem in der **Anforderungsanalyse** eingesetzt und dient als gemeinsame Basis zwischen Fachbereich, Entwicklung und Test.

## Blickwinkel

- **Außenperspektive:** Betrachtet das System von außen, nicht den internen Aufbau
- **Benutzerorientiert:** Was kann der Nutzer mit dem System tun?
- **Abgrenzung:** Zeigt klar, was zum System gehört und was nicht

## Elemente

| Element | Darstellung | Beschreibung |
|---|---|---|
| **Akteur (Actor)** | Strichmännchen | Eine Rolle außerhalb des Systems. Kann Mensch oder externes System sein. |
| **Use Case** | Ellipse mit Namen | Eine abgeschlossene Funktion des Systems (z.B. "Urlaub beantragen") |
| **Systemgrenze** | Rechteck | Trennt System von seiner Umgebung. Alles innerhalb gehört zum Projekt/Scope. |
| **Assoziation** | Durchgezogene Linie | Verbindet Akteur mit Use Case (Nutzung) |

## Beziehungen zwischen Use Cases

### 1. `<<include>>` - "wird immer ausgeführt"
Der Basis-Use-Case **ruft** den inkludierten Use-Case **unbedingt** auf.

**Darstellung:** Gestrichelte Linie mit Pfeil zum inkludierten Use Case, beschriftet mit `<<include>>`

**Beispiel:** "Buchung abschließen" *include* "Zahlung durchführen"
- Ohne Zahlung kann keine Buchung abgeschlossen werden

### 2. `<<extend>>` - "wird optional ausgeführt"
Der erweiternde Use-Case wird **nur unter bestimmten Bedingungen** ausgeführt.

**Darstellung:** Gestrichelte Linie mit Pfeil zum Basis-Use-Case, beschriftet mit `<<extend>>`

**Beispiel:** "Kunde einloggen" *extend* "Passwort vergessen"
- Nur wenn der Nutzer "Passwort vergessen" klickt, wird dieser Use Case ausgeführt

### 3. Generalisierung
Akteure oder Use Cases können vererbt werden.

**Beispiel:** "Mitarbeiter" kann Generalisierung von "Vorgesetzter" sein (Vorgesetzter hat alle Rechte des Mitarbeiters plus zusätzliche).

## Beispiel: Online-Shop

```
┌─────────────────────────────────────────┐
│           Online-Shop System             │
│                                         │
│  ┌─────────────┐    ┌─────────────┐   │
│  │ Artikel      │    │ Bestellung   │   │
│  │ durchsuchen  │    │ aufgeben     │   │
│  └─────────────┘    └──────┬──────┘   │
│                           <<include>> │
│                          ┌───▼───┐     │
│                          │Zahlung│     │
│                          │durchf.│     │
│                          └───────┘     │
└─────────────────────────────────────────┘
           △                        △
           │                        │
        ┌──┴──┐                  ┌──┴──┐
       Kunde Gast               Kunde
```

## Unterschied include vs. extend

| Kriterium | `<<include>>` | `<<extend>>` |
|---|---|---|
| **Ausführung** | Immer, wenn Basis-Use-Case ausgeführt wird | Nur unter Bedingung/Zusatzfall |
| **Richtung** | Basis → inkludierter Use Case | Erweiternder Use Case → Basis-Use Case |
| **Frage** | "Wird das IMMER benötigt?" | "Ist das nur in Sonderfällen?" |
| **Beispiel** | Anmeldung *include* Authentifizierung | Einkaufen *extend* Gutschein einlösen |

## Einsatz in der Prüfung

- **Anforderungsanalyse:** Funktionen für Benutzer erfassen
- **Systemabgrenzung:** Klären, was im Scope liegt
- **Kommunikation:** Zwischen Fachbereich und IT-Team
- **Testfallbasis:** Jeder Use Case kann Grundlage für Testfälle sein

## Prüfungsaufgabe (Muster)

> Zeichnen/Stichwortartig beschreiben Sie ein Use-Case-Diagramm für einen Bibliotheksausleihvorgang. Akteure: Leser, Bibliothekar.

**Lösungsideen:**
- **Akteur Leser:** "Medium suchen", "Medium ausleihen", "Medium zurückgeben"
- **Akteur Bibliothekar:** "Medium erfassen", "Benutzer verwalten", "Ausleihe verwalten"
- **include:** "Medium ausleihen" *include* "Benutzerauthentifizierung"
- **extend:** "Medium ausleihen" *extend* "Gebühr erheben" (bei verspäteter Vormerkung etc.)

## Merksatz

**"Use Case = Was, nicht Wie"** - Beschreibt die Funktionalität, nicht die technische Umsetzung.

## Weiterführend
- [Klassendiagramm](Klassendiagram.md)
- [Aktivitätsdiagramm](Aktivitätsdiagramm.md)
- [Sequenzdiagramm](Sequenzdiagramm)
