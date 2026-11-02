# DSGVO-Grundlagen und Datenschutzkonzept

> Abgrenzung: Dieses Blatt gibt den **Überblick** über die DSGVO (Begriffe, Rollen, Datenschutzkonzept). Die **TOM im Detail** (Kontrollarten, Zuordnungsaufgaben) stehen in [TOM](toms.md), die **Rechtsgrundlagen und Grundsätze** in [Rechtsgrundlagen und Grundsätze](rechtsgrundlagen-grundsaetze.md).

## In 30 Sekunden

- Die **DSGVO** schützt **personenbezogene Daten** natürlicher Personen; sie gilt EU-weit, das **BDSG** ergänzt sie in Deutschland.
- Personenbezogen ist alles, was sich einer Person zuordnen lässt – auch **IP-Adresse, Personalnummer, Kfz-Kennzeichen, Videoaufnahme**.
- Jede Verarbeitung braucht eine **Rechtsgrundlage** (Art. 6) und muss die **Grundsätze** (Art. 5) einhalten: Zweckbindung, Datenminimierung, Speicherbegrenzung …
- Sicherheit der Verarbeitung über **TOM** (Art. 32), Dienstleister über **AVV** (Art. 28), Datenpannen binnen **72 Stunden** an die Behörde (Art. 33).
- Prüfung 2024S: **vier Inhalte eines Datenschutzkonzepts** nennen.

## Kernwissen

**Wichtige Begriffe (Art. 4)**

| Begriff | Bedeutung |
|---|---|
| Personenbezogene Daten | alle Informationen über eine identifizierte oder identifizierbare natürliche Person |
| Besondere Kategorien (Art. 9) | Gesundheit, Religion, politische Meinung, Gewerkschaft, ethnische Herkunft, Sexualleben, **genetische und biometrische Daten** – Verarbeitung grundsätzlich verboten, nur mit Ausnahme erlaubt |
| Verarbeitung | jeder Vorgang: erheben, speichern, ändern, abfragen, übermitteln, löschen |
| Pseudonymisierung | Personenbezug nur mit getrennt aufbewahrten Zusatzinformationen herstellbar – bleibt personenbezogen |
| Verletzung des Schutzes | Datenpanne: Vernichtung, Verlust, Veränderung, unbefugte Offenlegung oder unbefugter Zugang |

**Rollen**

| Rolle | Aufgabe |
|---|---|
| **Verantwortlicher** | entscheidet über Zweck und Mittel der Verarbeitung, haftet |
| **Auftragsverarbeiter** | verarbeitet weisungsgebunden im Auftrag (Cloud, Hosting, Lohnabrechnung), braucht einen **AVV** |
| **Betroffene Person** | die Person, um deren Daten es geht – hat die **Betroffenenrechte** |
| **Datenschutzbeauftragter (DSB)** | berät und überwacht intern, weisungsfrei |
| **Aufsichtsbehörde** | Landesdatenschutzbehörde bzw. BfDI – kontrolliert, nimmt Meldungen an, verhängt Bußgelder |

**Datenschutzkonzept – Inhalte (nenne 4, Prüfung 2024S)**
1. **Verzeichnis von Verarbeitungstätigkeiten** (Art. 30): welche Daten, Zweck, Rechtsgrundlage, Empfänger, Löschfristen
2. **TOM** (Art. 32): Zutritts-, Zugangs-, Zugriffskontrolle, Verschlüsselung, Backup
3. **Lösch- und Aufbewahrungskonzept**: Fristen und Verfahren
4. **Prozesse für Betroffenenrechte**: Auskunft, Berichtigung, Löschung, Widerspruch
5. **Datenschutzverletzungen**: Melde- und Notfallprozess (Art. 33/34)
6. **Auftragsverarbeitung**: Liste der Dienstleister und AVV
7. **Datenschutz-Folgenabschätzung** (Art. 35) für Verarbeitungen mit hohem Risiko
8. **Verantwortlichkeiten**: Ansprechpartner, Datenschutzbeauftragter
9. **Schulungs- und Sensibilisierungskonzept** für Beschäftigte

**Weitere Pflichten im Überblick**
- **Privacy by Design / by Default** (Art. 25): Datenschutz schon bei der Planung, datenschutzfreundliche Voreinstellungen
- **Informationspflichten** (Art. 13/14): Betroffene bei Erhebung informieren (Datenschutzerklärung)
- **Rechenschaftspflicht** (Art. 5 Abs. 2): Einhaltung muss **nachgewiesen** werden können
- **Drittlandübermittlung** (Art. 44 ff.): nur mit Angemessenheitsbeschluss oder geeigneten Garantien wie SCC
- **Sanktionen** (Art. 83): Bußgelder bis 20 Mio. € oder 4 % des weltweiten Jahresumsatzes (je nachdem, was höher ist); dazu Schadensersatz (Art. 82)

## Prüfungsfrage

**Frage:** Ein Unternehmen verlagert seine Kundendatenbank zu einem Cloud-Anbieter und erstellt ein Datenschutzkonzept. Nennen Sie vier Inhalte dieses Konzepts.

**Musterantwort:** Erstens das Verzeichnis der Verarbeitungstätigkeiten, das festhält, welche Kundendaten zu welchem Zweck und auf welcher Rechtsgrundlage verarbeitet werden. Zweitens die technischen und organisatorischen Maßnahmen, zum Beispiel Verschlüsselung der Daten in der Cloud, MFA für Administratoren und ein Rollen- und Berechtigungskonzept. Drittens ein Löschkonzept mit festgelegten Aufbewahrungs- und Löschfristen. Viertens den Auftragsverarbeitungsvertrag nach Art. 28 DSGVO mit dem Cloud-Anbieter einschließlich Prüfung des Serverstandorts.

## Falle

- „Einwilligung ist immer nötig“ → falsch, es reicht **eine** passende Rechtsgrundlage aus Art. 6.
- Datenschutzbeauftragten und Aufsichtsbehörde verwechseln – der DSB ist **intern**, die Behörde **extern**.
- DSGVO und BDSG gleichsetzen: die DSGVO gilt unmittelbar EU-weit, das BDSG regelt nur Ergänzungen (z. B. DSB-Pflicht ab 20 Personen).
- Pseudonymisierte Daten als „nicht mehr personenbezogen“ bezeichnen → nur **anonymisierte** Daten fallen aus der DSGVO.

## Merksatz

**„Erst Rechtsgrundlage, dann Grundsätze, dann TOM – und alles dokumentieren.“**

## Verwandt

- [TOM – technische und organisatorische Maßnahmen](toms.md)
- [Rechtsgrundlagen und Grundsätze](rechtsgrundlagen-grundsaetze.md)
- [Betroffenenrechte](betroffenenrechte.md)
- [Auftragsverarbeitung (AVV)](auftragsverarbeitung-avv.md)
- [Anonymisierung und Pseudonymisierung](anonymisierung-pseudonymisierung.md)
- [Datenschutzbeauftragter](dbs-datenschutzbeauftragten.md)
- [Datenpanne und SCC](scc-datenpanne.md)
- [Schutzziele](../../FiSi-Administrator/Sicherheit/schutzziele.md)
- [Lektion 05 Datenschutz und Informationssicherheit](../../analyse-der-exams/05_Datenschutz_Informationssicherheit.md)
