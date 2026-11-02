# BPMN (Business Process Model and Notation)

## In 30 Sekunden

- **BPMN 2.0** (Business Process Model and Notation, OMG) ist eine standardisierte Notation für **Geschäftsprozesse** – verständlich für Fachabteilung und IT, keine UML.
- **Ereignisse** (Kreise: Start, Zwischen, Ende), **Aktivitäten/Tasks** (abgerundete Rechtecke), **Gateways** (Rauten: XOR, AND, OR).
- **Pool** = Teilnehmer (Organisation), **Lane** = Rolle/Abteilung im Pool.
- **Sequenzfluss** (durchgezogen) innerhalb eines Pools, **Nachrichtenfluss** (gestrichelt) zwischen Pools.
- Gegenstück in UML: das [Aktivitätsdiagramm](Aktivitaetsdiagramm.md).

## Kernwissen

### Definition
**BPMN** ist eine standardisierte grafische Notation zur Darstellung von Geschäftsprozessen. Sie dient als gemeinsame Sprache zwischen Fachabteilungen, IT und Management.

- Herausgeber: **OMG (Object Management Group)**
- Aktuelle Version im verbreiteten BPMN-2.x-Standard: BPMN 2.0
- Ziel: Geschäftsprozesse einheitlich, verständlich und eindeutig darstellen.

### Vorteile von BPMN
- **Transparenz:** Prozesse sind für alle Beteiligten nachvollziehbar.
- **Standardisierung:** Einheitliche Darstellung über Abteilungen hinweg.
- **Weniger Missverständnisse:** Grafische Symbole haben festgelegte Bedeutungen.
- **Optimierung:** Engpässe, unnötige Schritte und Doppelarbeit werden sichtbar.
- **Automatisierung:** BPMN-Modelle können von geeigneten Process Engines ausgeführt werden.
- **Compliance:** Prozesse können dokumentiert und für Audits nachvollziehbar gemacht werden.
- **Onboarding:** Neue Mitarbeitende können Abläufe leichter verstehen.

### Zentrale BPMN-Elemente

#### 1. Events (Ereignisse)
Werden als **Kreise** dargestellt und beschreiben Ereignisse, die einen Prozess auslösen, währenddessen auftreten oder ihn beenden.

- **Startereignis:** Dünner Kreis – startet den Prozess.
- **Zwischenereignis:** Doppelter Kreis – tritt während des Prozesses auf.
- **Endereignis:** Dicker Kreis – beendet den Prozess.

Ereignisse können beispielsweise durch Nachrichten, Zeitabläufe oder Fehler ausgelöst werden.

#### 2. Tasks (Aktivitäten)
Werden als **Rechtecke mit abgerundeten Ecken** dargestellt.

Sie beschreiben Tätigkeiten, die ausgeführt werden.

Beispiele:
- Bestellung prüfen
- Rechnung erstellen
- Zahlung durchführen

**Merksatz:** Eine Task-Bezeichnung besteht idealerweise aus Verb + Objekt, z. B. „Rechnung prüfen“.

Ein Task kann durch ein Symbol näher gekennzeichnet werden, beispielsweise als Benutzer- oder Service-Task.

#### 3. Gateways (Verzweigungen)
Werden als **Rauten** dargestellt und steuern den Ablauf des Prozesses.

- **XOR (exklusiv):** Genau ein möglicher Pfad wird ausgewählt.
- **AND (parallel):** Alle ausgehenden Pfade werden aktiviert.
- **OR (inklusiv):** Ein oder mehrere Pfade werden aktiviert.

Beispiel XOR:
„Rechnung korrekt?“
- Ja → Rechnung bezahlen
- Nein → Rechnung korrigieren

**Tipp:** Verzweigungen mit einer klaren Frage beschriften und die ausgehenden Pfeile mit den möglichen Antworten kennzeichnen.

#### 4. Verbindungen (Flows)

- **Sequenzfluss:** Durchgezogene Linie mit Pfeil. Zeigt die Reihenfolge der Aktivitäten innerhalb eines Pools.
- **Nachrichtenfluss:** Gestrichelte Linie mit offenem Pfeil. Zeigt den Nachrichtenaustausch zwischen unterschiedlichen Teilnehmern bzw. Pools.
- **Assoziation:** Gepunktete Verbindung. Verknüpft beispielsweise Aufgaben mit ergänzenden Informationen oder Datenobjekten.

### Pools und Lanes

#### Pool
Stellt einen Teilnehmer dar, beispielsweise:
- Ein Unternehmen
- Einen Kunden
- Einen externen Dienstleister

#### Lane
Unterteilt einen Pool in Rollen, Abteilungen oder Zuständigkeiten.

Beispiele:
- Vertrieb
- Buchhaltung
- IT-Abteilung

**Wichtige Regel:**
- Sequenzflüsse verlaufen innerhalb eines Pools.
- Zwischen verschiedenen Pools werden Nachrichtenflüsse verwendet.
- Nachrichtenflüsse dürfen nicht innerhalb desselben Pools verwendet werden.

### Token-Konzept
Ein **Token** steht vereinfacht für den Ablauf eines Prozesses.

- Es bewegt sich entlang der Sequenzflüsse.
- An einem XOR-Gateway folgt es genau einem ausgewählten Pfad.
- An einem AND-Gateway wird der Ablauf auf mehrere parallele Pfade aufgeteilt.
- An einem synchronisierenden AND-Gateway wartet der Ablauf auf alle erforderlichen eingehenden Pfade.

Das Token-Konzept hilft, den tatsächlichen Ablauf eines BPMN-Modells zu verstehen.

### Regeln für übersichtliche BPMN-Diagramme

- Prozesse möglichst von **links nach rechts** darstellen.
- Nicht zu viele Aktivitäten in ein Diagramm packen; etwa 15–20 Tasks können ein grober Orientierungswert sein, sind aber keine feste BPMN-Regel.
- Bei umfangreichen Prozessen Teilprozesse verwenden.
- Tasks mit **Verb + Objekt** benennen.
- Gateways eindeutig beschriften.
- Bei Verzweigungen die Bedingungen an den ausgehenden Sequenzflüssen angeben.
- Start- und Endereignisse sinnvoll festlegen. Für einen üblichen, vollständigen Prozess ist mindestens ein Start- und ein Endereignis empfehlenswert.

### BPMN vs. Flussdiagramm

| BPMN | Klassisches Flussdiagramm |
|---|---|
| Standardisierte Notation für Geschäftsprozesse | Allgemeine Darstellung von Abläufen |
| Pools und Lanes für Teilnehmer und Zuständigkeiten | Zuständigkeiten nicht standardmäßig über Pools und Lanes abgebildet |
| Eigene Symbole für Ereignisse, Gateways und Nachrichtenflüsse | Häufig einfachere Symbolauswahl |
| Kann mit geeigneten Process Engines ausführbar sein | Normalerweise nicht direkt ausführbar |

### Typische IHK-Prüfungsfragen

**1. Wofür steht BPMN?**
Business Process Model and Notation.

**2. Was ist der Unterschied zwischen einem Event und einer Task?**
Ein Event beschreibt ein Ereignis oder einen Zustand im Prozessablauf. Eine Task beschreibt eine auszuführende Tätigkeit.

**3. Was ist der Unterschied zwischen XOR und AND?**
XOR wählt genau einen Pfad aus. AND aktiviert alle ausgehenden Pfade parallel.

**4. Was ist der Unterschied zwischen Pool und Lane?**
Ein Pool stellt einen Teilnehmer dar. Eine Lane unterteilt diesen Teilnehmer beispielsweise nach Rollen oder Abteilungen.

**5. Wann wird ein Nachrichtenfluss verwendet?**
Zur Darstellung des Nachrichtenaustauschs zwischen unterschiedlichen Pools.

**6. Warum wird BPMN eingesetzt?**
Zur verständlichen, standardisierten Dokumentation, Analyse, Optimierung und gegebenenfalls Automatisierung von Geschäftsprozessen.

## Prüfungsfrage

**Frage:** Modellieren Sie den Prozess "Passwort-Reset" in BPMN: Ein Mitarbeiter meldet sich beim Service Desk; der Service Desk prüft die Identität. Ist die Prüfung erfolgreich, setzt er das Passwort zurück und informiert den Mitarbeiter, sonst wird die Anfrage abgelehnt. Nennen Sie die verwendeten Elemente.

**Musterantwort:** Pool "Unternehmen" mit den Lanes **Mitarbeiter** und **Service Desk**. In der Lane Mitarbeiter: **Startereignis** "Passwort vergessen" → Task "Reset anfordern". Sequenzfluss in die Lane Service Desk: Task "Identität prüfen" → **exklusives Gateway (XOR)** "Identität bestätigt?". Pfad *ja*: Task "Passwort zurücksetzen" → Task "Mitarbeiter informieren" → **Endereignis** "Passwort zurückgesetzt". Pfad *nein*: Task "Anfrage ablehnen" → Endereignis "Reset abgelehnt". Ist der Service Desk ein externer Dienstleister, wird er als eigener Pool modelliert und die Anfrage als **Nachrichtenfluss** (gestrichelt) dargestellt.

## Falle

- **Sequenzfluss zwischen Pools:** Zwischen Pools ist nur Nachrichtenfluss erlaubt.
- **XOR und AND verwechselt:** XOR wählt genau einen Pfad, AND startet alle parallel und wartet beim Zusammenführen auf alle.
- **Gateway ohne Zusammenführung:** Parallele Pfade (AND) müssen vor dem Ende wieder mit einem AND-Gateway synchronisiert werden.
- **Ereignis und Task vertauscht:** "Rechnung eingegangen" ist ein Ereignis, "Rechnung prüfen" eine Task.
- **BPMN als UML-Diagramm bezeichnen.**

## Merksatz

- **Event = Ereignis, Task = Tätigkeit, Gateway = Ablaufentscheidung oder Ablaufsteuerung.**
- **XOR = genau ein Pfad, AND = alle Pfade, OR = ein oder mehrere Pfade.**
- **Pool = Teilnehmer, Lane = Rolle oder Abteilung.**
- **Sequenzfluss = Reihenfolge innerhalb eines Pools.**
- **Nachrichtenfluss = Kommunikation zwischen Pools.**
- BPMN schafft eine gemeinsame Sprache zwischen Fachbereich, IT und Management.

## Verwandt

- [Aktivitätsdiagramm](Aktivitaetsdiagramm.md) – UML-Gegenstück zu BPMN
- [UML-Übersicht](readme.md)
- [ITIL: Incident, Problem, Change](../../Management/itil-incident-problem-change.md) – typische IT-Prozesse zum Modellieren
- [Zustandsdiagramm](zustandsdiagramm.md)
- [Lektion 14: Datenmodellierung, SQL & UML](../../analyse-der-exams/L14_Datenmodellierung_SQL_UML.md)
