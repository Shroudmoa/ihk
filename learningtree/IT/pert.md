# PERT

# PERT – Program Evaluation and Review Technique

## Was ist PERT?

PERT ist eine Projektmanagement-Methode zur **Schätzung von Vorgangsdauern unter Unsicherheit**. Statt nur einen einzelnen Zeitwert zu schätzen, werden drei mögliche Dauern betrachtet. Daraus wird ein gewichteter Erwartungswert berechnet.

**Einsatzbereiche:**
- IT-Projekte und Softwareentwicklung
- Servermigrationen und Netzwerkumbauten
- Einführung neuer IT-Systeme
- Projektplanung bei unsicheren Aufwänden
- Abschätzung der Projektdauer und kritischer Vorgänge

## Dreipunktschätzung

Für jeden Vorgang werden drei Werte geschätzt:

| Kürzel | Bezeichnung | Bedeutung |
|---|---|---|
| O | Optimistisch (Optimistic) | Dauer, wenn alles besonders gut läuft |
| W | Wahrscheinlich (Most Likely) | Realistischste bzw. am ehesten erwartete Dauer |
| P | Pessimistisch (Pessimistic) | Dauer, wenn ungünstige, aber plausible Probleme auftreten |

## Formeln

### Erwartungswert

Der Erwartungswert gewichtet den wahrscheinlichsten Wert vierfach:

\[
E = \frac{O + 4W + P}{6}
\]

### Standardabweichung

Die Standardabweichung beschreibt bei der klassischen PERT-Näherung die Unsicherheit der geschätzten Dauer:

\[
\sigma = \frac{P-O}{6}
\]

### Varianz

Die Varianz wird insbesondere bei der Berechnung von Unsicherheiten entlang eines kritischen Pfads verwendet:

\[
\sigma^2 = \left(\frac{P-O}{6}\right)^2
\]

## Beispiel aus der IT-Praxis

Ein Unternehmen plant die Migration eines Servers. Die Dauer hängt von Datenmenge, Netzwerkgeschwindigkeit und möglichen Kompatibilitätsproblemen ab.

- Optimistisch: O = 10 Tage
- Wahrscheinlich: W = 15 Tage
- Pessimistisch: P = 26 Tage

**Erwartungswert:**

\[
E = \frac{10 + 4 \cdot 15 + 26}{6}
\]

\[
E = \frac{96}{6} = 16 \text{ Tage}
\]

**Standardabweichung:**

\[
\sigma = \frac{26-10}{6} \approx 2{,}67 \text{ Tage}
\]

**Ergebnis:** Die geschätzte Dauer beträgt 16 Tage. Die Standardabweichung beträgt ungefähr 2,67 Tage.

## Was bedeutet die Standardabweichung?

Eine größere Standardabweichung bedeutet bei dieser Schätzung eine größere Spannweite zwischen optimistischer und pessimistischer Dauer und damit eine höhere angenommene Unsicherheit.

Bei der klassischen PERT-Annahme wird die Dauer näherungsweise als Beta-Verteilung modelliert. Der Erwartungswert allein garantiert deshalb nicht, dass ein Vorgang tatsächlich nach dieser Zeit abgeschlossen ist.

Die Faustregel „Erwartungswert ± eine Standardabweichung entspricht ungefähr 68 % Wahrscheinlichkeit“ gilt nur bei einer entsprechenden Normalverteilungsannahme. Sie ist bei PERT nicht automatisch exakt gültig.

## PERT und kritischer Pfad

PERT kann mit der Netzplantechnik kombiniert werden. Dabei werden Vorgänge, Abhängigkeiten und geschätzte Dauern betrachtet.

Der **kritische Pfad** ist die längste abhängige Vorgangskette im Projektplan. Verzögert sich ein Vorgang auf diesem Pfad, verschiebt sich bei unverändertem Ablaufplan grundsätzlich auch der Projektendtermin.

Für unabhängige Vorgangsdauern werden bei einer klassischen PERT-Näherung die Varianzen der Vorgänge auf dem kritischen Pfad addiert:

\[
\sigma_{\text{Projekt}} =
\sqrt{\sum \sigma_i^2}
\]

Diese Berechnung setzt voraus, dass die betrachteten Dauern unabhängig sind und der kritische Pfad als gegeben behandelt werden kann. In realen Projekten können Abhängigkeiten und wechselnde kritische Pfade die Aussagekraft einschränken.

## Vorteile und Nachteile

| Vorteile | Nachteile |
|---|---|
| Berücksichtigt Unsicherheiten | Schätzwerte können subjektiv sein |
| Liefert einen gewichteten Erwartungswert | Ergebnis hängt von der Qualität der Schätzungen ab |
| Hilft bei der Projekt- und Terminplanung | Wahrscheinlichkeiten sind nur unter Annahmen ableitbar |
| Unterstützt die Analyse kritischer Vorgänge | Bei großen Projekten kann die Berechnung aufwendig werden |

## PERT vs. einfache Schätzung

| Einfache Schätzung | PERT |
|---|---|
| Verwendet einen einzelnen Zeitwert | Verwendet drei Zeitwerte |
| Schnell und einfach | Berücksichtigt Unsicherheit stärker |
| Keine eigene Streuungsschätzung | Liefert eine geschätzte Standardabweichung |
| Geeignet bei gut bekannten Aufgaben | Besonders hilfreich bei unsicheren Vorgängen |

## IHK-Merksatz

PERT (Program Evaluation and Review Technique) ist eine Projektmanagement-Methode zur Schätzung unsicherer Vorgangsdauern. Aus optimistischem, wahrscheinlichstem und pessimistischem Wert werden ein gewichteter Erwartungswert sowie eine geschätzte Standardabweichung berechnet. In Verbindung mit der Netzplantechnik kann PERT helfen, Projektdauern und Risiken kritischer Vorgänge besser einzuschätzen.
