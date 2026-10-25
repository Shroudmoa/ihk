# Aufwandsschätzung

# Aufwandsschätzung – IHK AP2

## Definition

Die **Aufwandsschätzung** ist die Vorhersage des benötigten Arbeitsaufwands für ein Projekt, meist in **Personenstunden (h)** oder **Personentagen (PT)**.

Sie dient der Projektplanung, Terminplanung, Personalplanung und Kostenkalkulation.

**Wichtig:** Aufwand ist nicht gleich Dauer.
- **Aufwand:** 40 Personenstunden
- **Dauer:** 5 Arbeitstage bei einer Person mit 8 Stunden pro Tag
- Arbeiten mehrere Personen parallel, kann die Dauer sinken, aber nicht immer proportional.

## Methoden der Aufwandsschätzung

| Methode | Erklärung | Vorteil | Nachteil |
|---|---|---|---|
| **Expertenschätzung** | Erfahrene Fachleute schätzen den Aufwand. | Schnell und einfach | Subjektiv, abhängig von Erfahrung |
| **Analogieschätzung** | Vergleich mit ähnlichen, bereits abgeschlossenen Projekten. | Praxisnah und schnell | Nur zuverlässig bei vergleichbaren Projekten |
| **Dreipunktschätzung** | Betrachtet optimistischen, wahrscheinlichen und pessimistischen Aufwand. | Berücksichtigt Unsicherheiten | Qualität hängt von den Schätzwerten ab |
| **Planning Poker** | Teammitglieder schätzen Aufgaben mithilfe von Karten, häufig mit Fibonacci-Werten. | Fördert Diskussion und gemeinsame Einschätzung | Benötigt Zeit und funktioniert nicht ohne ausreichende Informationen |
| **Function-Point-Analyse** | Schätzt Software anhand ihrer fachlichen Funktionalität, z. B. Eingaben, Ausgaben und Datenbestände. | Auch vor vollständiger Implementierung möglich | Benötigt Erfahrung und klare Anforderungen |

## Dreipunktschätzung

Es werden drei Werte bestimmt:

- **O – Optimistisch:** Aufwand bei sehr günstigem Verlauf.
- **W – Wahrscheinlich:** Realistisch erwarteter Aufwand.
- **P – Pessimistisch:** Aufwand bei ungünstigem Verlauf.

### Formel: gewichteter Mittelwert (PERT)

\[
E = \frac{O + 4W + P}{6}
\]

Der wahrscheinlichste Wert wird vierfach gewichtet.

### Beispiel

Ein Softwaremodul soll entwickelt werden:

- Optimistisch: 2 Personentage
- Wahrscheinlich: 4 Personentage
- Pessimistisch: 8 Personentage

\[
E = \frac{2 + 4 \cdot 4 + 8}{6}
\]

\[
E = \frac{26}{6} \approx 4{,}33 \text{ Personentage}
\]

**Ergebnis:** Der geschätzte Aufwand beträgt rund **4,33 Personentage**.

## Planning Poker – Ablauf

1. Das Team bespricht eine Aufgabe und ihre Anforderungen.
2. Alle Beteiligten wählen verdeckt eine Schätzkarte.
3. Die Karten werden gleichzeitig aufgedeckt.
4. Bei großen Abweichungen erläutern die Beteiligten ihre Einschätzungen.
5. Das Team diskutiert und schätzt erneut, bis ein gemeinsamer Wert festgelegt ist.

Planning Poker verwendet häufig Fibonacci-Werte wie `1, 2, 3, 5, 8, 13, 21`.

Diese Werte können Aufwand relativ ausdrücken, beispielsweise als **Story Points**. Story Points sind keine festen Personenstunden.

## Wichtige Prüfungsfallen

- **Aufwand ≠ Dauer:** 16 Personenstunden entsprechen bei einer Person mit 8 Stunden täglich zwei Arbeitstagen.
- **Schätzung ≠ Garantie:** Tatsächlicher Aufwand kann abweichen.
- **Unklare Anforderungen** erhöhen die Schätzunsicherheit.
- **Puffer** können für Risiken eingeplant werden, sollten aber nachvollziehbar begründet sein.
- **Function Points** bewerten fachliche Funktionalität, nicht einfach die Anzahl der Codezeilen.

## Merksatz

**Expertenschätzung = Erfahrung, Analogieschätzung = Vergleich, Dreipunktschätzung = Unsicherheit, Planning Poker = Team, Function Points = Softwarefunktionalität.**
