# Ressourcenplan

## In 30 Sekunden

- Ein **Ressourcenplan** ordnet den Arbeitspaketen die benötigten **Personal-, Sach- und Finanzmittel** zu und zeigt, **wann** sie gebraucht werden.
- Ressourcenarten: **Personal** (Qualifikation, Kapazität), **Sachmittel** (Hardware, Software, Räume, Fahrzeuge), **Finanzmittel** (Budget).
- **Bedarf** (aus PSP, Aufwandsschätzung, Netzplan) wird der **Kapazität** (Verfügbarkeit) gegenübergestellt → Engpässe und Überlastungen werden sichtbar.
- Darstellung als **Ressourcenhistogramm** (Auslastung pro Zeiteinheit) oder Ressourcentabelle.
- **Ressourcenausgleich (Nivellierung):** Vorgänge innerhalb ihrer Puffer verschieben, um Spitzen abzubauen – reicht der Puffer nicht, verschiebt sich das Projektende.

## Kernwissen

### Vorgehen

1. Arbeitspakete aus dem [PSP](projektstrukturplan-psp.md) mit Aufwand aus der [Aufwandsschätzung](aufwandsschaetzung.md) übernehmen.
2. Benötigte Qualifikationen und Sachmittel je Arbeitspaket bestimmen.
3. Termine aus [Netzplan](netzplan.md)/[Gantt](gantt-diagramm.md) übernehmen.
4. Bedarf pro Zeiteinheit summieren und mit der Kapazität vergleichen.
5. Überlastung ausgleichen: verschieben (Puffer), umverteilen, Überstunden, externe Kräfte, Umfang reduzieren.

### Rechenbeispiel: Netzwerktechniker, Kapazität 1 Person = 8 h pro Tag

| Vorgang | Zeitraum (FAZ–FEZ) | Bedarf Netzwerktechnik | Gesamtpuffer |
|---|---|---:|---:|
| C Verkabelung | 3–7 | 8 h/Tag | 0 (kritisch) |
| G Switch-Konfiguration | 3–5 | 8 h/Tag | 4 |

Histogramm (Bedarf in h pro Tag):

| Tag | 3 | 4 | 5 | 6 | 7 | 8 |
|---|---:|---:|---:|---:|---:|---:|
| vor Ausgleich | **16** | **16** | 8 | 8 | 0 | 0 |
| nach Verschiebung von G um 4 Tage (auf 7–9) | 8 | 8 | 8 | 8 | 8 | 8 |

Tage 3 und 4 sind überlastet (16 h bei 8 h Kapazität). Da G 4 Tage Puffer hat, wird G auf Tag 7–9 verschoben: Die Auslastung liegt dauerhaft bei 8 h, das Projektende bleibt unverändert.

### Kapazität berechnen

`Kapazität = Personen × Arbeitstage × Stunden pro Tag × Verfügbarkeitsfaktor`

Beispiel: 2 Administratoren, 20 Arbeitstage, 8 h, davon 50 % für das Projekt (Rest Tagesgeschäft): 2 × 20 × 8 × 0,5 = **160 h**. Benötigt werden laut Schätzung 190 h → Engpass von 30 h, Maßnahmen nötig.

### Maßnahmen bei Engpässen

| Maßnahme | Wirkung | Nachteil |
|---|---|---|
| Vorgang im Puffer verschieben | Spitze geglättet, Ende unverändert | Puffer aufgebraucht |
| Überstunden | kurzfristig mehr Kapazität | Kosten, Fehlerrisiko, Arbeitszeitgesetz |
| externe Dienstleister | Know-how und Kapazität | Kosten, Einarbeitung |
| Umfang reduzieren | weniger Bedarf | Leistung sinkt (magisches Dreieck) |
| Projektende verschieben | Entlastung | Terminziel verfehlt |

## Prüfungsfrage

**Frage:** Im Rollout-Projekt sollen zwei Techniker in 10 Arbeitstagen 60 Arbeitsplätze umstellen. Pro Arbeitsplatz werden 2,5 Stunden benötigt, die Techniker stehen zu 75 % für das Projekt zur Verfügung (8-Stunden-Tag). Prüfen Sie, ob die Kapazität ausreicht, und nennen Sie zwei Maßnahmen bei einem Engpass.

**Musterantwort:** Bedarf: 60 × 2,5 h = **150 h**. Kapazität: 2 × 10 × 8 h × 0,75 = **120 h**. Es fehlen **30 h** – die Kapazität reicht nicht. Maßnahmen: (1) einen **dritten Techniker** oder einen externen Dienstleister für ca. 4 Tage hinzuziehen bzw. die Techniker vom Tagesgeschäft stärker freistellen (Verfügbarkeit 100 % ergäbe 160 h); (2) den **Zeitraum verlängern** auf mindestens 13 Arbeitstage (150 h ÷ 12 h pro Tag = 12,5 Tage) oder den Aufwand pro Platz durch automatisierte Softwareverteilung senken.

## Falle

- **Kapazität mit 100 % ansetzen:** Tagesgeschäft, Urlaub und Meetings senken die Verfügbarkeit.
- **Aufwand = Dauer:** 150 h Aufwand bei zwei Personen sind nicht 150 h Dauer.
- **Nivellierung über den Puffer hinaus:** dann verschiebt sich das Projektende.
- **Sachmittel vergessen:** Testumgebung, Lizenzen, Fahrzeuge oder Räume können ebenfalls Engpässe sein.
- **Qualifikation ignorieren:** Ein Anwendungsbetreuer ersetzt keinen Netzwerktechniker.

## Merksatz

**"Bedarf gegen Kapazität – Spitzen in den Puffer schieben."**

## Verwandt

- [Projektstrukturplan (PSP)](projektstrukturplan-psp.md) – Arbeitspakete
- [Aufwandsschätzung](aufwandsschaetzung.md) – Aufwand je Arbeitspaket
- [Netzplan](netzplan.md) – Puffer für den Ausgleich
- [Gantt-Diagramm](gantt-diagramm.md) – parallele Vorgänge
- [Stundensatz intern](stundensatz-intern.md) – Personalkosten
- [Soll-Ist-Vergleich](soll-ist-vergleich.md) – Ressourcenverbrauch kontrollieren
- [Arbeitszeit, Pausen, Urlaub](../../Wirtschaft/arbeitszeit-pausen-urlaub.md) – Grenzen für Überstunden
