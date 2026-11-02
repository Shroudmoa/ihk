# Lektion 17 – DHCP & DNS

**Bereich:** Netzdienste (Layer 7 / Infrastruktur)
**Relevanz:** Hoch – DORA-Ablauf, Relay und DNS-Auflösung sind Klassiker in AP2
**Lernzeit:** ca. 2h

## 1. Warum das wichtig ist
- DHCP verteilt automatisch IP-Konfiguration – Fehler führen zu „kein Netz".
- DNS macht Namen auflösbar – ohne DNS „geht nichts", obwohl Ping per IP klappt.
- Prüfung fragt Ablauf, Optionen, Relay und typische Diagnose.

## 2. Grundlagen – DHCP-Ablauf (DORA)
| Schritt | Name | Richtung | Beschreibung |
|---------|------|----------|--------------|
| 1 | **Discover** | Broadcast | Client sucht DHCP-Server |
| 2 | **Offer** | unicast/bcast | Server bietet Adresse an |
| 3 | **Request** | Broadcast | Client fordert angebotene Adresse an |
| 4 | **Ack** | unicast/bcast | Server bestätigt, Lease beginnt |

**Wichtige DHCP-Optionen:**
- Option 1: Subnetzmaske
- Option 3: Default-Gateway (Router)
- Option 6: DNS-Server
- Option 51: Lease-Time

## 3. DHCP-Relay
- DHCP-Server in **anderem Subnetz** als Client → Broadcast erreicht ihn nicht.
- Lösung: **DHCP-Relay / ip helper-address** auf dem Gateway-Router/SVI.
- Router leitet Broadcast als gerichtetes Unicast an den Server weiter.

## 4. Grundlagen – DNS
| Begriff | Bedeutung |
|---------|-----------|
| **Rekursiv** | Resolver macht die komplette Kette für den Client bis zur Antwort |
| **Iterativ** | Server verweist nur auf den nächsten zuständigen Server |
| **A-Record** | Name → IPv4-Adresse (z. B. `www.firma.de → 93.184.216.34`) |
| **PTR** | IP → Name (Reverse Lookup) |

**Auflösungskette:**
`Client → lokaler Resolver → Root-Server → TLD-Server (.de) → autoritativer Server (firma.de)`

## 5. Intern vs. öffentlich / Split-Horizon
- **Interner DNS:** interne Namen (z. B. `intranet.firma.local`) nur für das LAN.
- **Öffentlicher DNS:** für Internet erreichbare Namen.
- **Split-Horizon-DNS:** derselbe Name liefert **innen andere IP** als **außen** (z. B. intern private IP, extern öffentliche IP).

## 6. Reverse Proxy vs. Portweiterleitung
| Merkmal | Portweiterleitung | Reverse Proxy |
|---------|-------------------|---------------|
| Ebene | Layer 4 (Port/IP) | Layer 7 (HTTP-Host/URL) |
| Zielauswahl | nach Port | nach Hostname/ Pfad |
| TLS/Filter | nein | ja (SSL offload, WAF) |

## 7. Übungsaufgabe (im Prüfungsstil, selbst erstellt)

Ein Client erhält per DHCP eine IP, kann Webserver im Internet per **IP** erreichen, aber `www.example.com` wird nicht aufgelöst („DNS-Server responds: no such name"). Der Client pingt den konfigurierten DNS-Server erfolgreich. Nenne drei mögliche Ursachen.

<details>
<summary>Lösung (aufklappen)</summary>

1. **Falscher / erreichbarer, aber falscher DNS-Server:** Option 6 liefert DNS, der nicht autoritativ/forwarding-fähig ist (z. B. Gateway statt echter Resolver) → keine Weiterauflösung.
2. **Split-Horizon / interner Name:** Der Name existiert nur im internen DNS; Client nutzt aber öffentlichen DNS → „no such name".
3. **Forwarding defekt / Firewall blockiert UDP/53:** DNS-Server selbst ist per Ping (ICMP) erreichbar, aber Port 53 (DNS) wird blockiert oder Forwarder antwortet nicht.
4. (Zusatz) **A-Record fehlt:** Im zuständigen DNS fehlt der Eintrag komplett.

Diagnose-Sequenz: `ipconfig /all` (DNS-Option prüfen) → `nslookup www.example.com` → `nslookup` gegen anderen Resolver testen.

</details>

## Selbstcheck – kannst du das?
- [ ] Ich kann den DORA-Ablauf Schritt für Schritt nennen.
- [ ] Ich kenne die wichtigsten DHCP-Optionen (1, 3, 6, 51).
- [ ] Ich erkläre, wann ein DHCP-Relay nötig ist.
- [ ] Ich unterscheide rekursive und iterative DNS-Auflösung.
- [ ] Ich erkläre Split-Horizon-DNS und Reverse Proxy vs. Portweiterleitung.

## Weiterführende Ressourcen
- RFC 2131 (DHCP), RFC 1034/1035 (DNS)
- IHK-Prüfungsaufgaben AP2 zu Netzdiensten
- Cisco: ip helper-address Konfiguration
