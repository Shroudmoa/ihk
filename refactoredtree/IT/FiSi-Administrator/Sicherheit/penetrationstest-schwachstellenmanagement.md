# Penetrationstest und Schwachstellenmanagement

## In 30 Sekunden

- Ein **Penetrationstest** ist ein **beauftragter**, kontrollierter Angriff auf eigene Systeme, um ausnutzbare Schwachstellen zu finden, bevor Kriminelle es tun.
- Ohne **schriftliche Beauftragung** mit festgelegtem Umfang wäre derselbe Test strafbar (§§ 202a ff. StGB).
- Testarten: **Black-Box** (ohne Vorwissen), **White-Box** (mit Dokumentation, Zugangsdaten, Quellcode), **Grey-Box** (Teilwissen).
- Ein **Schwachstellenscan** findet bekannte Lücken automatisiert und regelmäßig; der Pentest geht tiefer und prüft die **tatsächliche Ausnutzbarkeit**.
- Prüfung 2025S: „zusätzliche sicherheitsrelevante Tests“ nennen – Pentest, Schwachstellenscan, Phishing-Simulation, Wiederherstellungstest.

## Kernwissen

**Testarten**

| Art | Vorwissen des Testers | Simuliert | Vorteil | Nachteil |
|---|---|---|---|---|
| **Black-Box** | keins | externen Angreifer | realistisch | zeitaufwendig, findet evtl. nicht alles |
| **White-Box** | vollständig (Netzplan, Konfiguration, Code) | Innentäter, gründliche Prüfung | sehr gründlich, effizient | weniger realistisch |
| **Grey-Box** | teilweise (z. B. normales Benutzerkonto) | Angreifer mit gestohlenen Zugangsdaten | guter Kompromiss | – |

**Ablauf (angelehnt an das BSI-Praxis-Modell)**
1. **Vorbereitung:** Ziele, Umfang (Scope), Zeitfenster, Ansprechpartner und Notfallabbruch **schriftlich** vereinbaren
2. **Informationsbeschaffung:** öffentlich verfügbare Informationen, erreichbare Systeme und Dienste erfassen
3. **Bewertung:** gefundene Schwachstellen analysieren, Angriffswege priorisieren
4. **Aktive Eindringversuche:** kontrollierte Ausnutzung, ohne Produktivdaten zu beschädigen
5. **Abschlussbericht:** Befunde mit Risikobewertung und konkreten Gegenmaßnahmen

**Weitere sicherheitsrelevante Tests**

| Test | Zweck |
|---|---|
| Automatisierter Schwachstellenscan | bekannte Lücken und Fehlkonfigurationen regelmäßig finden |
| Phishing-Simulation / Social-Engineering-Test | Awareness der Mitarbeiter messen, Schulungsbedarf erkennen |
| Red-Team-Übung | realistischer, längerer Angriff über viele Wege; prüft auch Erkennung und Reaktion |
| Wiederherstellungstest (Restore-Test) | funktioniert das Backup im Ernstfall wirklich? |
| Code-Review / Sicherheitstests in der Entwicklung | Lücken vor dem Livegang finden |
| Konfigurations-Audit | Abgleich mit Härtungsvorgaben (z. B. CIS-Benchmarks, BSI-Bausteine) |

**Schwachstellenmanagement als Kreislauf**
1. **Inventarisieren** – was haben wir überhaupt? (Assets, Softwareversionen)
2. **Erkennen** – Scans, Herstellerwarnungen, CVE-Meldungen, BSI-Warnungen
3. **Bewerten** – Schweregrad nach **CVSS** (0–10) plus Bedeutung des betroffenen Systems
4. **Beheben** – Patch, Konfigurationsänderung oder Workaround (über das Patchmanagement)
5. **Prüfen** – erneuter Scan, Dokumentation, und wieder von vorn

Begriffe: **CVE** = eindeutige Kennung einer bekannten Schwachstelle; **CVSS** = Bewertungsschema für ihren Schweregrad; **Zero-Day** = Lücke, für die es noch keinen Patch gibt; **Exploit** = Programm oder Methode, die eine Lücke ausnutzt.

## Prüfungsfrage

**Frage:** Ein Unternehmen lässt seine neue Webanwendung vor dem Livegang testen. Nennen Sie zwei sicherheitsrelevante Tests und unterscheiden Sie Black-Box- und White-Box-Penetrationstest.

**Musterantwort:** Sinnvoll sind ein Penetrationstest, bei dem ein beauftragter Dienstleister die Anwendung kontrolliert angreift, und ein automatisierter Schwachstellenscan, der bekannte Lücken und Fehlkonfigurationen aufdeckt. Beim Black-Box-Test hat der Tester kein Vorwissen und geht wie ein externer Angreifer vor; das ist realistisch, kann aber Lücken übersehen. Beim White-Box-Test erhält er Dokumentation, Zugangsdaten und gegebenenfalls Quellcode; dadurch prüft er gründlicher und effizienter, bildet aber weniger realistisch einen Außentäter ab.

## Falle

- Pentest ohne schriftliche Beauftragung und Scope → rechtlich ein Angriff.
- Schwachstellenscan und Pentest gleichsetzen: der Scan **meldet**, der Pentest **beweist** die Ausnutzbarkeit.
- Den Bericht ohne Maßnahmenplan abheften – ein Test ohne Behebung verbessert nichts.
- Black-Box/White-Box hier mit Softwaretest verwechseln: das Prinzip (mit/ohne Kenntnis des Inneren) ist dasselbe, der Zweck hier ist Sicherheit.

## Merksatz

**„Erst Auftrag, dann Angriff – und ohne Behebung war alles umsonst.“**

## Verwandt

- [White-, Grey- und Black-Hat](hacking/hat-hacker.md)
- [Patchmanagement](../Netzwerk-Administration/patchmanagement-tu-su-pu.md) – Schwachstellen beheben
- [ISMS und IT-Grundschutz](isms-bsi-grundschutz.md)
- [Angriffsarten](hacking/hackers-can.md)
- [OS-Härtung](../Server/os-haertung.md)
- [Lektion 28 GA1-Praxisthemen (White-/Black-Box)](../../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md)
