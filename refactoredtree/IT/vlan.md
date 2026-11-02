# VLAN (Virtual LAN)

## In 30 Sekunden

- **VLAN** = Logische Trennung eines physischen Switch-Netzes
- Trennt **Broadcast-Domänen** ohne extra Switches
- Höhere Sicherheit, weniger Broadcast-Verkehr, bessere Struktur
- **Access-Port** = Für Endgeräte (gehört zu genau einem VLAN)
- **Trunk-Port** = Verbindet Switches, transportiert mehrere VLANs (802.1Q)
- **VLAN-Hopping** = Angriff, den man durch richtiges Konfigurieren vermeiden kann

## Kernwissen

### Warum VLANs?

Ohne VLAN: Alle Geräte sind in derselben Broadcast-Domäne. Mit VLAN: Jedes VLAN ist eine eigene Broadcast-Domäne. Kommunikation zwischen VLANs ist **nur über einen Router oder Layer-3-Switch** möglich.

### Portarten

| Portart | Funktion | Erklärung |
|---|---|---|
| **Access-Port** | Endgeräte | Gehört zu **einem** VLAN. Ungetaggt. z. B. PC, Drucker |
| **Trunk-Port** | Switch zu Switch | Trägt **mehrere** VLANs. Markiert mit **802.1Q-Tag**. |
| **Hybrid-Port** | Sonderfall | Kann sowohl tagged als auch untagged Verkehr führen (seltener in Prüfung) |

### Inter-VLAN-Routing

Damit z. B. VLAN 10 mit VLAN 20 kommunizieren kann, brauchst du:

- **Router-on-a-Stick** – Ein Router mit Subinterfaces über einen Trunk
- **Layer-3-Switch** – Inter-VLAN-Routing direkt im Switch (sehr verbreitet)

### VLAN-Hopping

| Angriff | Erklärung | Gegenmaßnahme |
|---|---|---|
| **Switch Spoofing** | Angreifer gibt vor, ein Switch zu sein und fordert Trunk auf | **Trunk** nur dort aktivieren, wo wirklich Switch-Switch-Verbindung ist |
| **Double Tagging** | Zwei 802.1Q-Tags, um in fremdes VLAN zu gelangen | Native VLAN ändern, **Tagging** sauber konfigurieren |

## Prüfungsfrage

**Frage:** Warum teilt man ein Netzwerk mit VLANs auf?

**Musterantwort:** VLANs trennen ein physisches Netzwerk logisch in mehrere Broadcast-Domänen. Dadurch wird der Broadcast-Verkehr reduziert, die Netzstruktur übersichtlicher und die Sicherheit erhöht, da Geräte aus verschiedenen VLANs ohne Routing nicht direkt miteinander kommunizieren können. Für die Kommunikation zwischen VLANs ist ein Router oder Layer-3-Switch nötig.

## Falle

- **VLAN trennt Broadcast-Domänen, aber nicht automatisch alles** – Nur durch korrekte Konfiguration
- **Access-Port = nur 1 VLAN** – Trunk-Port = mehrere VLANs
- **802.1Q** = Standard für VLAN-Tagging
- **Native VLAN** – Ungetaggter Verkehr auf einem Trunk läuft im Native VLAN. Niemals das gleiche VLAN wie User-VLAN nehmen
- **VLAN 1** – Oft Default, aber in der Praxis lieber umkonfigurieren

## Merksatz

**„Access = rein, Trunk = gemischt.“**

## Verwandt

- [Netzwerktypen](netzwerktypen.md) – LAN-Segmentierung
- [Firewall](firewall.md) – Segmentierung zur Absicherung
- [OSI](osi.md) – Schicht 2 (Switching)
- [NAT & PAT](nat-pat.md) – Oft zusammen mit Routing
