# Lektion 16 – Firewall & Port-Security

**Bereich:** Netzwerksicherheit (Perimeter / Access-Layer)
**Relevanz:** Hoch – Sicherheitskonzepte und Regelinterpretation sind Prüfungsschwerpunkt
**Lernzeit:** ca. 2h

## 1. Warum das wichtig ist
- Firewalls bilden den Schutz am Netzübergang (Perimeter) und intern zwischen Zonen.
- In der Prüfung muss man Regelwerke **lesen, bewerten und selbst schreiben** können.
- Port-Security schützt den Access-Layer (Switch) vor unerlaubten Endgeräten.
- Fehlkonfiguration = entweder Sicherheitslücke oder Legitimation blockiert.

## 2. Grundlagen – Firewall-Aufgaben
| Funktion | Kurz erklärt |
|----------|--------------|
| **Paketfilter** | prüft src/dst IP, Port, Protokoll (Layer 3/4) |
| **Stateful Packet Inspection (SPI)** | merkt Verbindungszustand (Session-Tracking), nur Antworten auf erlaubte Requests durchlassen |
| **NAT / PAT** | übersetzt private in öffentliche IPs (PAT = Port-Adress-Übersetzung) |
| **VPN-Endpunkt** | Terminierung von Tunneln (IPsec/SSL) |
| **Logging / Alerting** | Protokollierung und Alarmierung verdächtiger Ereignisse |

## 3. Regeln lesen und erstellen
Schema: **Quelle → Ziel / Port / Protokoll / Richtung → Aktion (allow/deny)**

| Nr | Quelle | Ziel | Port | Protokoll | Richtung | Aktion |
|----|--------|------|------|-----------|----------|--------|
| 1 | LAN (192.168.1.0/24) | ANY | 443 | TCP | out | allow |
| 2 | ANY | WAN-IP:443 | 443 | TCP | in | allow |
| 3 | GAST (192.168.9.0/24) | LAN | ANY | ANY | – | deny |
| 4 | ANY | ANY | ANY | ANY | – | deny (implizit) |

**Beispiel:** Regel 3 blockiert Gästen den Zugriff ins interne LAN, erlaubt aber (via Regel 1) Surfen ins Internet.

## 4. SPI vs. NGFW
| Merkmal | SPI-Firewall | NGFW (Next Generation) |
|---------|--------------|------------------------|
| Filtertiefe | Layer 3/4 + Zustand | zusätzlich **Layer 7** (Anwendung) |
| App-Control | nein | ja (z. B. YouTube blocken) |
| IDS/IPS | meist nein | integriert |
| TLS-Inspection | nein | ja (entschlüsselt zur Prüfung) |

## 5. Grenzen des Schutzes
- **Kein** Schutz vor Social Engineering (Phishing).
- **Kein** Schutz vor Insider-Angriffen aus dem vertrauenswürdigen LAN.
- **Kein** Schutz vor USB-Malware (physisch eingebracht).
- **Kein** Schutz vor verschlüsseltem internem Traffic (keine Inspection).
- Firewall ersetzt nicht: Patch-Management, Awareness, Endpoint-Schutz.

## 6. Port-Security (Switch-Access-Port)
- Beschränkt erlaubte **MAC-Adressen** an einem Port.
- **Statisch:** MAC manuell hinterlegt.
- **Sticky:** gelernte MAC wird automatisch übernommen und gespeichert.
- **Violation Mode:**
  | Modus | Verhalten bei Verstoß |
  |-------|------------------------|
  | **shutdown** | Port wird deaktiviert (err-disable) |
  | **restrict** | Paket verworfen + Log/SNMP |
  | **protect** | Paket still verworfen, kein Log |

## 7. Übungsaufgabe (im Prüfungsstil, selbst erstellt)

Gegeben folgende Regel-Tabelle einer Unternehmens-Firewall:

| Nr | Quelle | Ziel | Port | Proto | Aktion |
|----|--------|------|------|-------|--------|
| 1 | 10.0.0.0/24 | 10.0.1.10 | 3389 | TCP | allow |
| 2 | 10.0.0.0/24 | ANY | 80,443 | TCP | allow |
| 3 | ANY | 10.0.1.10 | 3389 | TCP | deny |
| 4 | ANY | ANY | ANY | ANY | deny |

Ein Mitarbeiter aus `10.0.0.5` darf per RDP auf den Server `10.0.1.10` zugreifen, ein externer Partner (Internet) jedoch nicht. Bewerte, ob die Tabelle das erfüllt, und begründe die Reihenfolge.

<details>
<summary>Lösung (aufklappen)</summary>

Ja, die Tabelle erfüllt die Anforderung:
- Regel 1 erlaubt RDP (3389/TCP) aus dem internen Netz `10.0.0.0/24` zum Server – trifft zuerst für den Mitarbeiter.
- Regel 3 (deny für ANY nach `10.0.1.10`:3389) blockiert externen Partner, da dessen Quelle **nicht** in `10.0.0.0/24` liegt und Regel 1 nicht passt.
- Regel 4 ist die implizite/explizite Catch-All-Sperre.
**Wichtig:** Firewalls werten Regeln **top-down, first-match**. Reihenfolge ist entscheidend – spezifische allow-Regeln müssen vor allgemeinen deny-Regeln stehen.

</details>

## Selbstcheck – kannst du das?
- [ ] Ich kann eine Firewall-Regeltabelle lesen und bewerten.
- [ ] Ich erkläre den Unterschied SPI vs. NGFW.
- [ ] Ich nenne drei Grenzen des Firewall-Schutzes.
- [ ] Ich beschreibe Port-Security (static/sticky) und Violation-Modi.
- [ ] Ich weiß, dass Firewall-Regeln First-Match auswertet werden.

## Weiterführende Ressourcen
- IHK-Prüfungsaufgaben AP2 zu Sicherheitskonzepten
- Cisco: Port-Security Konfiguration
- BSI-Grundschutz: Netzarchitektur und Firewall
