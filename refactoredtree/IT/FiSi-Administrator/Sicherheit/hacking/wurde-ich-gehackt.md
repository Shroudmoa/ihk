# Sicherheitsvorfall – Incident Response („Wurde ich gehackt?“)

## In 30 Sekunden

- Ein **Sicherheitsvorfall** ist ein Ereignis, das Vertraulichkeit, Integrität oder Verfügbarkeit verletzt oder bedroht (Malware, Kontoübernahme, Datenabfluss, Ausfall).
- Ablauf nach bewährtem Schema: **Vorbereiten → Erkennen & Melden → Eindämmen → Beseitigen → Wiederherstellen → Nachbereiten**.
- Erste Regel: betroffenes System **vom Netz trennen, aber nicht ausschalten** – flüchtige Spuren im RAM bleiben erhalten; nur bei laufender Verschlüsselung ist sofortiges Abschalten abzuwägen.
- **Beweise sichern**, bevor etwas verändert wird (Logs, RAM-Abbild, Festplatten-Image), alles mit Zeitstempel dokumentieren.
- Rechtliches parallel prüfen: **DSGVO-Meldung binnen 72 h** an die Aufsichtsbehörde bei Risiko für Betroffene (Art. 33), Information der Betroffenen bei hohem Risiko (Art. 34), ggf. Meldung an das BSI (KRITIS/NIS2) und Strafanzeige.

## Kernwissen

**1. Vorbereitung** – bevor etwas passiert
- Notfallplan mit Meldewegen, Zuständigkeiten, Telefonliste (auch offline ausgedruckt)
- Logging und Zeitsynchronisation (NTP), zentrale Auswertung (SIEM)
- getestete Offline-Backups, Wiederanlaufpläne, Übungen

**2. Erkennen und Melden** – Indikatoren einer Kompromittierung
- ungewöhnliche CPU-/RAM-Last, Abstürze, unbekannte Prozesse oder Dienste
- neue Konten oder Adminrechte, Anmeldungen zu ungewöhnlichen Zeiten/Orten, viele Fehlversuche
- ungewöhnlicher ausgehender Datenverkehr (Datenabfluss, Command-and-Control-Server)
- verschlüsselte Dateien, Erpresserschreiben, deaktivierter Virenschutz
- Alarme von Virenschutz/EDR, IDS/IPS oder SIEM, Hinweise von außen (Kunden, Behörden)

Mitarbeiter melden Verdachtsfälle **sofort** über einen festgelegten Weg (Helpdesk, Security-Team) – ohne Angst vor Sanktionen.

**3. Bewerten** – Schweregrad und Priorität
- Welche Systeme und Daten sind betroffen (Einzel-PC, Server, Domäne)?
- Sind **personenbezogene Daten** oder Geschäftsgeheimnisse betroffen?
- Wer muss informiert werden: IT-Leitung, Geschäftsführung, ISB, **Datenschutzbeauftragter**, ggf. externe Forensiker, Versicherung?

**4. Eindämmen**
- System vom Netz trennen (Kabel ziehen, WLAN aus, Switch-Port sperren) bzw. in ein Quarantäne-VLAN verschieben
- kompromittierte Konten sperren, Passwörter zurücksetzen, Sitzungen und Tokens ungültig machen
- Ausbreitung stoppen: Firewall-Regeln, Segmente trennen
- Beweise sichern: Logs exportieren, RAM-Abbild, Festplatten-Image, Chain of Custody dokumentieren

**5. Beseitigen und Wiederherstellen**
- Ursache finden und schließen (Patch, Konfiguration, gestohlene Zugangsdaten)
- kompromittierte Systeme **neu aufsetzen** statt „sauber zu putzen“
- Daten aus einem **sauberen Backup** vor dem Infektionszeitpunkt zurückspielen
- verstärkt überwachen, ob der Angreifer zurückkommt

**6. Nachbereiten (Lessons Learned)**
- Abschlussbericht: Zeitlinie, Ursache, Schaden, Maßnahmen
- Notfallplan, TOM und Schulungen anpassen

**Meldepflichten im Überblick**

| Adressat | Wann | Frist |
|---|---|---|
| Aufsichtsbehörde (Art. 33 DSGVO) | Verletzung personenbezogener Daten mit Risiko für Betroffene | **72 Stunden** nach Bekanntwerden |
| Betroffene (Art. 34 DSGVO) | voraussichtlich **hohes** Risiko | unverzüglich |
| BSI | KRITIS-Betreiber bzw. Einrichtungen nach NIS2 | NIS2: Frühwarnung 24 h, Meldung 72 h |
| intern | jede Datenschutzverletzung dokumentieren (Art. 33 Abs. 5) | immer |

## Prüfungsfrage

**Frage:** Ein Mitarbeiter meldet, dass auf seinem PC plötzlich Dateien mit unbekannter Endung auftauchen und ein Erpresserschreiben angezeigt wird. Beschreiben Sie vier Sofortmaßnahmen in sinnvoller Reihenfolge.

**Musterantwort:** Zuerst wird der PC sofort vom Netzwerk getrennt, damit sich die Ransomware nicht auf Netzlaufwerke und andere Systeme ausbreitet. Danach wird das Security-Team bzw. die IT-Leitung informiert und der Datenschutzbeauftragte eingebunden, weil personenbezogene Daten betroffen sein können; dieser prüft die Meldung an die Aufsichtsbehörde innerhalb von 72 Stunden. Anschließend werden Beweise gesichert und das betroffene Benutzerkonto gesperrt sowie dessen Passwort zurückgesetzt. Schließlich wird das System neu aufgesetzt und die Daten werden aus einem sauberen, vor der Infektion erstellten Backup wiederhergestellt.

## Falle

- „Rechner sofort herunterfahren“ als erste Maßnahme → RAM-Spuren gehen verloren; erst **isolieren**.
- „Virus entfernen und weiterarbeiten“ → bei Kompromittierung **neu aufsetzen**, Hintertüren bleiben sonst unentdeckt.
- 72-Stunden-Frist für die Information der **Betroffenen** angeben → die gilt für die **Behörde**; Betroffene werden bei hohem Risiko unverzüglich informiert.
- Aus einem Backup wiederherstellen, das die Schadsoftware schon enthält – Infektionszeitpunkt prüfen.

## Merksatz

**„Trennen statt ausschalten, sichern statt löschen, neu aufsetzen statt putzen – und die 72 Stunden im Blick.“**

## Verwandt

- [Malware und Ransomware](malware-ransomware.md)
- [Datenpanne und SCC](../../../Management/DSGVO/scc-datenpanne.md) – Meldepflichten nach Art. 33/34
- [AAA und SIEM](../aaa-siem.md) – Vorfälle erkennen
- [IDS und IPS](../firewall/ids-ips.md)
- [ISMS und IT-Grundschutz](../isms-bsi-grundschutz.md) – Notfallmanagement (BSI 200-4)
- [Snapshots und Backups](../../Server/virtualisierung/snapshots-backups.md)
- [Lektion 05 Datenschutz und Informationssicherheit](../../../analyse-der-exams/05_Datenschutz_Informationssicherheit.md)
