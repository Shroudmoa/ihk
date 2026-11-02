# VDI, Terminalserver und Thin Client

## In 30 Sekunden

- Bei **Desktop-Virtualisierung** läuft der Arbeitsplatz im Rechenzentrum. Der Client zeigt nur Bild, Ton und Eingaben an (Protokolle RDP, ICA/HDX, Blast, PCoIP).
- **VDI** (Virtual Desktop Infrastructure): **jeder Benutzer eine eigene VM** mit Client-Betriebssystem.
- **Terminalserver** (RDS, Session-based): **viele Benutzer teilen ein Server-Betriebssystem**, jeder bekommt eine Sitzung.
- **Thin Client**: kleines, sparsames Endgerät ohne lokale Daten. **Zero Client**: noch einfacher, ohne eigenes Betriebssystem. **Fat Client**: normaler PC.
- Vorteile: zentrale Verwaltung und Datenhaltung, Sicherheit, wenig Strom am Arbeitsplatz. Nachteile: Abhängigkeit von Netz und Rechenzentrum, Lizenzkosten, Grafikleistung.

## Kernwissen

### VDI vs. Terminalserver

| Kriterium | VDI | Terminalserver (RDS) |
|---|---|---|
| Betriebssystem | eigene Client-VM je Benutzer (Windows 11) | ein Windows Server, viele Sitzungen |
| Isolation | stark, eigene VM | schwächer, ein Absturz trifft alle Sitzungen |
| Individualisierung | hoch (eigene Software möglich) | gering (gleiche Software für alle) |
| Ressourcenbedarf | hoch (RAM/CPU je VM) | gering (geteiltes OS) |
| Varianten | **persistent** (Desktop bleibt) oder **nicht persistent** (nach Abmeldung zurückgesetzt) | Sitzungen auf einer Serverfarm mit Broker |
| Lizenz | Windows-VDA-Lizenz bzw. passendes Abo + Zugriffslizenzen | Windows Server CAL **und** RDS CAL |
| Einsatz | Entwicklung, Spezialsoftware, Power-User | Callcenter, Schulung, einheitliche Büroarbeitsplätze |

### Thin Client vs. Fat Client

| | Thin Client | Fat Client (PC) |
|---|---|---|
| Rechenleistung | gering, Anwendung läuft im RZ | lokal |
| Daten | keine lokalen Daten | lokal gespeichert |
| Stromverbrauch | ca. 5–15 W | ca. 50–150 W |
| Verwaltung | zentral, Austausch in Minuten | Software und Patches je Gerät |
| Lebensdauer | lang (kaum Aufrüstbedarf) | kürzer |
| ohne Netzwerk | nicht arbeitsfähig | arbeitsfähig |

### Rechenweg: Stromersparnis

60 Arbeitsplätze, PC 90 W, Thin Client 12 W, 8 h · 220 Arbeitstage = 1.760 h/Jahr.
- Ersparnis pro Platz: 78 W · 1.760 h = 137,28 kWh
- 60 Plätze: 8.236,8 kWh · 0,30 €/kWh ≈ **2.471 €/Jahr**
- Gegenrechnen: zusätzlicher Stromverbrauch der VDI-Hosts im RZ (inkl. Kühlung). Die Ersparnis ist daher kleiner als die reine Arbeitsplatzdifferenz.

### Planungsaspekte

- **Netz:** Bandbreite und Latenz pro Sitzung (Büroarbeit ca. 0,1–2 Mbit/s, Video/3D deutlich mehr), QoS, redundante Anbindung.
- **Bootstorm/Anmeldespitze** morgens um 8 Uhr: Storage-IOPS ausreichend dimensionieren (SSD, Caching).
- **Hochverfügbarkeit:** Fällt der Host oder das Netz aus, kann niemand arbeiten → Cluster, redundante Broker.
- **Profile:** servergespeicherte Profile bzw. Profilcontainer (FSLogix), damit Einstellungen erhalten bleiben.
- **Speicher sparen:** viele gleiche Desktops → Linked Clones/Deduplizierung.

## Prüfungsfrage

**Frage:** Ein Callcenter mit 80 gleichartigen Arbeitsplätzen soll modernisiert werden. Empfehlen Sie Terminalserver mit Thin Clients oder VDI und nennen Sie zwei Vorteile sowie ein Risiko Ihrer Lösung.

**Musterantwort:** Empfohlen werden Terminalserver mit Thin Clients, weil alle Beschäftigten dieselbe Software nutzen und keine individuellen Desktops brauchen. Das geteilte Server-Betriebssystem benötigt weniger Ressourcen als 80 einzelne VMs. Vorteile: Software wird zentral nur auf den Terminalservern installiert und gepatcht, und es liegen keine Daten auf den Endgeräten, was Datenschutz und Diebstahlschutz verbessert. Die Thin Clients verbrauchen zudem wenig Strom. Risiko: Fällt ein Terminalserver oder die Netzanbindung aus, können viele Mitarbeiter gleichzeitig nicht arbeiten. Deshalb sind mehrere Server in einer Farm mit Lastverteilung und eine redundante Netzanbindung nötig.

## Falle

- VDI und Terminalserver gleichsetzen. VDI = **eine VM je Benutzer**, RDS = **eine Sitzung je Benutzer**.
- Die RDS CAL vergessen. Sie wird **zusätzlich** zur normalen Server-CAL benötigt.
- Thin Clients als „offline-fähig“ verkaufen. Ohne Netz geht nichts.
- Nur die Ersparnis am Arbeitsplatz rechnen und den Mehrbedarf im Rechenzentrum ignorieren.

## Merksatz

**„VDI: jeder sein eigenes Haus. Terminalserver: alle in einer WG. Thin Client: nur die Fernbedienung.“**

## Verwandt

- [Virtualisierung](virtualisierung.md) – Arten der Virtualisierung
- [VM-Container-Server](vm-container-server.md)
- [Lizenzierung](../lizenzierung.md) – CAL und RDS CAL
- [Energieeffizienz & Stromkosten](../energieeffizienz-stromkosten.md)
- [Deduplizierung & Komprimierung](../../../Komponenten/Storage/deduplizierung-komprimierung.md)
- [HA-Hochverfügbarkeit](../ha-hochverfuegbarkeit.md)
- [Lektion L13 Virtualisierung](../../../analyse-der-exams/L13_Virtualisierung.md)
