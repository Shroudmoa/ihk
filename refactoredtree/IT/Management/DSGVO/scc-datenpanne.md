# Datenpanne (Art. 33/34 DSGVO) und SCC

## In 30 Sekunden

- **Datenpanne** = Verletzung des Schutzes personenbezogener Daten: Vernichtung, Verlust, Veränderung, unbefugte Offenlegung oder unbefugter Zugang (Art. 4 Nr. 12).
- Meldung an die **Aufsichtsbehörde binnen 72 Stunden** nach Bekanntwerden – außer, es besteht voraussichtlich **kein Risiko** für die Betroffenen (Art. 33).
- Bei voraussichtlich **hohem Risiko** zusätzlich **unverzüglich die Betroffenen** informieren (Art. 34).
- **Jede** Datenpanne intern dokumentieren – auch die nicht meldepflichtige (Art. 33 Abs. 5).
- **SCC** (Standardvertragsklauseln) sind eine Grundlage für Datenübermittlungen in **Drittländer** (Art. 46).
- Prüfung: 2021W, 2024S (3 Sofortmaßnahmen), 2025S (3 Kriterien für die Kundeninformation).

## Kernwissen

### 1. Datenpanne – Datenschutzverletzung

#### Kurz gefasst

- **Art. 4 Nr. 12 DSGVO:** Eine Verletzung des Schutzes personenbezogener Daten führt zur Vernichtung, zum Verlust, zur Veränderung, zur unbefugten Offenlegung oder zum unbefugten Zugang.
- Betroffen sein können alle drei Schutzziele: **Vertraulichkeit, Integrität und Verfügbarkeit**.
- **Ransomware** kann eine Datenpanne sein, z. B. durch Verschlüsselung, Datenverlust oder Datenabfluss.
- Der Verantwortliche muss eine meldepflichtige Verletzung grundsätzlich innerhalb von **72 Stunden nach Bekanntwerden** der zuständigen Aufsichtsbehörde melden (Art. 33 DSGVO).
- Jede Datenschutzverletzung muss intern dokumentiert werden – auch wenn sie nicht meldepflichtig ist.

#### Beispiele und Schutzziele

| Vorfall | Betroffenes Schutzziel |
|---|---|
| Kundendaten werden veröffentlicht | Vertraulichkeit |
| Unbefugte verändern Kundendaten | Integrität |
| Ransomware verschlüsselt Daten | Verfügbarkeit |
| Backup wird versehentlich gelöscht | Verfügbarkeit |
| Laptop mit unverschlüsselten Kundendaten geht verloren | Vertraulichkeit, ggf. Verfügbarkeit |

#### Ablauf bei einer Datenpanne

1. **Erkennen und melden:** Vorfall intern an die zuständige Stelle weitergeben.
2. **Eindämmen:** Betroffene Systeme isolieren, Zugänge sperren und weitere Schäden verhindern.
3. **Bewerten und dokumentieren:** Datenarten, Umfang, Risiken, Ursachen und Gegenmaßnahmen erfassen.
4. **Behörde informieren:** Bei voraussichtlichem Risiko für Rechte und Freiheiten der Betroffenen grundsätzlich innerhalb von 72 Stunden.
5. **Betroffene informieren:** Bei voraussichtlich hohem Risiko grundsätzlich unverzüglich (Art. 34 DSGVO).
6. **Nachbereiten:** Ursache beheben, TOM anpassen und Wiederholungen verhindern.

**Prüfungsfalle:** Die 72-Stunden-Frist gilt nicht automatisch für jede Datenpanne. Die Meldung an die Aufsichtsbehörde entfällt, wenn die Verletzung voraussichtlich kein Risiko für die Rechte und Freiheiten natürlicher Personen zur Folge hat. Die interne Dokumentationspflicht bleibt bestehen.

#### Kriterien für die Information der Betroffenen (Prüfung 2025S)

- **Hohes Risiko** für Rechte und Freiheiten, z. B. Identitätsdiebstahl, finanzieller Schaden, Rufschädigung
- **Art der Daten**: besondere Kategorien (Gesundheit), Bank- und Kreditkartendaten, Zugangsdaten
- **Umfang**: Anzahl der Betroffenen und Datensätze
- **Ausnahmen** (Art. 34 Abs. 3): Daten waren wirksam **verschlüsselt**, das Risiko wurde nachträglich beseitigt, oder eine Einzelbenachrichtigung wäre unverhältnismäßig (dann öffentliche Bekanntmachung)

#### Inhalt der Meldung an die Behörde (Art. 33 Abs. 3)

Art der Verletzung, Kategorien und ungefähre Zahl der Betroffenen und Datensätze, Kontakt des DSB, wahrscheinliche Folgen, ergriffene oder vorgeschlagene Maßnahmen. Informationen dürfen schrittweise nachgereicht werden.

#### Kurzantwort

Eine Datenschutzverletzung liegt vor, wenn personenbezogene Daten verloren gehen, verändert, vernichtet, offengelegt oder unbefugt zugänglich werden. Der Verantwortliche muss die Verletzung dokumentieren und sie bei voraussichtlichem Risiko grundsätzlich innerhalb von 72 Stunden der zuständigen Aufsichtsbehörde melden. Bei voraussichtlich hohem Risiko müssen zusätzlich die betroffenen Personen unverzüglich informiert werden.

### 2. SCC – Standard Contractual Clauses

#### Kurz gefasst

- **SCC = Standardvertragsklauseln**, vorformulierte Vertragsklauseln der EU-Kommission.
- Sie dienen als geeignete Garantie für bestimmte Übermittlungen personenbezogener Daten in Drittländer außerhalb des EWR, sofern die gesetzlichen Voraussetzungen erfüllt sind.
- Rechtsgrundlage: **Art. 46 DSGVO**.
- SCC ersetzen nicht automatisch jede weitere Prüfung. Je nach Zielland und Übermittlung sind zusätzliche Schutzmaßnahmen erforderlich.

#### Die vier SCC-Module

| Modul | Vertragsbeziehung |
|---|---|
| Modul 1 | Verantwortlicher → Verantwortlicher |
| Modul 2 | Verantwortlicher → Auftragsverarbeiter |
| Modul 3 | Auftragsverarbeiter → Auftragsverarbeiter |
| Modul 4 | Auftragsverarbeiter → Verantwortlicher |

**Merksatz:** Die Rollen der beteiligten Parteien bestimmen das passende SCC-Modul.

#### Beispiel

Ein deutsches Unternehmen speichert Kundendaten bei einem Cloud-Anbieter in einem Drittland. Besteht kein einschlägiger Angemessenheitsbeschluss, können SCC als Übermittlungsgrundlage dienen. Zusätzlich muss geprüft werden, ob das Schutzniveau im Zielland praktisch gewährleistet ist und ergänzende Maßnahmen erforderlich sind.

#### Kurzantwort

Standard Contractual Clauses sind von der EU-Kommission vorformulierte Vertragsklauseln für bestimmte Datenübermittlungen in Drittländer. Sie können nach Art. 46 DSGVO geeignete Garantien für den Datenschutz schaffen. Vor der Übermittlung müssen das passende SCC-Modul, die rechtlichen Voraussetzungen und gegebenenfalls zusätzliche technische und organisatorische Schutzmaßnahmen geprüft werden.

## Prüfungsfrage

**Frage:** Ein Mitarbeiter verschickt versehentlich eine Excel-Liste mit Namen und Bankverbindungen von 300 Kunden an einen falschen externen Empfänger. Nennen Sie drei unverzügliche Maßnahmen nach DSGVO.

**Musterantwort:** Erstens wird der Vorfall sofort intern an den Datenschutzbeauftragten gemeldet und dokumentiert; der falsche Empfänger wird kontaktiert und aufgefordert, die Datei zu löschen und dies zu bestätigen. Zweitens wird die Datenpanne innerhalb von 72 Stunden nach Bekanntwerden der zuständigen Aufsichtsbehörde gemeldet, da durch die Bankdaten ein Risiko für die Kunden besteht. Drittens werden die betroffenen Kunden unverzüglich informiert, weil wegen möglichen Missbrauchs der Bankverbindungen ein hohes Risiko vorliegt; anschließend werden Ursachen analysiert und Maßnahmen wie Schulungen oder verschlüsselter Versand eingeführt.

## Falle

- „72 Stunden an Behörde **und** Betroffene“ → 72 Stunden gelten für die **Behörde**; Betroffene werden bei hohem Risiko **unverzüglich** informiert.
- Die Frist ab Beginn des Vorfalls rechnen → sie läuft ab **Bekanntwerden** beim Verantwortlichen.
- Nicht meldepflichtige Pannen gar nicht festhalten → die **interne Dokumentation** ist immer Pflicht.
- SCC als Allheilmittel sehen: zusätzlich muss geprüft werden, ob das Schutzniveau im Zielland tatsächlich gewährleistet ist.

## Merksatz

**„72 Stunden für die Behörde, unverzüglich für die Betroffenen, immer für die eigene Akte.“**

## Verwandt

- [Sicherheitsvorfall – Incident Response](../../FiSi-Administrator/Sicherheit/hacking/wurde-ich-gehackt.md)
- [Malware und Ransomware](../../FiSi-Administrator/Sicherheit/hacking/malware-ransomware.md)
- [Auftragsverarbeitung (AVV)](auftragsverarbeitung-avv.md)
- [TOM](toms.md)
- [Datenschutzbeauftragter](dbs-datenschutzbeauftragten.md)
- [Schutzziele](../../FiSi-Administrator/Sicherheit/schutzziele.md)
- [Lektion 05 Datenschutz und Informationssicherheit](../../analyse-der-exams/05_Datenschutz_Informationssicherheit.md)
