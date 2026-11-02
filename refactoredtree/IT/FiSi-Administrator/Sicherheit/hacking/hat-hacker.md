# White-, Grey- und Black-Hat-Hacker

## In 30 Sekunden

- Unterschieden wird nach **Erlaubnis** und **Absicht** – nicht nach Können oder Werkzeugen.
- **White-Hat** („Ethical Hacker“): handelt **mit Auftrag** des Eigentümers, sucht Schwachstellen, damit sie geschlossen werden (Penetrationstest, Bug-Bounty).
- **Black-Hat**: handelt **ohne Erlaubnis** zum eigenen Vorteil – Datendiebstahl, Erpressung, Sabotage.
- **Grey-Hat**: dringt **ohne Erlaubnis** ein, aber ohne Schädigungsabsicht, und meldet die Lücke – trotzdem **strafbar**.
- Prüfung 2024S GA1: White-Hat und Black-Hat unterscheiden.

## Kernwissen

| | White-Hat | Grey-Hat | Black-Hat |
|---|---|---|---|
| Erlaubnis | **ja**, schriftlicher Auftrag mit Umfang | nein | nein |
| Absicht | Sicherheit verbessern | Lücken aufdecken, evtl. Anerkennung oder Belohnung | Bereicherung, Schaden, Spionage |
| Umgang mit Lücken | vertraulicher Bericht an den Auftraggeber | Meldung an den Betreiber, teils Veröffentlichung | Ausnutzung oder Verkauf |
| Rechtslage | legal | **illegal** (§§ 202a–202c, 303a/b StGB) | illegal |
| Beispiele | Pentester, Sicherheitsberater, Bug-Bounty-Teilnehmer | „Hobby-Forscher“ ohne Auftrag | Ransomware-Gruppen, Datendiebe |

**Weitere Begriffe**
- **Responsible / Coordinated Disclosure:** Lücke erst dem Hersteller melden, Frist zur Behebung geben, dann veröffentlichen.
- **Bug-Bounty-Programm:** Unternehmen erlaubt öffentlich das Testen bestimmter Systeme nach Regeln und zahlt Prämien – macht aus Grey-Hats legale White-Hats.
- **Script-Kiddie:** nutzt fertige Werkzeuge ohne tiefes Verständnis.
- **Hacktivist:** politisch motivierte Angriffe (z. B. DDoS, Defacement).
- **Red Team / Blue Team:** beauftragte Angreifer bzw. Verteidiger bei Sicherheitsübungen.

## Prüfungsfrage

**Frage:** Unterscheiden Sie White-Hat- und Black-Hat-Hacker und erläutern Sie, wann ein Unternehmen einen White-Hat beauftragt.

**Musterantwort:** Ein White-Hat-Hacker dringt mit ausdrücklicher, schriftlicher Erlaubnis des Eigentümers in Systeme ein, um Schwachstellen zu finden und dem Auftraggeber zu melden, damit sie geschlossen werden. Ein Black-Hat-Hacker handelt ohne Erlaubnis, um sich zu bereichern oder Schaden anzurichten, etwa durch Datendiebstahl oder Erpressung. Ein Unternehmen beauftragt einen White-Hat zum Beispiel für einen Penetrationstest vor dem Livegang einer neuen Webanwendung oder regelmäßig zur Überprüfung der Sicherheitsmaßnahmen.

## Falle

- Grey-Hat als „legal, weil gut gemeint“ bezeichnen → ohne Erlaubnis bleibt es strafbar.
- White-Hat über das Werkzeug definieren („nutzt nur legale Tools“) → entscheidend ist der **Auftrag**.

## Merksatz

**„Die Farbe des Huts bestimmt die Erlaubnis, nicht das Werkzeug.“**

## Verwandt

- [Penetrationstest und Schwachstellenmanagement](../penetrationstest-schwachstellenmanagement.md)
- [Angriffsarten im Überblick](hackers-can.md)
- [Sicherheitsvorfall – Incident Response](wurde-ich-gehackt.md)
- [Lektion 05 Datenschutz und Informationssicherheit](../../../analyse-der-exams/05_Datenschutz_Informationssicherheit.md)
