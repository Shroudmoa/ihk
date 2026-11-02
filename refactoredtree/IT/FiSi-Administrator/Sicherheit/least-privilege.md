# Least-Privilege-Prinzip

## In 30 Sekunden

- **Least Privilege** (Prinzip der minimalen Rechte): Jeder Benutzer, Dienst und Prozess erhält **nur die Rechte, die er für seine Aufgabe zwingend braucht** – nicht mehr und nicht länger als nötig.
- Ziel: **Angriffsfläche und Schadensausmaß begrenzen** – ein kompromittiertes Konto kann nur so viel anrichten, wie es darf.
- Verwandt: **Need-to-know** (nur die Informationen, die man für die Aufgabe kennen muss) und **Funktionstrennung** (kritische Schritte auf mehrere Personen verteilen).
- Umsetzung: **Rollen- und Gruppenkonzept (RBAC)**, getrennte Admin-Konten, UAC/sudo, Dienstkonten mit minimalen Rechten, regelmäßige Rechteprüfung.
- Prüfung 2025S GA1 (Abgrenzung zu Zero Trust), 2022S/2025S (OS-Härtung), 2021W (Maßnahmen gegen unbefugten DB-Zugriff).

## Kernwissen

**Maßnahmen**

| Bereich | Umsetzung |
|---|---|
| Benutzerrechte | Rechte über **Gruppen/Rollen** statt einzeln vergeben (AGDLP im AD), Standardbenutzer ohne lokale Adminrechte |
| Administratoren | **getrenntes Admin-Konto** nur für Verwaltung, normales Konto für Mail und Surfen; Tier-Modell (Domänen-, Server-, Client-Admins getrennt) |
| Zeitliche Begrenzung | **Just-in-Time-Rechte** (Adminrechte nur auf Antrag für begrenzte Zeit), Privileged Access Management |
| Lokale Admins | **LAPS**: eindeutiges, rotierendes Passwort je Gerät |
| Dienste | Dienstkonten bzw. gMSA mit minimalen Rechten, Webserver nicht als root/SYSTEM |
| Anwendungen | Application Whitelisting (AppLocker/WDAC), Datenbanknutzer mit nur SELECT-Rechten, wo nur gelesen wird |
| Dateisystem | NTFS-/Linux-Rechte so eng wie möglich, keine „Jeder: Vollzugriff“-Freigaben |
| Kontrolle | **Rezertifizierung** (regelmäßige Überprüfung der Rechte), Rechte beim Abteilungswechsel entziehen, Konten beim Austritt sperren |

**Vorteile:** begrenzter Schaden bei Malware oder Kontoübernahme (Ransomware verschlüsselt nur, worauf das Konto schreiben darf), weniger Fehlbedienungen, bessere Nachvollziehbarkeit, erfüllt Datenschutzgrundsätze (Art. 5 und Art. 25 DSGVO: Datenminimierung, Privacy by Default).

**Nachteile/Herausforderungen:** Verwaltungsaufwand, Gefahr der „Rechte-Ansammlung“ über die Zeit (Privilege Creep), Nutzer fordern ungeduldig Ausnahmen.

**Abgrenzung**
- **Least Privilege:** Wie **viel** darf jemand? (Umfang)
- **Need-to-know:** Welche **Informationen** braucht jemand?
- **Zero Trust:** Wird **jeder einzelne Zugriff** geprüft? (Least Privilege ist ein Baustein davon)

## Prüfungsfrage

**Frage:** Erläutern Sie das Least-Privilege-Prinzip und nennen Sie zwei Maßnahmen, mit denen es auf einem Windows-Fileserver umgesetzt wird.

**Musterantwort:** Nach dem Least-Privilege-Prinzip erhält jeder Benutzer und jeder Prozess nur die Rechte, die er für seine konkrete Aufgabe zwingend benötigt; dadurch wird der Schaden durch Fehlbedienung, Schadsoftware oder ein kompromittiertes Konto begrenzt. Auf dem Fileserver werden erstens die NTFS- und Freigabeberechtigungen über AD-Gruppen je Abteilung vergeben, sodass etwa die Buchhaltung nur auf ihren eigenen Ordner schreiben darf. Zweitens arbeiten Administratoren im Alltag mit einem normalen Benutzerkonto und verwenden ein getrenntes Admin-Konto nur für Verwaltungsaufgaben.

## Falle

- „Admins brauchen immer Vollzugriff“ → gerade Admin-Konten sind das wertvollste Angriffsziel.
- Rechte direkt an einzelne Benutzer vergeben → unübersichtlich, wird beim Abteilungswechsel vergessen.
- Least Privilege mit Datenminimierung gleichsetzen: das eine begrenzt **Rechte**, das andere die **gesammelten Daten** – beide hängen aber zusammen.

## Merksatz

**„So viel wie nötig, so wenig wie möglich – und nur so lange wie nötig.“**

## Verwandt

- [Zero Trust](zero-trust.md)
- [RBAC](../Netzwerk-Administration/rbac.md) – Rollenkonzept
- [Zugriffskontrollmodelle](../../nice-to-know/access-control.md) – DAC, MAC, RBAC, ACL
- [GPO](../Netzwerk-Administration/gpo.md)
- [OS-Härtung](../Server/os-haertung.md)
- [chmod](../chmod.md) – Linux-Rechte
- [Lektion 28 GA1-Praxisthemen (Least Privilege und Zero Trust)](../../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md)
