# Klassendiagramm

> Statisches UML-Diagramm zur Darstellung von Klassen, deren Eigenschaften, Methoden und Beziehungen untereinander.

## Zweck

Das **Klassendiagramm** zeigt den statischen Aufbau eines objektorientierten Systems. Es bildet Klassen, Attribute, Operationen (Methoden) und die Beziehungen zwischen ihnen ab.

## Aufbau einer Klasse

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

## Sichtbarkeitsmodifikatoren

| Symbol | Sichtbarkeit | Bedeutung |
|---|---|---|
| `+` | public | Von überall zugreifbar |
| `-` | private | Nur innerhalb der Klasse zugreifbar |
| `#` | protected | Innerhalb der Klasse und abgeleiteter Klassen |
| `~` | package (default) | Innerhalb des gleichen Pakets |

## Beziehungen im Klassendiagramm

### 1. Assoziation (allgemeine Beziehung)
Die einfachste Beziehung zwischen Klassen. Eine durchgezogene Linie.

```
Kunde ──── Rechnung
```

Bedeutung: Ein Kunde kann Rechnungen haben und umgekehrt.

### 2. Aggregation (lose Beziehung)
"hat ein" - Teil kann auch ohne das Ganze existieren. Dargestellt mit **leerer Raute** am Ganzen.

```
Abteilung ◇──── Mitarbeiter
```

**Beispiel:** Mitarbeiter kann auch ohne Abteilung existieren (kann wechseln/entlassen werden).

### 3. Komposition (starke Beziehung)
"besteht aus" - Teil kann **nicht** ohne das Ganze existieren. Dargestellt mit **gefüllter Raute** am Ganzen.

```
Gebäude ◆──── Raum
```

**Beispiel:** Ein Raum existiert nicht ohne das Gebäude. Wird das Gebäude zerstört, geht auch der Raum verloren. *(Wurde 2023S abgefragt!)*

### 4. Vererbung / Generalisierung
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

### 5. Abhängigkeit (Dependency)
Temporäre Nutzung einer Klasse. Gestrichelte Linie mit offener Pfeilspitze.

```
Bestellung ┈┈> Lager
```

## Multiplizität (Kardinalität)

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

## Prüfungsrelevante Unterschiede

| Beziehung | Existenzabhängig? | Symbol |
|---|---|---|
| **Assoziation** | Egal | Durchgezogene Linie |
| **Aggregation** | Teil kann ohne Ganzes existieren | ◇ (leere Raute) |
| **Komposition** | Teil kann **NICHT** ohne Ganzes existieren | ◆ (gefüllte Raute) |
| **Vererbung** | "ist ein" | △ (leere Dreiecksspitze) |

## Merksätze

- **Aggregation vs. Komposition:** Stell dir vor, das Gebäude wird abgerissen. Bleiben die Räume übrig? Nur bei Aggregation (Mieter) ja, bei Komposition (Räume) nein.
- **Vererbung vs. Assoziation:** "ist ein" = Vererbung. "hat ein" = Assoziation/Aggregation/Komposition

## Prüfungsaufgabe (Muster)

> Ordnen Sie zu: Gebäude – Räume, Mitarbeiter – Abteilung. Begründen Sie.
 
**Lösung:**
- **Gebäude – Räume:** Komposition (◆), da Räume ohne Gebäude nicht existieren können
- **Mitarbeiter – Abteilung:** Aggregation (◇), da Mitarbeiter auch außerhalb der Abteilung existieren kann (z.B. Kündigung, Versetzung)

## Weiterführend
- [Aktivitätsdiagramm](Aktivitätsdiagramm.md)
- [Anwendungsfalldiagramm-Use-Case-Diagramm](Anwendungsfalldiagramm-Use-Case-Diagramm.md)
- [Sequenzdiagramm](Sequenzdiagramm)
