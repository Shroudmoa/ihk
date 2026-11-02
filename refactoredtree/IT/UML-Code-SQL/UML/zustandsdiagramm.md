# Zustandsdiagramm (UML State Machine)

## In 30 Sekunden

- Das **Zustandsdiagramm** (Zustandsautomat, State Machine) zeigt, **welche Zustände ein Objekt annehmen kann** und **durch welche Ereignisse es von einem Zustand in den nächsten wechselt**.
- Es gehört zu den **dynamischen** UML-Diagrammen (Verhaltensdiagramme), zusammen mit Aktivitäts- und Sequenzdiagramm. Diese Zuordnung wurde 2021W abgefragt.
- Bausteine: **Startzustand** (gefüllter Kreis), **Zustand** (abgerundetes Rechteck), **Transition** (Pfeil) mit `Ereignis [Bedingung] / Aktion`, **Endzustand** (Kreis mit Punkt).
- Typische IT-Beispiele: Ticket-Status, Prozesszustände im Betriebssystem, TCP-Verbindung, Druckauftrag, Bestellung.

## Kernwissen

### Notation

| Element | Symbol | Bedeutung |
|---|---|---|
| Startzustand | ● | Beginn, genau einer pro Automat |
| Zustand | ( Name ) abgerundetes Rechteck | Situation, in der das Objekt verweilt |
| Transition | ──► | Zustandswechsel |
| Beschriftung | `ereignis [wächter] / aktion` | Auslöser, Bedingung, ausgeführte Aktion |
| Endzustand | ◉ | Objekt hat seinen Lebenszyklus beendet |
| Interne Aktionen | `entry /`, `do /`, `exit /` | beim Betreten, während, beim Verlassen des Zustands |
| Entscheidung | ◇ | Verzweigung nach Wächterbedingungen |
| Zusammengesetzter Zustand | Zustand mit Unterzuständen | Verschachtelung |

**Wächterbedingung (Guard):** steht in eckigen Klammern. Die Transition feuert nur, wenn das Ereignis eintritt **und** die Bedingung wahr ist.

### Beispiel: Ticket im Service Desk

```
 ●
 │ ticket erstellt
 ▼
(Neu) ──zugewiesen──► (In Bearbeitung) ──Rückfrage──► (Wartet auf Kunde)
                          │    ▲                            │
                          │    └────── Antwort erhalten ────┘
                          │ gelöst / Kunde informieren
                          ▼
                      (Gelöst) ──[Kunde widerspricht]──► (In Bearbeitung)
                          │
                          │ 5 Tage ohne Rückmeldung / Ticket schließen
                          ▼
                     (Geschlossen) ──► ◉
```

### Beispiel: Prozesszustände im Betriebssystem

```
● ─► (Neu) ─zugelassen─► (Bereit) ─Scheduler wählt aus─► (Laufend) ─beendet─► ◉
                           ▲   ▲                           │   │
                           │   └──── Zeitscheibe abgelaufen ┘   │
                           │                                    │ wartet auf E/A
                           └──── E/A abgeschlossen ──── (Blockiert) ◄┘
```

### Abgrenzung

| Diagramm | Frage, die es beantwortet |
|---|---|
| **Zustandsdiagramm** | In welchen Zuständen kann **ein Objekt** sein und wodurch wechselt es? |
| **Aktivitätsdiagramm** | Welche **Schritte** laufen in welcher Reihenfolge ab, wer macht was? |
| **Sequenzdiagramm** | Welche **Nachrichten** tauschen mehrere Objekte in zeitlicher Reihenfolge aus? |

### Umsetzung im Code

Ein Zustandsautomat lässt sich als `enum` plus `switch` umsetzen:

```csharp
enum TicketStatus { Neu, InBearbeitung, WartetAufKunde, Geloest, Geschlossen }

TicketStatus status = TicketStatus.Neu;

switch (status) {
    case TicketStatus.Neu:
        if (zugewiesen) status = TicketStatus.InBearbeitung;
        break;
    case TicketStatus.Geloest:
        if (tageOhneRueckmeldung >= 5) status = TicketStatus.Geschlossen;
        break;
}
```

## Prüfungsfrage

**Frage:** Ein Druckauftrag kann die Zustände "wartend", "druckt", "angehalten", "fehlerhaft" und "abgeschlossen" annehmen. Beschreiben Sie ein UML-Zustandsdiagramm mit sinnvollen Ereignissen. Ordnen Sie das Diagramm der statischen oder dynamischen Sicht zu.

**Musterantwort:** Startzustand → **wartend**. Ereignis "Drucker frei" → **druckt**. Aus "druckt": Ereignis "Benutzer pausiert" → **angehalten**, Ereignis "fortsetzen" zurück → **druckt**; Ereignis "Papierstau" bzw. "Toner leer" → **fehlerhaft**, Ereignis "Störung behoben" → **druckt**; Ereignis "letzte Seite gedruckt" → **abgeschlossen** → Endzustand. Zusätzlich kann aus "wartend" und "angehalten" das Ereignis "abbrechen" direkt zum Endzustand führen. Das Zustandsdiagramm gehört zur **dynamischen Sicht** (Verhaltensdiagramm), weil es das Verhalten eines Objekts über die Zeit beschreibt.

## Falle

- **Zustand mit Aktivität verwechselt:** "Druckt" ist ein Zustand, "Seite drucken" eine Aktivität. Zustände sind Situationen, keine Arbeitsschritte.
- **Transition ohne Ereignis:** Jeder Pfeil braucht einen Auslöser (Ausnahme: automatischer Übergang nach Abschluss der `do`-Aktivität).
- **Startzustand vergessen** oder mehrere Startzustände gezeichnet.
- **Als statisches Diagramm eingeordnet:** Zustandsdiagramme sind dynamisch.
- **Unerreichbare Zustände:** Jeder Zustand muss vom Start aus erreichbar sein.

## Merksatz

**Zustand = "Wo bin ich?", Transition = "Was bringt mich weiter?"** Beschriftung immer als `Ereignis [Bedingung] / Aktion`.

## Verwandt

- [UML-Übersicht](readme.md)
- [Aktivitätsdiagramm](Aktivitaetsdiagramm.md) – Abläufe statt Zustände
- [Sequenzdiagramm](Sequenzdiagramm.md) – Nachrichten zwischen Objekten
- [Klassendiagramm](Klassendiagramm.md) – die Klasse, deren Zustände modelliert werden
- [Ticketsystem, SLA & Eskalation](../../Management/ticketsystem-sla-eskalation.md) – Ticket-Status in der Praxis
- [Lektion 4: Programmierung](../../analyse-der-exams/04_Programmierung_Grundlagen.md) – statische vs. dynamische Sicht (2021W)
