# Klassendiagramm

> Statisches UML-Diagramm zur Darstellung von Klassen, deren Eigenschaften, Methoden und Beziehungen untereinander.

## In 30 Sekunden

- Das **Klassendiagramm** ist das wichtigste **statische** UML-Diagramm: Es zeigt **Klassen** mit **Attributen** und **Methoden** sowie deren **Beziehungen**.
- Klasse = Rechteck mit drei Bereichen: Name, Attribute, Methoden. Sichtbarkeit: `+` public, `-` private, `#` protected, `~` package.
- Beziehungen: **Assoziation** (Linie), **Aggregation** (leere Raute am Ganzen), **Komposition** (gefüllte Raute am Ganzen), **Vererbung** (durchgezogene Linie mit leerem Dreieck zur Oberklasse), **Abhängigkeit** (gestrichelter Pfeil).
- **Multiplizitäten** wie `1`, `0..1`, `0..*`, `1..*` an den Linienenden.
- Geprüft: **Aggregation vs. Komposition** (2023S, Gebäude – Mieter/Raum).

## Kernwissen

### Zweck

Das **Klassendiagramm** zeigt den statischen Aufbau eines objektorientierten Systems. Es bildet Klassen, Attribute, Operationen (Methoden) und die Beziehungen zwischen ihnen ab.

### Aufbau einer Klasse

Eine Klasse wird als Rechteck mit **drei Bereichen** dargestellt:

```
┌─────────────────────────┐
│      Klassenname        │  ← Oben: Name der Klasse
├─────────────────────────┤
│  - Attribut1: Typ       │  ← Mitte: Attribute (Eigenschaften)
│  + Attribut2: Typ       │
├─────────────────────────┤
│  + Methode1(): Rückgabe │  ← Unten: Operationen/Methoden
│  - Methode2(): void     │
└─────────────────────────┘
```

### Sichtbarkeitsmodifikatoren

| Symbol | Sichtbarkeit | Bedeutung |
|---|---|---|
| `+` | public | Von überall zugreifbar |
| `-` | private | Nur innerhalb der Klasse zugreifbar |
| `#` | protected | Innerhalb der Klasse und abgeleiteter Klassen |
| `~` | package (default) | Innerhalb des gleichen Pakets |

### Beziehungen im Klassendiagramm

#### 1. Assoziation (allgemeine Beziehung)
Die einfachste Beziehung zwischen Klassen. Eine durchgezogene Linie.

```
Kunde ──── Rechnung
```

Bedeutung: Ein Kunde kann Rechnungen haben und umgekehrt.

#### 2. Aggregation (lose Beziehung)
"hat ein" - Teil kann auch ohne das Ganze existieren. Dargestellt mit **leerer Raute** am Ganzen.

```
Abteilung ◇──── Mitarbeiter
```

**Beispiel:** Mitarbeiter kann auch ohne Abteilung existieren (kann wechseln/entlassen werden).

#### 3. Komposition (starke Beziehung)
"besteht aus" - Teil kann **nicht** ohne das Ganze existieren. Dargestellt mit **gefüllter Raute** am Ganzen.

```
Gebäude ◆──── Raum
```

**Beispiel:** Ein Raum existiert nicht ohne das Gebäude. Wird das Gebäude zerstört, geht auch der Raum verloren. *(Wurde 2023S abgefragt!)*

#### 4. Vererbung / Generalisierung
"ist ein" - Über-/Unterklasse. Dargestellt mit durchgezogener Linie und **leerer Dreiecksspitze** zur Oberklasse.

```
                ┌─────────┐
                │ Fahrzeug│
                └────┬────┘
                     △
                    / \
               ┌───┴───┐
             PKW     LKW
```

**Beispiel:** PKW *ist ein* Fahrzeug, LKW *ist ein* Fahrzeug.

#### 5. Abhängigkeit (Dependency)
Temporäre Nutzung einer Klasse. Gestrichelte Linie mit offener Pfeilspitze.

```
Bestellung ┈┈> Lager
```

### Multiplizität (Kardinalität)

Gibt an, wie viele Objekte beteiligt sind.

| Angabe | Bedeutung |
|---|---|
| `1` | Genau eins |
| `0..1` | Null oder eins |
| `*` oder `0..*` | Beliebig viele (0 oder mehr) |
| `1..*` | Mindestens eins |
| `n..m` | Zwischen n und m |

**Beispiel:**
```
Kunde "1" ──── "0..*" Rechnung
```
Ein Kunde kann 0 oder mehr Rechnungen haben, jede Rechnung gehört zu genau einem Kunden.

### Prüfungsrelevante Unterschiede

| Beziehung | Existenzabhängig? | Symbol |
|---|---|---|
| **Assoziation** | Egal | Durchgezogene Linie |
| **Aggregation** | Teil kann ohne Ganzes existieren | ◇ (leere Raute) |
| **Komposition** | Teil kann **NICHT** ohne Ganzes existieren | ◆ (gefüllte Raute) |
| **Vererbung** | "ist ein" | △ (leere Dreiecksspitze) |

## Prüfungsfrage

> Ordnen Sie zu: Gebäude – Räume, Mitarbeiter – Abteilung. Begründen Sie.
 
**Musterantwort:**
- **Gebäude – Räume:** Komposition (◆), da Räume ohne Gebäude nicht existieren können
- **Mitarbeiter – Abteilung:** Aggregation (◇), da Mitarbeiter auch außerhalb der Abteilung existieren kann (z.B. Kündigung, Versetzung)

## Falle

- **Raute auf der falschen Seite:** Sie sitzt am **Ganzen**, nicht am Teil.
- **Leere und gefüllte Raute vertauscht:** leer = Aggregation (Teil lebt weiter), gefüllt = Komposition (Teil stirbt mit).
- **Vererbungspfeil gestrichelt gezeichnet:** Die Generalisierung ist eine **durchgezogene** Linie; gestrichelt mit leerem Dreieck ist die Realisierung eines Interfaces.
- **Multiplizität verkehrt herum:** Die Zahl am Ende einer Linie gibt an, wie viele Objekte **dieser** Seite zu einem Objekt der anderen Seite gehören.

## Merksatz

- **Aggregation vs. Komposition:** Stell dir vor, das Gebäude wird abgerissen. Bleiben die Räume übrig? Nur bei Aggregation (Mieter) ja, bei Komposition (Räume) nein.
- **Vererbung vs. Assoziation:** "ist ein" = Vererbung. "hat ein" = Assoziation/Aggregation/Komposition

## Verwandt

- [Aktivitätsdiagramm](Aktivitaetsdiagramm.md)
- [Anwendungsfalldiagramm-Use-Case-Diagramm](Anwendungsfalldiagramm-Use-Case-Diagramm.md)
- [Sequenzdiagramm](Sequenzdiagramm.md)
- [Zustandsdiagramm](zustandsdiagramm.md)
- [UML-Übersicht](readme.md)
- [Lektion 14: Datenmodellierung, SQL & UML](../../analyse-der-exams/L14_Datenmodellierung_SQL_UML.md)
- [ER-Modell & Kardinalitäten](../sql/er-modell-kardinalitaeten.md) – Multiplizitäten im Datenmodell
- [OOP-Grundlagen](../Pseudocode/oop-grundlagen.md) – Kapselung und Vererbung im Code
