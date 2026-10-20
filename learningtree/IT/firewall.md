# Firewall

## Firewall-Typen und Funktionsweisen (IHK)

Eine **Firewall** kontrolliert Netzwerkverkehr anhand definierter Sicherheitsregeln. Sie schützt Systeme und Netzwerke, bietet aber keinen vollständigen Schutz gegen Phishing, Social Engineering, Malware über Wechseldatenträger oder bereits kompromittierte Systeme.

| Firewall-Typ | Funktionsweise | OSI-Schicht | Vorteile / Grenzen |
|---|---|---|---|
| Paketfilter-Firewall | Prüft IP-Adressen, Protokolle und Ports einzelner Pakete. | 3/4 | Schnell, aber keine tiefe Inhaltsanalyse. |
| Stateful Firewall | Verfolgt den Zustand von Verbindungen und prüft, ob Pakete zu einer erlaubten Sitzung gehören. | 3/4 | Erkennt ungültige oder unerwartete Verbindungspakete; analysiert nicht automatisch den Anwendungsinhalt. |
| Proxy-Firewall / Application-Level Gateway | Vermittelt zwischen Client und Server und prüft Anwendungsprotokolle. | 7 | Detaillierte Kontrolle; zusätzlicher Verarbeitungsaufwand. |
| Application Firewall | Kontrolliert anwendungsspezifische Kommunikation anhand definierter Regeln. | Häufig 7 | Erkennt unerlaubte Anwendungsaktionen; Schutz hängt von Regeln und Protokollunterstützung ab. |
| WAF (Web Application Firewall) | Analysiert HTTP(S)-Anfragen an Webanwendungen. | 7 | Schutz gegen bestimmte SQL-Injection- und XSS-Angriffe; kein vollständiger Ersatz für sichere Programmierung. |
| NGFW (Next-Generation Firewall) | Kombiniert Stateful Inspection mit Anwendungserkennung, Deep Packet Inspection und häufig IDS/IPS. | 3–7, je nach Funktion | Erweiterte Erkennung; höhere Kosten und Konfigurationsaufwand. |
| Host-/Personal Firewall | Kontrolliert ein- und ausgehenden Verkehr direkt auf einem Endgerät. | 3–7, je nach Funktion | Schützt einzelne Hosts auch außerhalb des Firmennetzes; kein Schutz vor allen lokalen Angriffen. |
| Netzwerk-Firewall | Kontrolliert zentral den Verkehr zwischen Netzwerken, z. B. LAN und Internet. | Abhängig vom Firewall-Typ | Schützt mehrere Systeme; Fehlkonfigurationen können ganze Netzbereiche gefährden. |

## IHK-Prüfungswissen

### Paketfilter vs. Stateful Firewall

- **Paketfilter:** Entscheidet anhand einzelner Paketmerkmale, z. B. Quell-IP, Ziel-IP, TCP und Port 443.
- **Stateful:** Berücksichtigt zusätzlich den Zustand der Verbindung und die zugehörigen Sitzungsinformationen.
- **Beispiel:** Eine Stateful Firewall lässt Antwortpakete einer erlaubten Verbindung zu, ohne jede Antwortverbindung separat freigeben zu müssen.

### WAF vs. NGFW

- **WAF:** Spezialisierter Schutz für Webanwendungen, insbesondere gegen Angriffe auf HTTP(S)-Anfragen.
- **NGFW:** Breiterer Netzwerkschutz mit Anwendungsidentifikation, Inhaltsanalyse und häufig IPS.
- **Abgrenzung:** Eine WAF ersetzt keine Netzwerk-Firewall; beide können sich ergänzen.

### Typische Firewall-Regeln

| Regel | Wirkung |
|---|---|
| TCP 443 erlauben | HTTPS-Verbindungen zulassen |
| TCP 22 aus dem Internet blockieren | SSH-Zugriff von extern verhindern |
| Eingehenden Verkehr standardmäßig blockieren | Nicht ausdrücklich erlaubte Verbindungen verwerfen |
| Nur erforderliche Ports freigeben | Angriffsfläche reduzieren |
| Ausgehenden Verkehr kontrollieren | Verbindungen kompromittierter Systeme einschränken |

### Grenzen einer Firewall

- Phishing und Social Engineering werden nicht zuverlässig verhindert.
- Malware kann über USB oder andere lokale Medien eingeschleust werden.
- Bereits kompromittierte interne Systeme können erlaubte Verbindungen missbrauchen.
- Verschlüsselter Verkehr ist ohne geeignete Inspektion nur eingeschränkt inhaltlich prüfbar.
- Fehlkonfigurationen und unbekannte Schwachstellen können Schutzmaßnahmen umgehen.

### Verknüpfung mit anderen IHK-Themen

- **OSI-Modell:** Paketfilter typischerweise Layer 3/4; WAF typischerweise Layer 7.
- **Netzwerkgeräte:** Router verbinden Netze; Firewalls kontrollieren den Verkehr; Switches verbinden Geräte innerhalb eines LANs.
- **VPN/IPsec:** Schützt die Kommunikation zwischen Standorten oder Endpunkten, ersetzt aber keine Firewall-Regeln.
- **DSGVO Art. 32:** Firewalls können Bestandteil angemessener technischer und organisatorischer Maßnahmen (TOM) sein.
- **Schutzziele:** Firewalls unterstützen Vertraulichkeit, Integrität und Verfügbarkeit, garantieren diese aber nicht.

**Merksatz:** Eine Firewall kontrolliert Netzwerkverkehr. Sie ersetzt weder Endgeräteschutz noch sichere Software, Benutzeraufklärung oder ein funktionierendes Backup-Konzept.

## Siehe auch

- [Proxy](proxy.md) – Application-Level Gateway
- [Netzwerktypen](netzwerktypen.md) – LAN, WAN und Segmentierung
- [hackers-can](hackers-can.md) – welche Angriffe abgewehrt werden
- [OSI](osi.md) – Zuordnung der Firewall-Typen zu Schichten

