# DSGVO & TOM

## In 30 Sekunden

- Personenbezogene Daten sind alles, was sich auf eine Person beziehen lässt – auch IP-Adresse und Personalnummer.
- Jede Verarbeitung braucht eine **Rechtsgrundlage** nach Art. 6 DSGVO.
- **TOM** (Art. 32) sind technische und organisatorische Maßnahmen – die Prüfung will konkrete Beispiele, keine Floskeln.
- Betroffenenrechte: Auskunft, Berichtigung, Löschung, Einschränkung, Datenübertragbarkeit, Widerspruch.
- Datenschutzverletzung mit Risiko: Meldung an die Aufsichtsbehörde **innerhalb von 72 Stunden**.
- Ziele sind freier Datenverkehr innerhalb der europaischen Binnenmakrts und Schutzp Personenbezogener Daten
 
## Kernwissen

**Rechtsgrundlagen nach Art. 6**
| Grundlage | Wann | Beispiel |
|---|---|---|
| Einwilligung (lit. a) | freiwillig, informiert, widerrufbar | Newsletter |
| Vertrag (lit. b) | für die Vertragserfüllung nötig | Lieferadresse |
| Rechtliche Pflicht (lit. c) | Gesetz verlangt es | Rechnungsaufbewahrung |
| Berechtigtes Interesse (lit. f) | Abwägung nötig | Videoüberwachung |

**Die TOM, die in der Prüfung zählen**
| Maßnahme | Konkret |
|---|---|
| Zutrittskontrolle | Wer kommt ins Rechenzentrum? Karte, Protokoll |
| Zugangskontrolle | Wer darf sich anmelden? Passwort, MFA |
| Zugriffskontrolle | Wer darf welche Daten? Rollen, ACL |
| Weitergabekontrolle | Was verlässt das Netz? TLS, VPN |
| Eingabekontrolle | Wer hat was geändert? Logs, Vier-Augen-Prinzip |
| Auftragskontrolle | Externer Dienstleister? AVV nach Art. 28 |
| Verfügbarkeitskontrolle | Backup, RAID, USV |
| Trennungskontrolle | Mandanten, Test und Produktion getrennt |

**Rollen**
- **Verantwortlicher:** entscheidet über Zweck und Mittel der Verarbeitung.
- **Auftragsverarbeiter:** verarbeitet weisungsgebunden, braucht einen AVV.
- **Datenschutzbeauftragter:** berät und überwacht intern, ist nicht die Aufsichtsbehörde.

**Wichtig:** Eine Einwilligung ist nicht immer nötig. Es muss nur eine passende Rechtsgrundlage geben.

## Prüfungsfrage

**Frage:** Ein Dienstleister soll für ein Unternehmen E-Mails archivieren. Nenne zwei DSGVO-Pflichten und je eine passende TOM.

**Musterantwort:** Es braucht einen Auftragsverarbeitungsvertrag nach Art. 28 und eine Rechtsgrundlage für die Verarbeitung. Technisch: Verschlüsselung bei Übertragung und Speicherung als Weitergabekontrolle. Organisatorisch: Rollenkonzept mit minimalen Rechten und Protokollierung als Zugriffskontrolle.

## Falle

- „Einwilligung ist immer nötig“ → falsch, eine andere Rechtsgrundlage reicht.
- TOM nur mit „Firewall“ beantworten → organisatorische Maßnahmen fehlen, das gibt Punktabzug.
- Datenschutzbeauftragter und Aufsichtsbehörde verwechseln.
- DSGVO und BDSG gleichsetzen: die DSGVO gilt EU-weit, das BDSG ergänzt national.

## Merksatz

**„Rechtsgrundlage zuerst, TOM danach.“** Und die acht Kontrollen: **Zutritt, Zugang, Zugriff, Weitergabe, Eingabe, Auftrag, Verfügbarkeit, Trennung.**

## Verwandt

- [Schutzziele](schutzziele.md) – Vertraulichkeit, Integrität, Verfügbarkeit
- [hackers-can](hackers-can.md) – typische Angriffe auf diese Schutzziele
- [Symmetrische-und-Asymmetrische](symmetrische-und-asymmetrische.md) – Verschlüsselung als TOM
