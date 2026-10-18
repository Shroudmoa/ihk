# Lektion 12 – Backup-Strategien, MTBF & Speicher

**Bereich:** GA1 (Konzeption und Administration von IT-Systemen)
**Relevanz:** hoch – Themen wie Backup, Verfügbarkeit und Ausfallsicherheit sind Dauerbrenner in GA1 und GA2
**Lernzeit:** ca. 2h

---

## 1. Warum das wichtig ist

Ein funktionierendes Backup ist die letzte Verteidigungslinie gegen Datenverlust (Ransomware, Hardware-Defekt, Bedienfehler). In der AP2 wird erwartet, dass du Backup-Arten vergleichst, Restore-Aufwände abschätzt und Kennzahlen wie MTBF einordnen kannst. Typische Szenario-Frage: "Welche Strategie minimiert Speicherplatz bei vertretbarem Restore-Aufwand?"

> ✅ **Verifiziert am echten Prüfungskorpus:** Backup ist ein Dauerbrenner-Thema.
> Belegt sind konkret: Backup-Arten mit Archivbit-Logik (**2022S, 2022W**),
> Band-Rotation (**2022W, 2023S, 2023W, 2025W**), GVS/GFS-Rotation mit
> konkretem Rechenbeispiel (**2025W**), 3-2-1-Regel explizit (**2025S**),
> RTO/RPO mit SLA-Rechenbeispiel (**2023S, 2026S**).

## 2. Grundlagen – Backup-Arten

| Art | Was wird gesichert | Backup-Dauer | Speicherbedarf | Restore-Aufwand |
|---|---|---|---|---|
| **Vollständig (Full)** | alle Daten | langsam | hoch | gering (1 Medium) |
| **Differenziell** | alles, was sich **seit dem letzten Voll-Backup** geändert hat | mittel (wächst mit der Zeit) | mittel | Voll + letztes Diff |
| **Inkrementell** | alles, was sich **seit dem letzten beliebigen Backup** geändert hat | schnell | gering | Voll + **alle** inkrementellen Bänder in korrekter Reihenfolge |

⚠️ **Restore-Kette merken:**
- Differenziell: benötigt **Voll + 1 Diff**
- Inkrementell: benötigt **Voll + KOMPLETTE Kette** (jeder einzelne Schritt!) → bei einem fehlenden Zwischenband ist der Restore unvollständig.

### Archivbit-Logik (2022S, 2022W)

Jede Datei besitzt ein **Archivbit**, das anzeigt, ob sie seit dem letzten
Backup verändert wurde:
- **Voll-Backup:** sichert alle Dateien, **setzt danach alle Archivbits zurück**
- **Differenziell:** sichert alle Dateien mit gesetztem Archivbit,
  **verändert das Archivbit NICHT** → beim nächsten Diff werden dieselben
  Änderungen wieder mitgesichert (deshalb wächst es)
- **Inkrementell:** sichert alle Dateien mit gesetztem Archivbit,
  **setzt das Archivbit danach zurück** → beim nächsten Inkrementell werden
  nur die seither neu geänderten Dateien gesichert

### Band-Rotation (2022W, 2023S, 2023W, 2025W)

Mehrfach geprüftes Thema: Bandmedien werden nach einem festen Rotationsschema
wiederverwendet (z. B. tägliche Bänder für Mo–Fr, wöchentliche Bänder für
die GVS-Hierarchie), um mit einer begrenzten Anzahl an Medien einen möglichst
langen historischen Wiederherstellungszeitraum abzudecken, ohne ständig neue
Bänder kaufen zu müssen.

## 3. Großvater-Vater-Sohn (GVS / GFS)

Rotationstierarchie für Backup-Medien:

- **Söhne:** tägliches Backup (z. B. Mo–Fr)
- **Vater:** wöchentliches Backup (das letzte des Monats wird zum Vater)
- **Großvater:** monatliches Backup (langfristige Aufbewahrung, z. B. ein Jahr)

Vorteil: gute Feinkörnigkeit (täglich) bei vertretbarem Medienbedarf; historische Wiederherstellung über Wochen/Monate möglich.

**Konkretes Prüfungsbeispiel (2025W):** Die Bänder wurden durchnummeriert und
den GVS-Rollen zugeordnet: **Vollbackup B22 = Sohn**, **Vater = B15**,
**Großvater = B08**. Merke: die Nummerierung zeigt, dass ältere Bänder
(niedrigere Nummer) den übergeordneten Rollen (Vater/Großvater) zugeordnet
werden, während das aktuelle/letzte Band der Sohn-Rolle entspricht.

## 4. 3-2-1-Regel & D2D2T

**3-2-1-Regel** (Goldstandard, **explizit geprüft 2025S**):
- **3** Kopien der Daten (1 Produktiv + 2 Backups)
- **2** verschiedene Speichermedien (z. B. Disk + Tape)
- **1** Kopie extern / Offsite (räumlich getrennt, gegen Brand/Diebstahl)

**D2D2T** (Disk-to-Disk-to-Tape): schnelles lokales Restore von Disk, langfristige/Offline-Kopie auf Tape (robust gegen Ransomware).

## 4b. RTO/RPO & SLA-Rechenbeispiel (2023S, 2026S)

- **RTO (Recovery Time Objective):** maximal tolerierbare **Zeit bis zur
  Wiederherstellung** eines Systems nach einem Ausfall ("Wie lange darf der
  Ausfall dauern?").
- **RPO (Recovery Point Objective):** maximal tolerierbarer **Datenverlust**
  in Zeit gemessen, also seit dem letzten Backup ("Wie viel darf verloren
  gehen?"). Bestimmt direkt das nötige Backup-Intervall.

**SLA-Rechenbeispiel** (Verfügbarkeit → maximale Ausfallzeit → Verlust):
- SLA-Zusage: **99,9 %** Verfügbarkeit pro Jahr
- Maximale Ausfallzeit pro Jahr: `(1 - 0,999) × 365 Tage × 24 h ≈ 8,76 h`
- Wird diese Ausfallzeit voll ausgeschöpft und entsteht dabei ein
  Umsatzverlust von z. B. 500 €/h, ergibt sich: `8,76 h × 500 €/h = 4.380 €`
  Verlust pro Jahr bei Ausschöpfung des SLA.

## 5. MTBF vs MTTF & USV

| Kennzahl | Bedeutung | Anwendung |
|---|---|---|
| **MTBF** (Mean Time Between Failures) | mittlere Zeit **zwischen** Ausfällen | für **reparierbare** Systeme (Bauteil wird getauscht, läuft weiter) |
| **MTTF** (Mean Time To Failure) | mittlere Zeit **bis zum** Ausfall | für **nicht reparierbare** Systeme (z. B. SSD-Lebensdauer) |

Parallel geschaltete Komponenten: die Gesamt-MTBF **sinkt** (mehr Teile = mehr potenzielle Fehlerquellen). Näherung: `1 / λ_ges = 1/MTBF_1 + 1/MTBF_2 + …` (kurz wissen, keine tiefe Rechnung nötig).

**USV (Unterbrechungsfreie Stromversorgung):** überbrückt kurzzeitigen Stromausfall und gibt dem System Zeit für ein **geordnetes Herunterfahren** – verhindert Datenkorruption, ist aber kein Ersatz für langen Inselbetrieb (dafür Notstromaggregat).

## 6. Übungsaufgabe (im Prüfungsstil, selbst erstellt)

> Eine Firma sichert jeden Sonntag ein Voll-Backup (V) und an den Werktagen Mo–Sa ein inkrementelles Backup (I). Am Donnerstagmorgen tritt ein Datenverlust auf.
>
> a) Welche Medien werden für den Restore am Donnerstag benötigt?
> b) Wie viele Backups müssen mindestens eingespielt werden (Voll + wie viele Inkrementelle)?
> c) Angenommen: Voll-Backup = 100 GB, jedes tägliche Inkrementell = 5 GB. Wie viel Speicherplatz verbraucht die Kette Sonntag bis Donnerstag für die Backups selbst?

<details>
<summary>Lösung (aufklappen)</summary>

a) Das **Voll-Backup von Sonntag** plus **alle** inkrementellen Backups Mo, Di, Mi, Do (die Kette muss lückenlos sein).

b) **1 Voll + 4 Inkrementelle = 5 Backups** müssen in korrekter Reihenfolge eingespielt werden.

c) 100 GB (Voll) + 4 × 5 GB (Mo–Do) = 100 + 20 = **120 GB**.

Zum Vergleich differenziell (Di–Do jeweils seit Sonntag): Restore bräuchte nur Voll + 1 Diff, aber das tägliche Diff-Wachstum wäre größer.

</details>

## 7. Selbstcheck – kannst du das?

- [ ] Ich kann Voll-, Differenziell- und Inkrementell-Backup mit Restore-Aufwand vergleichen
- [ ] Ich kann die GVS-Hierarchie (Söhne/Vater/Großvater) erklären
- [ ] Ich kenne die 3-2-1-Regel und den Sinn von D2D2T
- [ ] Ich kann MTBF von MTTF unterscheiden (reparierbar vs. nicht reparierbar)
- [ ] Ich kann erklären, wozu eine USV dient

## 8. Weiterführende Ressourcen

- **Wikipedia: Datensicherung** – übersichtliche Darstellung der Backup-Verfahren und GVS
- **Veeam / Veeam Backup & Replication** – praxisnahe Erklärvideos zu 3-2-1 und D2D2T
- **Heise / c't**: Artikel zu USV-Auswahl und MTBF-Kennzahlen
- Übe mit den GA1-Musterlösungen in deinem Repo (Stichwort "Verfügbarkeit", "Backup-Konzept")

---
*Nächste Lektion: Virtualisierung (Hypervisor & Container)*
