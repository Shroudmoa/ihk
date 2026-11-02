# NTFS- und Freigabeberechtigungen

## In 30 Sekunden

- Windows hat **zwei Ebenen**: **Freigabeberechtigungen** (gelten nur beim Zugriff **über das Netzwerk**, SMB) und **NTFS-Berechtigungen** (gelten **immer**, auch lokal).
- Innerhalb einer Ebene **addieren** sich die Rechte aller Gruppen eines Benutzers. **Verweigern** schlägt Zulassen.
- Zwischen den Ebenen gilt das **restriktivere** Ergebnis: effektiv = Minimum aus (Freigabe, NTFS).
- Rechte werden von oben nach unten **vererbt**. Vererbung kann man unterbrechen, das macht die Verwaltung aber unübersichtlich.
- Rechte an **Gruppen** vergeben, nicht an Einzelpersonen (AGDLP). Freigabe großzügig, Feinsteuerung über NTFS.

## Kernwissen

### Die Rechte

| NTFS-Standardrecht | erlaubt |
|---|---|
| **Vollzugriff** | alles, inkl. Berechtigungen ändern und Besitz übernehmen |
| **Ändern** | Lesen, Schreiben, Ausführen, **Löschen** |
| **Lesen, Ausführen** | Dateien öffnen und Programme starten |
| **Ordnerinhalt anzeigen** | Ordner durchsuchen (nur Ordner) |
| **Lesen** | Inhalte und Attribute lesen |
| **Schreiben** | neue Dateien anlegen, Inhalte ändern (ohne Löschen) |

| Freigaberecht | erlaubt |
|---|---|
| **Lesen** | Dateien anzeigen und öffnen |
| **Ändern** | zusätzlich schreiben und löschen |
| **Vollzugriff** | zusätzlich Freigaberechte ändern |

### Auswertungsregeln

1. **Pro Ebene kumulieren:** Rechte aller Gruppen addieren. „Vertrieb: Lesen“ + „Projekt-X: Ändern“ → Ändern.
2. **Verweigern gewinnt:** Ein explizites Verweigern hebt Zulassen auf. Ausnahme: Ein **explizites** Zulassen am Objekt schlägt ein nur **vererbtes** Verweigern.
3. **Ebenen kombinieren:** Beim Netzwerkzugriff gilt das restriktivere Ergebnis aus Freigabe und NTFS. Lokal (am Server angemeldet, per RDP) gilt nur NTFS.

**Rechenbeispiel:** Anna ist in den Gruppen *Mitarbeiter* und *Buchhaltung*.

| Ebene | Mitarbeiter | Buchhaltung | Ergebnis der Ebene |
|---|---|---|---|
| Freigabe `\\srv\finanzen` | Lesen | Ändern | **Ändern** |
| NTFS Ordner `D:\Finanzen` | Lesen | Lesen, Ausführen | **Lesen, Ausführen** |
| **Effektiv über das Netzwerk** | | | **Lesen, Ausführen** (restriktiver) |

Soll Anna schreiben, muss die NTFS-Berechtigung der Buchhaltung auf Ändern erhöht werden.

### Vererbung und Kopieren/Verschieben

| Aktion | NTFS-Rechte danach |
|---|---|
| **Kopieren** (auch im selben Volume) | erbt die Rechte des **Zielordners** |
| **Verschieben im selben Volume** | behält die **ursprünglichen** expliziten Rechte |
| **Verschieben auf ein anderes Volume** | erbt die Rechte des **Zielordners** (ist technisch Kopieren + Löschen) |
| auf **FAT32/exFAT** | alle NTFS-Rechte gehen verloren (kein ACL-Konzept) |

### Best Practice: AGDLP

**A**ccounts → **G**lobale Gruppen (nach Rolle, z. B. „GG_Buchhaltung“) → **D**omänen**l**okale Gruppen (nach Ressource und Recht, z. B. „DL_Finanzen_Ändern“) → **P**ermissions auf dem Ordner.
Vorteil: Ein neuer Mitarbeiter wird nur in die Rollengruppe aufgenommen. Die Ordnerrechte bleiben unverändert.

### Vergleich mit Linux

| | Windows NTFS | Linux (ext4) |
|---|---|---|
| Modell | ACL mit beliebig vielen Einträgen | Besitzer / Gruppe / andere (`rwx`), optional POSIX-ACL |
| Befehl | `icacls`, Eigenschaften → Sicherheit | `chmod`, `chown`, `setfacl` |
| Netzwerk | SMB-Freigabe mit eigenen Rechten | Samba/NFS, Rechte über Dateisystem und Export |

## Prüfungsfrage

**Frage:** Ein Mitarbeiter hat auf der Freigabe „Projekte“ das Freigaberecht „Ändern“ und auf dem Ordner das NTFS-Recht „Lesen“. Welche Rechte hat er über das Netzwerk, welche bei lokaler Anmeldung am Server? Begründen Sie.

**Musterantwort:** Über das Netzwerk werden Freigabe- und NTFS-Berechtigungen kombiniert, und es gilt das restriktivere Recht. Der Mitarbeiter darf also nur **lesen**. Bei lokaler Anmeldung am Server werden Freigabeberechtigungen nicht ausgewertet, es gilt nur NTFS. Auch dort darf er nur **lesen**. Damit er schreiben kann, muss die NTFS-Berechtigung auf „Ändern“ erhöht werden.

## Falle

- Freigabe- und NTFS-Rechte addieren. Zwischen den Ebenen gilt das **Minimum**.
- Freigaberechte für lokalen Zugriff ansetzen. Sie gelten nur über das Netzwerk.
- „Verweigern“ großzügig einsetzen. Es sperrt auch Admins in derselben Gruppe aus, besser Rechte gar nicht erst vergeben.
- Rechte an Einzelbenutzer vergeben. Das ist nach dem Ausscheiden nicht mehr nachvollziehbar (verwaiste SIDs).
- „Ändern“ und „Schreiben“ gleichsetzen. Nur „Ändern“ erlaubt das **Löschen**.

## Merksatz

**„In der Ebene addieren, zwischen den Ebenen das Strengere. Verweigern schlägt Zulassen.“**

## Verwandt

- [chmod](../chmod.md) – Linux-Rechte
- [Dateisysteme](../../Komponenten/Storage/dateisysteme.md) – NTFS, ReFS, FAT32
- [Active Directory](../Sicherheit/active-directory.md) – Gruppen und Domäne
- [RBAC](../Netzwerk-Administration/rbac.md) – rollenbasierte Rechte
- [Least Privilege](../Sicherheit/least-privilege.md)
- [GPO](../Netzwerk-Administration/gpo.md)
