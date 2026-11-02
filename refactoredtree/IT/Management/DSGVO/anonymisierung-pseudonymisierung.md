# Anonymisierung und Pseudonymisierung

## In 30 Sekunden

- **Pseudonymisierung:** Identifizierende Merkmale werden durch ein Kennzeichen ersetzt; mit der **getrennt aufbewahrten Zuordnungstabelle** ist der Personenbezug **wiederherstellbar** (Art. 4 Nr. 5 DSGVO).
- **Anonymisierung:** Der Personenbezug wird **unwiderruflich** entfernt – niemand kann die Person mit vertretbarem Aufwand wieder identifizieren.
- Pseudonymisierte Daten sind **weiterhin personenbezogen** (DSGVO gilt), anonymisierte Daten fallen **aus der DSGVO** heraus.
- Pseudonymisierung ist eine ausdrücklich genannte **TOM** (Art. 25, Art. 32).
- Prüfung 2021W GA1: Unterschied erklären.

## Kernwissen

| Merkmal | Pseudonymisierung | Anonymisierung |
|---|---|---|
| Personenbezug | mit Zusatzwissen herstellbar | dauerhaft entfernt |
| DSGVO anwendbar | **ja** | **nein** |
| Umkehrbar | ja (über die Zuordnungstabelle) | nein |
| Typischer Zweck | Verarbeitung mit geringerem Risiko, wenn später doch zugeordnet werden muss (Testdaten, Studien mit Rückfragen, Kundennummer statt Name) | Statistik, Veröffentlichung, Auswertung ohne Personenbezug |
| Beispiel | „Kunde K-4711“ statt „Max Müller“, Tabelle K-4711 → Max Müller liegt getrennt und geschützt | „32 % der Kunden aus Bayern bestellen monatlich“ |

**Techniken**
- Pseudonymisierung: Zufalls-IDs mit Zuordnungstabelle, **Hash mit geheimem Schlüssel** (HMAC) statt einfachem Hash, Tokenisierung, Verschlüsselung einzelner Felder
- Anonymisierung: **Aggregation** (nur Summen/Durchschnitte), **Generalisierung** (Alter 37 → „30–39“, PLZ 80331 → „80xxx“), Weglassen von Merkmalen, Verrauschen; Ziel z. B. **k-Anonymität** (jede Kombination kommt mindestens k-mal vor)

**Risiko Re-Identifizierung:** Schon wenige Merkmale zusammen (Geburtsdatum + Postleitzahl + Geschlecht) können eine Person eindeutig machen. Ein einfacher Hash einer E-Mail-Adresse ist **keine** Anonymisierung – er lässt sich durch Durchprobieren bekannter Adressen zurückrechnen.

## Prüfungsfrage

**Frage:** Erläutern Sie den Unterschied zwischen Anonymisierung und Pseudonymisierung personenbezogener Daten und geben Sie je ein Beispiel.

**Musterantwort:** Bei der Pseudonymisierung werden Namen und andere identifizierende Merkmale durch ein Kennzeichen ersetzt, etwa eine Kundennummer; über eine getrennt und geschützt aufbewahrte Zuordnungstabelle lässt sich die Person wieder ermitteln, deshalb gilt die DSGVO weiterhin. Bei der Anonymisierung wird der Personenbezug unwiderruflich entfernt, zum Beispiel indem nur noch zusammengefasste Verkaufszahlen pro Region ausgewertet werden; eine Rückführung auf einzelne Personen ist nicht mehr möglich, und die DSGVO findet keine Anwendung mehr.

## Falle

- Pseudonymisierte Daten als „anonym“ behandeln und frei weitergeben → Datenschutzverstoß.
- Zuordnungstabelle im selben System und mit denselben Rechten speichern → Schutz wirkungslos.
- Einfaches Hashen von Namen oder E-Mail-Adressen für Anonymisierung halten.
- Verschlüsselung mit Anonymisierung verwechseln – verschlüsselte Daten bleiben personenbezogen.

## Merksatz

**„Pseudonym: Schlüssel liegt im Tresor. Anonym: Es gibt keinen Schlüssel mehr.“**

## Verwandt

- [DSGVO-Grundlagen und Datenschutzkonzept](dsgvo-tom.md)
- [TOM](toms.md)
- [Rechtsgrundlagen und Grundsätze](rechtsgrundlagen-grundsaetze.md) – Datenminimierung
- [Hash, Salt und HMAC](../../FiSi-Administrator/Sicherheit/Verschluesselung/hash.md)
- [Lektion 05 Datenschutz und Informationssicherheit](../../analyse-der-exams/05_Datenschutz_Informationssicherheit.md)
