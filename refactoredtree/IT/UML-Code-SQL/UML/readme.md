# UML - Unified Modeling Language

> UML zur Prüfungsvorbereitung für die IHK AP2 (FiSi) - Fokus auf prüfungsrelevante Diagramme

## Prüfungsrelevanz

Laut Analyse der Prüfungen 2021-2026:
- **UML explizit abgefragt:** 2021W (Zuordnung statische/dynamische Diagramme) und 2022S (Aktivitätsdiagramm zur NAS-Speicherbelegung vervollständigen)
- **Verwandte Themen:** Aggregation/Komposition (2023S), Datenkapselung/Vererbung (2022W)
- **Trend:** Seit 2023 eher ER-Diagramme und Normalisierung, UML-Grundlagen bleiben aber relevant

## Statische vs. dynamische Sicht

| Sicht | Zweck | Diagramme |
|---|---|---|
| **Statisch** (Strukturdiagramme) | Aufbau des Systems | Klassendiagramm, Objektdiagramm, Paketdiagramm, Komponentendiagramm, Verteilungsdiagramm |
| **Dynamisch** (Verhaltensdiagramme) | Abläufe, Interaktionen, Zustände | Aktivitätsdiagramm, Sequenzdiagramm, Zustandsdiagramm, Anwendungsfalldiagramm, Kommunikationsdiagramm |

Das **Anwendungsfalldiagramm** zählt in UML 2 offiziell zu den Verhaltensdiagrammen, beschreibt aber keinen zeitlichen Ablauf. Das **Kommunikationsdiagramm** (früher Kollaborationsdiagramm) ist ein Interaktionsdiagramm; manche Prüfungslösungen ordnen das Kollaborationsdiagramm der statischen Sicht zu, weil es die Struktur der Objektbeziehungen zeigt.

## Übersicht der Diagramme

| Diagramm | Typ | Wann nutzen? | Prüfungsrelevanz |
|---|---|---|---|
| **[Klassendiagramm](Klassendiagramm.md)** | Statisch | Klassen, Attribute, Methoden, Vererbung, Beziehungen | ★★★★★ |
| **[Aktivitätsdiagramm](Aktivitaetsdiagramm.md)** | Dynamisch | Abläufe, Entscheidungen, Verantwortlichkeiten (Swimlanes) | ★★★★★ |
| **[Anwendungsfalldiagramm/Use-Case-Diagramm](Anwendungsfalldiagramm-Use-Case-Diagramm.md)** | Verhalten (ohne Ablauf) | Anforderungen, Akteure, Systemgrenze, Use Cases | ★★★★☆ |
| **[Sequenzdiagramm](Sequenzdiagramm.md)** | Dynamisch | Nachrichten zwischen Objekten über die Zeit | ★★★★☆ |
| **[Zustandsdiagramm](zustandsdiagramm.md)** | Dynamisch | Zustände eines Objekts und Übergänge | ★★★☆☆ |
| **[BPMN](BPMN.md)** | keine UML, Prozessnotation | Geschäftsprozesse fachlich modellieren (Pools, Lanes, Gateways) | ★★★☆☆ |

## Lernpfad

1. **[Klassendiagramm](Klassendiagramm.md)** - Klassen, Assoziation, Aggregation, Komposition, Vererbung
2. **[Aktivitätsdiagramm](Aktivitaetsdiagramm.md)** - Abläufe modellieren (häufig als Zeichenaufgabe)
3. **[Anwendungsfalldiagramm/Use-Case-Diagramm](Anwendungsfalldiagramm-Use-Case-Diagramm.md)** - Anforderungen erfassen
4. **[Sequenzdiagramm](Sequenzdiagramm.md)** - Interaktionen zwischen Objekten
5. **[Zustandsdiagramm](zustandsdiagramm.md)** - Lebenszyklus eines Objekts
6. **[BPMN](BPMN.md)** - Geschäftsprozesse für Fachabteilungen

## Verknüpfung

- [Datenmodellierung, SQL & UML (Analyse der Exams)](../../analyse-der-exams/L14_Datenmodellierung_SQL_UML.md) - echte Prüfungsfälle
- [Programmierung (Analyse der Exams)](../../analyse-der-exams/04_Programmierung_Grundlagen.md) - UML-Zuordnung 2021W, Aktivitätsdiagramm 2022S
- [ER-Modell & Kardinalitäten](../sql/er-modell-kardinalitaeten.md) - Datenmodell statt Klassenmodell
- [Struktogramm & PAP](../Pseudocode/struktogramm-pap.md) - Ablaufdarstellung ohne UML
- [OOP-Grundlagen](../Pseudocode/oop-grundlagen.md) - Klassen, Kapselung, Vererbung im Code
- [Pseudocode](../Pseudocode/readme.md)
