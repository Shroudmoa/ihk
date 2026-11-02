# ISMS, BSI IT-Grundschutz und ISO 27001

## In 30 Sekunden

- Ein **ISMS** (Informationssicherheits-Managementsystem) organisiert Informationssicherheit als **dauerhaften Prozess** mit Verantwortlichen, Leitlinie, Risikobewertung und Kontrolle – nicht als einmaliges Projekt.
- **ISO/IEC 27001** ist die internationale, **zertifizierbare** Norm für ein ISMS; die Maßnahmen stehen im Anhang A (Leitfaden: ISO 27002).
- Der **BSI IT-Grundschutz** ist die deutsche Methode dazu: **BSI-Standards 200-1 bis 200-4** plus **IT-Grundschutz-Kompendium** mit Bausteinen; ISO-27001-Zertifizierung auf Basis von IT-Grundschutz ist möglich.
- Kernschritt ist die **Schutzbedarfsfeststellung**: normal – hoch – sehr hoch, bezogen auf Vertraulichkeit, Integrität, Verfügbarkeit.
- Gearbeitet wird im **PDCA-Zyklus** (Plan – Do – Check – Act).

## Kernwissen

**BSI-Standards**

| Standard | Inhalt |
|---|---|
| **200-1** | Managementsysteme für Informationssicherheit (Anforderungen an ein ISMS) |
| **200-2** | IT-Grundschutz-Methodik: **Basis-**, **Standard-** oder **Kern-Absicherung** |
| **200-3** | Risikoanalyse auf Basis von IT-Grundschutz (bei hohem/sehr hohem Schutzbedarf) |
| **200-4** | Business Continuity Management (Notfallmanagement) |

**IT-Grundschutz-Kompendium:** Bausteine mit Anforderungen, gegliedert in Prozess-Bausteine (z. B. ISMS, ORP – Organisation und Personal, CON – Konzepte, OPS – Betrieb, DER – Detektion und Reaktion) und System-Bausteine (z. B. APP – Anwendungen, SYS – IT-Systeme, NET – Netze, INF – Infrastruktur, IND – Industrie).

**Vorgehen nach Standard-Absicherung (vereinfacht)**
1. **Leitlinie** zur Informationssicherheit von der Geschäftsleitung verabschieden, Informationssicherheitsbeauftragten (ISB) benennen
2. **Geltungsbereich** festlegen und **Strukturanalyse**: Geschäftsprozesse, Anwendungen, IT-Systeme, Räume, Netze erfassen
3. **Schutzbedarfsfeststellung**
4. **Modellierung**: passende Bausteine zuordnen
5. **IT-Grundschutz-Check**: Soll-Ist-Vergleich der Anforderungen
6. **Risikoanalyse** für Objekte mit hohem Schutzbedarf
7. **Umsetzung** der Maßnahmen planen und realisieren
8. **Aufrechterhaltung**: Audits, Revision, kontinuierliche Verbesserung

**Schutzbedarfsfeststellung**

| Kategorie | Bedeutung |
|---|---|
| **normal** | Schadensauswirkungen begrenzt und überschaubar |
| **hoch** | Schadensauswirkungen beträchtlich |
| **sehr hoch** | Schadensauswirkungen existenzbedrohend oder katastrophal |

Bewertet wird je Grundwert (Vertraulichkeit, Integrität, Verfügbarkeit) anhand von Schadensszenarien: Verstoß gegen Gesetze/Verträge, Beeinträchtigung des informationellen Selbstbestimmungsrechts, der persönlichen Unversehrtheit, der Aufgabenerfüllung, negative Innen- oder Außenwirkung, finanzielle Auswirkungen.

Vererbung des Schutzbedarfs vom Geschäftsprozess über die Anwendung auf das IT-System und den Raum:
- **Maximumprinzip:** Das IT-System erhält den höchsten Schutzbedarf der darauf laufenden Anwendungen.
- **Kumulationseffekt:** Viele Anwendungen mit „normal“ auf einem Host können zusammen „hoch“ ergeben (z. B. Virtualisierungshost).
- **Verteilungseffekt:** Ist eine Anwendung redundant auf mehrere Systeme verteilt, kann der Schutzbedarf des einzelnen Systems (bei der Verfügbarkeit) niedriger sein.

**ISO 27001 vs. IT-Grundschutz**

| | ISO/IEC 27001 | BSI IT-Grundschutz |
|---|---|---|
| Herkunft | international | deutsch (BSI) |
| Ansatz | Anforderungen an das ISMS, Risikoanalyse durch das Unternehmen selbst | konkrete Bausteine mit vorgegebenen Anforderungen |
| Zertifikat | ISO 27001 | ISO 27001 auf Basis von IT-Grundschutz |
| Typisch | international tätige Firmen, Cloud-Anbieter | Behörden, KRITIS, deutsche Mittelständler |

**Rechtlicher Rahmen am Rand:** Das **BSI-Gesetz** verpflichtet Betreiber kritischer Infrastrukturen (KRITIS) zu Sicherheitsmaßnahmen und Meldungen an das BSI. Die EU-Richtlinie **NIS2** erweitert diese Pflichten auf deutlich mehr Unternehmen („wichtige“ und „besonders wichtige“ Einrichtungen) und sieht ein gestuftes Meldeverfahren vor: Frühwarnung binnen **24 Stunden**, Meldung binnen **72 Stunden**, Abschlussbericht nach einem Monat; die Geschäftsleitung haftet für die Umsetzung. In Deutschland wird NIS2 über eine Neufassung des BSI-Gesetzes umgesetzt.

## Prüfungsfrage

**Frage:** Ein Unternehmen möchte ein ISMS nach BSI IT-Grundschutz aufbauen. Erläutern Sie die Schutzbedarfsfeststellung und das Maximumprinzip an einem Beispiel.

**Musterantwort:** Bei der Schutzbedarfsfeststellung wird für jeden Geschäftsprozess, jede Anwendung und jedes IT-System getrennt nach Vertraulichkeit, Integrität und Verfügbarkeit bewertet, wie groß der Schaden bei einer Verletzung wäre; die Kategorien sind normal, hoch und sehr hoch. Nach dem Maximumprinzip übernimmt ein IT-System den höchsten Schutzbedarf der Anwendungen, die darauf laufen. Läuft auf einem Server sowohl das Intranet mit normalem als auch die Personalverwaltung mit hohem Vertraulichkeitsbedarf, hat der Server insgesamt einen hohen Schutzbedarf bei der Vertraulichkeit.

## Falle

- ISMS mit „Firewall und Virenscanner“ verwechseln – ein ISMS ist ein **Managementprozess** mit Verantwortung der Leitung.
- Schutzbedarf nur pauschal pro System angeben, ohne die drei Grundwerte getrennt zu betrachten.
- ISO 27001 und ISO 27002 verwechseln: **27001 ist zertifizierbar**, 27002 ist der Maßnahmen-Leitfaden.
- Informationssicherheitsbeauftragten (ISB) und Datenschutzbeauftragten (DSB) gleichsetzen: der ISB schützt **alle Informationen**, der DSB wacht über **personenbezogene Daten**.

## Merksatz

**„Erst Struktur, dann Schutzbedarf, dann Bausteine – und das Ganze im PDCA-Kreis.“**

## Verwandt

- [Schutzziele](schutzziele.md) – die drei Grundwerte
- [TOM nach DSGVO](../../Management/DSGVO/toms.md) – Maßnahmen für personenbezogene Daten
- [Datenschutzbeauftragter](../../Management/DSGVO/dbs-datenschutzbeauftragten.md)
- [Penetrationstest und Schwachstellenmanagement](penetrationstest-schwachstellenmanagement.md) – Wirksamkeit prüfen
- [Sicherheitsvorfall – Incident Response](hacking/wurde-ich-gehackt.md)
- [Lektion 03 Verfügbarkeit und Ausfallsicherheit](../../analyse-der-exams/03_Verfuegbarkeit_Ausfallsicherheit.md)
- [Lektion 05 Datenschutz und Informationssicherheit](../../analyse-der-exams/05_Datenschutz_Informationssicherheit.md)
