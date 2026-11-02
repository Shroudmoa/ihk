# Patchmanagement (Test – Staging – Produktion)

## In 30 Sekunden

- **Patchmanagement** ist der geregelte Prozess, Updates zu **erfassen, bewerten, testen, verteilen und dokumentieren** – nicht „Update-Knopf drücken“.
- Updates werden stufenweise ausgerollt: **TU** (Testumgebung) → **SU** (Staging-/Abnahmeumgebung) → **PU** (Produktivumgebung), vorher **Backup/Snapshot** und **Rollback-Plan**.
- **Kritikalität** bestimmt das Tempo: aktiv ausgenutzte Sicherheitslücken sofort (ggf. verkürzter Test), Funktionsupdates geplant im Wartungsfenster.
- **Patch/Update vs. Upgrade** (Prüfung 2023W, 2025S): Patch/Update behebt Fehler und Lücken innerhalb der Version, ein Upgrade wechselt auf eine neue Haupt-Version mit neuen Funktionen.
- Zentrale Verteilung: **WSUS**, Microsoft Configuration Manager/Intune, bei Linux Repositories mit Ansible o. ä.

## Kernwissen

**Patch, Update, Upgrade**

| Begriff | Inhalt | Beispiel |
|---|---|---|
| **Patch / Hotfix** | kleine Korrektur einer konkreten Lücke oder eines Fehlers | Sicherheitspatch für eine OpenSSL-Lücke |
| **Update** | Aktualisierung innerhalb der Version, oft gesammelte Patches | monatliches kumulatives Windows-Update, Version 2.4.1 → 2.4.2 |
| **Upgrade** | Wechsel auf eine neue Hauptversion, neue Funktionen, ggf. neue Systemanforderungen und Lizenzkosten | Windows Server 2019 → 2025, PHP 7 → 8 |

**Der Prozess**
1. **Inventar:** Welche Systeme, Betriebssysteme und Softwareversionen gibt es?
2. **Informieren:** Herstellerhinweise, CVE-Meldungen, BSI-Warnungen beobachten
3. **Bewerten:** Schweregrad (CVSS), wird die Lücke aktiv ausgenutzt, wie wichtig ist das System?
4. **Testen:** zuerst in der **Testumgebung (TU)** – meist virtualisiert, mit nachgebildeten wichtigen Systemen; Fehler dort schaden niemandem
5. **Abnehmen:** in der **Staging-Umgebung (SU)**, die der Produktion möglichst genau gleicht (Hardware, Konfiguration, Datenmenge); Schnittstellen, Performance, Pilotgruppe
6. **Ausrollen:** in die **Produktion (PU)** gestaffelt (Rolling Update, Ringe), im **Wartungsfenster**, nach **Change-Freigabe**
7. **Absichern:** vorher **Backup bzw. Snapshot**, dokumentierter **Rollback-Plan**
8. **Prüfen und dokumentieren:** Erfolg kontrollieren (Compliance-Bericht im WSUS/Intune), Ausnahmen begründen

**Priorisierung (Beispiel)**

| Kritikalität | Beispiel | Ziel |
|---|---|---|
| kritisch | aktiv ausgenutzte Lücke in einem aus dem Internet erreichbaren System | sofort bzw. binnen 24–72 h, verkürzter Test |
| hoch | schwere Lücke, intern erreichbar | innerhalb weniger Tage |
| mittel/niedrig | geringes Risiko, Funktionsupdate | im regulären Monatszyklus |

**Warum testen?** Updates können Anwendungen, Treiber oder Schnittstellen beschädigen, Dienste nach dem Neustart nicht mehr starten lassen oder Einstellungen zurücksetzen. **Warum trotzdem zügig?** Nach Veröffentlichung eines Patches analysieren Angreifer ihn und greifen ungepatchte Systeme gezielt an.

**Werkzeuge:** WSUS (Windows Server Update Services, Clients holen Updates vom eigenen Server; Standardports 8530/8531), Microsoft Configuration Manager, Intune/Windows Update for Business, Linux-Paketverwaltung mit eigenen Repository-Spiegeln, Ansible; für Drittanbieter-Software spezielle Patch-Tools.

**Wenn kein Patch möglich ist** (Altsystem, Hersteller-Support beendet): System isolieren (eigenes VLAN, strikte Firewall-Regeln), nicht benötigte Dienste abschalten, verstärkt überwachen, Ablösung planen.

## Prüfungsfrage

**Frage:** Ein Administrator möchte ein sicherheitskritisches Update sofort auf allen 40 Servern installieren. Beschreiben Sie ein strukturiertes Vorgehen und unterscheiden Sie dabei Patch und Upgrade.

**Musterantwort:** Zuerst wird das Update nach Kritikalität bewertet und in einer Testumgebung auf nachgebildeten Systemen installiert, um Inkompatibilitäten zu erkennen. Danach folgt ein Test in der Staging-Umgebung, die der Produktion entspricht, gegebenenfalls mit einer Pilotgruppe. Vor dem Rollout in die Produktion werden Snapshots bzw. Backups erstellt und ein Rollback-Plan festgelegt; dann wird das Update über ein zentrales Werkzeug wie WSUS gestaffelt im Wartungsfenster verteilt, kontrolliert und dokumentiert. Ein Patch behebt eine konkrete Sicherheitslücke oder einen Fehler innerhalb der installierten Version, während ein Upgrade auf eine neue Hauptversion mit neuen Funktionen wechselt.

## Falle

- „Updates immer sofort überall installieren“ → ohne Test drohen Ausfälle; richtig ist **schnell, aber kontrolliert** – bei aktiv ausgenutzten Lücken mit verkürztem Test.
- „Updates erst nach Monaten“ → lässt bekannte Lücken offen.
- Patch und Upgrade verwechseln.
- Backup/Snapshot und Rollback-Plan vergessen.
- Nach dem Update nicht prüfen, ob Härtungseinstellungen noch greifen (z. B. reaktivierte Altprotokolle).

## Merksatz

**„Bewerten – testen – stufenweise ausrollen – zurückrollen können – dokumentieren.“**

## Verwandt

- [Penetrationstest und Schwachstellenmanagement](../Sicherheit/penetrationstest-schwachstellenmanagement.md)
- [OS-Härtung](../Server/os-haertung.md)
- [Snapshots und Backups](../Server/virtualisierung/snapshots-backups.md)
- [Blue-Green-Deployment](../Server/blue-green-deployment.md)
- [GPO](gpo.md) – Update-Einstellungen zentral setzen
- [MDM und BYOD](mdm-byod.md) – Updates auf mobilen Geräten
- [Lektion 28 GA1-Praxisthemen (Patchmanagement)](../../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md)
