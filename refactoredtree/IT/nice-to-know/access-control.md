# Zugriffskontrolle (Access Control)

## In 30 Sekunden

- Zugriffskontrolle legt fest und setzt durch, **wer** (Identität) **worauf** (Ressource) **wie** (Recht) zugreifen darf.
- Ablauf nach **AAA**: **Authentifizierung** (Wer bist du?) → **Autorisierung** (Was darfst du?) → **Accounting** (Was hast du getan?).
- Modelle: **DAC** (Besitzer entscheidet), **MAC** (System nach Sicherheitsstufen), **RBAC** (Rolle entscheidet), **ABAC** (Attribute/Kontext entscheiden).
- Umgesetzt wird das technisch über **ACLs** am Objekt (NTFS, Linux-Rechte, Router/Firewall).
- Leitprinzipien: **Least Privilege**, **Need-to-know**, **Vier-Augen-Prinzip**, Funktionstrennung.

## Kernwissen

**Zugriffskontrollmodelle**

| Modell | Wer entscheidet? | Merkmal | Beispiel |
|---|---|---|---|
| **DAC** – Discretionary Access Control (benutzerbestimmbar) | der **Besitzer** des Objekts | Besitzer kann Rechte weitergeben; flexibel, aber fehleranfällig | NTFS-Rechte, Linux `chmod` |
| **MAC** – Mandatory Access Control (systembestimmt) | das **System** anhand von Sicherheitsstufen/Labels | Benutzer kann Rechte **nicht** weitergeben; sehr strikt | SELinux, AppArmor, Geheimhaltungsstufen (VS-NfD, geheim) |
| **RBAC** – Role-Based Access Control | die **Rolle** im Unternehmen | Rechte an Rollen, Benutzer in Rollen; Standard in Unternehmen | AD-Sicherheitsgruppen, Rollen im ERP |
| **ABAC** – Attribute-Based Access Control | **Regeln über Attribute** (Abteilung, Standort, Uhrzeit, Gerät) | sehr fein, kontextabhängig | Conditional Access: Zugriff nur von verwalteten Geräten |

**ACL (Access Control List):** Liste von Einträgen an einem Objekt – wer welches Recht hat („Gruppe Vertrieb: Lesen, Schreiben“, „Gäste: Verweigern“). Gibt es bei Dateisystemen, aber auch als Paketfilter auf Routern und Firewalls. Ein explizites **Verweigern** hat bei NTFS Vorrang vor einem Erlauben.

**Weitere Begriffe**
- **Least Privilege:** nur die minimal nötigen Rechte.
- **Need-to-know:** nur Zugriff auf Informationen, die man für die konkrete Aufgabe kennen muss.
- **Vier-Augen-Prinzip:** kritische Aktionen (Überweisung, Löschen eines Servers) brauchen die Bestätigung einer zweiten Person.
- **Funktionstrennung:** unvereinbare Aufgaben (Bestellung anlegen und freigeben) auf verschiedene Personen verteilen.
- **SSO (Single Sign-On):** einmal anmelden, danach Zugriff auf mehrere Dienste ohne erneute Passworteingabe (z. B. Kerberos im AD).
- **MFA:** Anmeldung mit mindestens zwei Faktoren verschiedener Kategorien.

**Abgrenzung zu den TOM:** Zutrittskontrolle (Raum), Zugangskontrolle (System), **Zugriffskontrolle (Daten)** – dieses Blatt behandelt die Zugriffskontrolle im technischen Sinn.

**Hinweis Lizenzierung:** **CAL** (Client Access License) ist kein Sicherheitsmodell, sondern eine **Lizenz** für den Zugriff auf Serverdienste (User-CAL pro Person, Device-CAL pro Gerät) – siehe [Lektion 10 Lizenzierung](../analyse-der-exams/10_Lizenzierung.md).

## Prüfungsfrage

**Frage:** Unterscheiden Sie DAC und RBAC und erläutern Sie, warum Unternehmen meist RBAC einsetzen.

**Musterantwort:** Bei DAC entscheidet der Besitzer einer Datei selbst, wem er welche Rechte gibt, und kann diese auch weitergeben. Bei RBAC werden Rechte zentral an Rollen vergeben, die den Funktionen im Unternehmen entsprechen, und Benutzer erhalten ihre Rechte nur über die Rollenzugehörigkeit. Unternehmen setzen RBAC ein, weil die Rechtevergabe zentral gesteuert, übersichtlich und prüfbar bleibt: Bei einem Abteilungswechsel wird nur die Rolle geändert, und das Least-Privilege-Prinzip lässt sich besser durchsetzen als bei individuell vergebenen Besitzerrechten.

## Falle

- MAC (Mandatory Access Control) mit der **MAC-Adresse** verwechseln.
- DAC als „diskret“ übersetzen – gemeint ist **benutzerbestimmbar** (nach Ermessen des Besitzers).
- Authentifizierung und Autorisierung vertauschen: erst **wer**, dann **was**.
- CAL als Zugriffskontrollmodell beschreiben.

## Merksatz

**„DAC: der Besitzer. MAC: das System. RBAC: die Rolle. ABAC: der Kontext.“**

## Verwandt

- [RBAC](../FiSi-Administrator/Netzwerk-Administration/rbac.md)
- [Least Privilege](../FiSi-Administrator/Sicherheit/least-privilege.md)
- [AAA und SIEM](../FiSi-Administrator/Sicherheit/aaa-siem.md)
- [MFA](../FiSi-Administrator/Sicherheit/MFA.md)
- [Authentifizierung und Autorisierung](../FiSi-Administrator/Server/authentifizierung-autoritisierung-mfa.md)
- [chmod](../FiSi-Administrator/chmod.md)
- [TOM](../Management/DSGVO/toms.md)
