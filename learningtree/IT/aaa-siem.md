# AAA-SIEM

## 1. Ziel
- **AAA (Authentication, Authorization, Accounting)** zentral steuern, um zu definieren, wer sich anmelden darf, welche Aktionen er ausführen darf und welche Aktivitäten protokolliert werden.
- **SIEM (Security Information & Event Management)** sammelt, normalisiert und korreliert Logs aller Systeme, um Sicherheitsvorfälle zu erkennen und DSGVO‑konforme Audits zu erfüllen.

---

## 2. AAA – Übersicht

| Komponente | Aufgabe | Typische Implementierung | AD‑Integration |
|------------|---------|--------------------------|----------------|
| **Authentication** | Prüft Anmeldeidentität | • **RADIUS** (NPS, FreeRADIUS, CISCO ISE) – WLAN, VPN<br>• **TACACS+** – Netzwerkgeräte‑Admini<br>• **LDAP/Kerberos** – Windows‑Geräte | AD dient als Benutzer‑/Gruppen‑Quelle; RADIUS/TACACS+ fragt den AD‑Server (via LDAP) nach Berechtigungsdaten. |
| **Authorization** | Entscheidet über zulässige Aktionen | • **RADIUS**‑Attribute (e.g. Framed‑Policy‑Key) <br>• **TACACS+**‑Policy‑Sets <br>• **AD‑Gruppen** (Security, Distribution, Universal) | AD‑Gruppen werden in den AAA‑Servern als Policy‑Referenzen verwendet. |
| **Accounting** | Protokolliert, was der Nutzer macht | • **RADIUS** – Session‑Start/Stop, Framed‑IP‑Address <br>• **TACACS+** – Command‑Log, Session‑Duration <br>• **AD** – begrenzte Nutzung (z. B. Logon‑Events) | AD‑Logon‑Events (Security‑Event‑ID 4624/4625) dienen als Accounting‑Basis. |

### 2.1 RADIUS‑Server (z. B. Microsoft NPS)
- **Datenbank:** AD‑LDAP (Benutzer‑ und Gruppeninformationen).
- **Policy‑Engine:** Mapping von Gruppen → Rechte (z. B. VPN‑Zugang nur für „VPN‑Admins“).
- **Protokollierung:** Session‑Start/Stop, Access‑Reject/Accept, Detail‑Logs (via Event‑Log).

### 2.2 TACACS+ (Cisco ISE, FreeRADIUS‑TACACS+)
- **Transport:** TCP 49, verschlüsselt (TLS).
- **Rollen‑basiert:** Admin‑Geräte → *Privileged‑Exec*, *Configuration*‑Rechte.
- **Logging:** Vollständige Command‑History, Session‑Zeit, Auth‑/Authz‑Ergebnisse.

### 2.3 AD als Datenquelle
- **Klasse:** `user`, `computer`, `group`.
- **Attribute für AAA:** `memberOf` (Gruppen), `userPrincipalName`, `sAMAccountName`.
- **Filter:** z. B. nur Geräte mit `msDS-ManagedBy` = „VPN‑Server“.

---

## 3. SIEM – Log‑Management & Korrelation

| Quelle | Log‑Typ | Relevante Felder | Nutzung im SIEM |
|--------|---------|------------------|-----------------|
| **OS (Windows/Linux)** | Sicherheits‑Events, System‑Events | Event‑ID, Source‑IP, User‑Name, Prozess‑ID | Anmeldeversuche, Fehlermeldungen, Service‑Start/Stop |
| **Anwendungen** | Application‑Logs | Logger‑Name, Message‑Level, DB‑Query‑Fehler | DB‑Zugriffs‑Anomalien, Business‑Logik‑Fehler |
| **Netzwerkgeräte** | Syslog, SNMP, NetFlow | Device‑IP, Port, Protocol, Event‑Code | Brute‑Force‑Erkennung, Ungewöhnlicher Traffic, FW‑Block‑Events |
| **AAA‑Server** | Auth‑/Authz‑Logs, Accounting‑Logs | User‑Name, Auth‑Method, Result, Session‑ID | Nachvollziehbare Zugriffs‑Kontrolle, Audits, Incident‑Response |

### 3.1 Log‑Pipeline
1. **Collection Agent** (z. B. Filebeat, Winlogbeat, nxlog) → **Transport** (TLS, Syslog‑UDP/TCP).
2. **Normalisierung** (Log‑Parser, JSON‑Format) → **Enrichment** (Verknüpfung mit AD‑Gruppen, Host‑Inventar).
3. **Speicherung** (Elasticsearch, Splunk, Graylog) → **Index‑Retention** (z. B. 90 Tage für DSGVO).

### 3.2 Korrelation
- **Rule‑Beispiele:**
  - *Mehrfach‑Fehlversuche* (z. B. >5 Fehlversuche innerhalb 5 min) → Alert „Possible Brute‑Force“.
  - *Login von ungewöhnlicher IP* → Alert „Unusual Remote Access“.
  - *TACACS+‑Command‑Log* + *RADIUS‑Session* → Korrelation von Admin‑Aktion und Netzwerk‑Zugriff.

### 3.3 Compliance & DSGVO
- **CIA‑Modell** (Confidentiality, Integrity, Availability) wird durch:
  - **Verschlüsselung** (TLS, DB‑AES) → Confidentiality.
  - **Audit‑Trail** (SIEM‑Log‑Retention) → Integrity.
  - **Verfügbarkeit** (redundante SIEM‑Nodes, Backup) → Availability.
- **Audit‑Log‑Aufbewahrung** ≥ 12 Monate (je nach Unternehmens‑Richtlinie).

---

## 4. Empfohlene Tools (Kurz‑Übersicht)

| Aufgabe | Tool | Hinweis |
|---------|------|---------|
| **AAA‑Server** | Microsoft NPS (RADIUS) <br> Cisco ISE (TACACS+) <br> FreeRADIUS (Open‑Source) | Alle unterstützen AD‑Backend. |
| **Log‑Collection** | Filebeat, Winlogbeat, nxlog, rsyslog | Leicht in Docker/VM einsetzbar. |
| **SIEM** | Elastic Stack (ELK) <br> Splunk <br> Graylog | Open‑Source‑Option: Elastic; kommerziell: Splunk. |
| **AD‑Management** | PowerShell (RSAT), ADUC, `repadmin`, `ldp.exe` | Skripte für Bulk‑Import/Export (CSV). |
| **Monitoring** | Zabbix, PRTG, Prometheus + Grafana | Kann AAA‑Metriken (Session‑Count, Auth‑Rate) abfragen. |

---

## 5. Best‑Practice‑Checkliste (Kurz)

- **AD‑Gruppen** klar nach Funktions‑ und Rollen‑Kategorien strukturieren (z. B. *VPN‑Users*, *Network‑Admins*).
- **RADIUS‑Policy** strikt auf **Group‑Based** Zuweisungen setzen – keine Hard‑Coded‑Benutzer.
- **TACACS+** mit **TLS** aktivieren, Zertifikate regelmäßig erneuern.
- **Log‑Retention** gemäß DSGVO‑Anforderungen (mind. 12 Monate, unveränderlich).
- **SIEM‑Correlation‑Rules** regelmäßig prüfen und an neue Bedrohungen anpassen (Threat‑Intel‑Feed).
- **Backup** von AAA‑Konfigurationen (NPS‑Policy‑Export, ISE‑Config‑Backup).
- **Test‑Umgebung** für Änderungen (z. B. Test‑RADIUS‑Server) bevor Produktion.

---
