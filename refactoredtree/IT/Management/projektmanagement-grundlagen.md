# Projektmanagement-Grundlagen

## In 30 Sekunden

- Ein **Projekt** ist ein **einmaliges, zeitlich begrenztes Vorhaben** mit einem definierten Ziel, begrenzten Ressourcen und eigener Organisation (DIN 69901).
- **Magisches Dreieck:** **Leistung/Qualität, Zeit, Kosten** – ändert man eine Ecke, ändern sich die anderen.
- Ziele werden **SMART** formuliert: spezifisch, messbar, attraktiv/akzeptiert, realistisch, terminiert.
- **Phasen:** Initialisierung → Definition → Planung → Durchführung/Steuerung → Abschluss.
- **Meilenstein** = Ereignis mit besonderer Bedeutung (z. B. Abnahme), **Dauer 0**, Prüfpunkt für "weiter oder stopp".
- Organisationsformen: **Einfluss-** (Stabs-), **Matrix-** und **reine Projektorganisation**.
- Prüfungsbezug: Die betriebliche Projektarbeit der AP2 folgt genau diesem Ablauf (Antrag, Planung, Durchführung, Dokumentation, Fachgespräch).

## Kernwissen

### Merkmale eines Projekts

| Merkmal | Beispiel "Einführung eines Ticketsystems" |
|---|---|
| Einmaligkeit | dieses System, in diesem Unternehmen, mit diesen Anforderungen |
| zeitliche Begrenzung | Start 01.03., Go-Live 30.06. |
| klares Ziel | alle Supportanfragen werden ab Juli im Ticketsystem erfasst |
| begrenzte Ressourcen | 25.000 € Budget, 2 Administratoren zu je 50 % |
| Komplexität, Risiko | Schnittstellen zu AD und Mailserver, Akzeptanz der Nutzer |
| eigene Organisation | Projektleitung, Projektteam, Lenkungsausschuss |

Keine Projekte sind Routinetätigkeiten wie das tägliche Backup oder das monatliche Patchen.

### Magisches Dreieck (Teufelsquadrat)

```
            Leistung / Qualität
                   /\
                  /  \
                 /    \
           Zeit /______\ Kosten
```

- Mehr Leistung bei gleicher Zeit → höhere Kosten.
- Kürzere Zeit bei gleichen Kosten → weniger Leistung oder Qualität.
- Das **Teufelsquadrat** (nach Sneed) ergänzt die **Quantität** und stellt die Qualität in die Mitte.

### SMART-Ziele

| Buchstabe | Bedeutung | Beispiel |
|---|---|---|
| S | spezifisch | "Alle 120 Clients werden auf Windows 11 migriert" |
| M | messbar | "… mit höchstens 2 % Fehlerquote beim Rollout" |
| A | attraktiv / akzeptiert | von Geschäftsführung und Fachabteilungen freigegeben |
| R | realistisch | mit 2 Technikern und Softwareverteilung machbar |
| T | terminiert | "… bis 31.10." |

Unterscheide **Muss-, Soll- und Kann-Ziele** sowie **Nicht-Ziele** (was ausdrücklich nicht zum Projekt gehört).

### Projektphasen

| Phase | Ergebnisse / Dokumente |
|---|---|
| **Initialisierung** | Projektidee, Machbarkeitsstudie, Projektauftrag |
| **Definition** | Ziele, Lastenheft, Stakeholderanalyse, Kick-off |
| **Planung** | Pflichtenheft, PSP, Aufwandsschätzung, Netzplan, Gantt, Ressourcen-, Kosten-, Risikoplan |
| **Durchführung und Steuerung** | Umsetzung, Statusberichte, Soll-Ist-Vergleich, Change Requests |
| **Abschluss** | Abnahmeprotokoll, Übergabe, Dokumentation, Lessons Learned, Abschlussbericht |

### Projektauftrag (Project Charter)

Enthält: Ausgangssituation, Ziele, Nicht-Ziele, Budget, Termine/Meilensteine, Projektleitung und Team, Auftraggeber, Risiken, Unterschrift. Mit ihm erhält die Projektleitung ihre Befugnisse.

### Rollen

| Rolle | Aufgabe |
|---|---|
| **Auftraggeber** | beauftragt, finanziert, nimmt ab |
| **Lenkungsausschuss** | entscheidet bei Meilensteinen und Eskalationen |
| **Projektleitung** | plant, steuert, berichtet, verantwortet Ziele im magischen Dreieck |
| **Projektteam** | bearbeitet Arbeitspakete |
| **Stakeholder** | alle Betroffenen und Beteiligten |

### Projektorganisationsformen

| Form | Beschreibung | Vorteil | Nachteil |
|---|---|---|---|
| **Einfluss-/Stabsprojektorganisation** | Projektleitung koordiniert nur, Mitarbeitende bleiben in der Linie | einfach, keine Umorganisation | wenig Weisungsbefugnis, langsame Entscheidungen |
| **Matrixorganisation** | Mitarbeitende haben fachliche (Projekt) und disziplinarische (Linie) Vorgesetzte | flexible Ressourcennutzung | Konflikte zwischen Projekt- und Linienleitung |
| **reine Projektorganisation** | eigenes Team, volle Weisungsbefugnis | schnelle Entscheidungen, hohe Identifikation | Ressourcen gebunden, Rückkehr in die Linie unklar |

### Kick-off-Meeting

Offizieller Projektstart: Ziele und Rahmenbedingungen vorstellen, Rollen klären, Kommunikationsregeln (Jour fixe, Statusbericht) festlegen, Motivation schaffen.

## Prüfungsfrage

**Frage:** Ihr Ausbildungsbetrieb plant die Umstellung der Telefonanlage auf VoIP. Begründen Sie, warum es sich um ein Projekt handelt, formulieren Sie ein SMART-Ziel und erläutern Sie am magischen Dreieck, was passiert, wenn der Go-Live-Termin um vier Wochen vorgezogen wird.

**Musterantwort:** Die Umstellung ist **einmalig** (eine Ablösung der bestehenden Anlage), **zeitlich begrenzt** (Start und Go-Live), hat ein **klares Ziel**, ein begrenztes Budget und Personal und erfordert eine eigene Organisation mit Projektleitung. **SMART-Ziel:** "Bis zum 30.09. sind alle 85 Nebenstellen am Standort Augsburg auf VoIP umgestellt, die Sprachqualität erreicht einen MOS-Wert von mindestens 4,0, das Budget von 18.000 € wird eingehalten." **Magisches Dreieck:** Wird die Zeit verkürzt, muss entweder mehr Personal oder ein Dienstleister eingesetzt werden (höhere **Kosten**) oder der **Leistungsumfang** sinkt (z. B. Umstellung der Faxgeräte in eine zweite Phase verschieben) bzw. die Qualität leidet (weniger Tests).

## Falle

- **Routineaufgabe als Projekt:** Wiederkehrende Tätigkeiten sind Linienaufgaben.
- **Ziele ohne Messgröße:** "Das Netzwerk soll schneller werden" ist nicht SMART.
- **Magisches Dreieck mit vier Ecken nennen,** wenn nach dem Dreieck gefragt ist (das Teufelsquadrat ist eine Erweiterung).
- **Meilenstein mit Dauer:** Ein Meilenstein ist ein Zeitpunkt.
- **Auftraggeber = Projektleitung:** Der Auftraggeber beauftragt und nimmt ab, die Projektleitung führt durch.

## Merksatz

**"Einmalig, befristet, zielgerichtet."** Und im magischen Dreieck gilt: Wer an einer Ecke zieht, verschiebt die beiden anderen.

## Verwandt

- [Vorgehensmodelle](vorgehensmodelle.md) – Wasserfall, V-Modell, Scrum, Kanban
- [Lastenheft & Pflichtenheft](lastenheft-pflichtenheft.md)
- [Stakeholder- & Risikoanalyse](stakeholder-risikoanalyse.md)
- [Projektstrukturplan (PSP)](projektstrukturplan-psp.md)
- [Netzplan](netzplan.md) · [Gantt-Diagramm](gantt-diagramm.md) · [Ressourcenplan](ressourcenplan.md)
- [Soll-Ist-Vergleich](soll-ist-vergleich.md) – Steuerung
- [Abnahme, Übergabe & Dokumentation](abnahme-uebergabe-dokumentation.md) – Abschluss
- [Leitungssysteme & Organisation](../../Wirtschaft/Unternehmen/leitungssysteme-organisation.md) – Linien- und Matrixorganisation
