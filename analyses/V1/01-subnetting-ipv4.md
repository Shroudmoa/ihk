# Lektion 1 – IPv4-Adressierung & Subnetting

**Bereich:** GA2 (Analyse und Entwicklung von Netzwerken)
**Relevanz:** sehr hoch – taucht in fast jeder AP2-Klausur in irgendeiner Form auf
**Lernzeit:** ca. 2h

---

## 1. Warum das wichtig ist

Subnetting ist die Grundlage für einen großen Teil der GA2-Aufgaben: Netzplanung, VLANs, Routing, DHCP-Konfiguration – überall wird vorausgesetzt, dass du aus einer IP-Adresse mit Präfix sofort ablesen kannst, welches Netz gemeint ist, wie viele Hosts reinpassen und wo die Grenzen liegen.

## 2. Die Grundlagen

Eine IPv4-Adresse hat **32 Bit**, aufgeteilt in 4 Oktette (je 8 Bit), z. B. `192.168.10.65`.

Das Präfix (z. B. `/26`) gibt an, wie viele Bits von links für das **Netz** reserviert sind. Der Rest sind **Host-Bits**.

```
Präfix    → Netzbits
32 - Präfix → Hostbits
```

### Formeln, die du auswendig können musst

| Was | Formel |
|---|---|
| Anzahl Adressen im Netz | 2^(Hostbits) |
| Nutzbare Hosts | 2^(Hostbits) − 2 (Netz- und Broadcast-Adresse abziehen) |
| Anzahl möglicher Subnetze bei gegebenem Ober-Präfix | 2^(neue Netzbits − alte Netzbits) |

⚠️ **Häufiger Fehler:** Hostbits mit Präfix verwechseln. Merke: `Hostbits = 32 − Präfix`, nicht umgekehrt.

## 3. Schritt-für-Schritt-Beispiel

Gegeben: `192.168.10.65/26`

1. **Hostbits berechnen:** 32 − 26 = 6 Bit
2. **Blockgröße bestimmen:** 2^6 = 64 → jedes Subnetz hat 64 Adressen
3. **Netzadresse finden:** Das 3. Oktett bleibt gleich (10), das 4. Oktett wird auf ein Vielfaches von 64 abgerundet:
   - Vielfache von 64: 0, 64, 128, 192
   - 65 liegt zwischen 64 und 128 → **Netzadresse = 192.168.10.64**
4. **Broadcast-Adresse:** Netzadresse + Blockgröße − 1 = 64 + 64 − 1 = 127 → **192.168.10.127**
5. **Nutzbarer Hostbereich:** 192.168.10.65 bis 192.168.10.126
6. **Nutzbare Hosts:** 2^6 − 2 = **62**

## 4. Subnetzmaske in Dezimalschreibweise

Falls in der Aufgabe die Maske statt des Präfixes verlangt wird:

| Präfix | Subnetzmaske |
|---|---|
| /24 | 255.255.255.0 |
| /25 | 255.255.255.128 |
| /26 | 255.255.255.192 |
| /27 | 255.255.255.224 |
| /28 | 255.255.255.240 |
| /29 | 255.255.255.248 |
| /30 | 255.255.255.252 |

Tipp: Das letzte relevante Oktett ergibt sich aus `256 − Blockgröße`.

## 5. Wie die IHK das typischerweise abfragt

In den Altprüfungen (z. B. GA2-Aufgaben zu Netzplanung) wird das Thema meist eingebettet in ein Praxisszenario: Eine Firma bekommt ein Adressband zugewiesen (z. B. `192.168.0.0/24`) und soll es in mehrere gleich große oder unterschiedlich große Teilnetze für verschiedene Abteilungen aufteilen (VLSM). Häufig gefragt:

- Netzadresse und Broadcast-Adresse eines gegebenen Subnetzes bestimmen
- Anzahl maximal möglicher Hosts pro Abteilung berechnen
- Passende Subnetzmaske/Präfix für eine geforderte Hostanzahl finden ("mindestens 30 Hosts" → welches Präfix reicht gerade so?)
- Adressbereiche tabellarisch eintragen (VLSM-Tabelle)

## 6. Übungsaufgabe (im Prüfungsstil, selbst erstellt)

> Die IT-Beispiel GmbH erhält das Adressband `10.0.0.0/23` und möchte es in vier gleich große Subnetze für die Abteilungen Verwaltung, Vertrieb, Produktion und Gäste-WLAN aufteilen.
>
> a) Wie viele Hostbits stehen in jedem der vier Subnetze zur Verfügung?
> b) Geben Sie Netzadresse, Broadcast-Adresse und nutzbaren Hostbereich für das zweite Subnetz an.
> c) Wie viele nutzbare Hosts stehen pro Subnetz zur Verfügung?

<details>
<summary>Lösung (aufklappen)</summary>

a) `/23` hat 9 Hostbits (32−23). Für 4 gleich große Subnetze brauchen wir 2 zusätzliche Netzbits (2² = 4) → neues Präfix `/25`, verbleibende Hostbits = 32 − 25 = **7 Bit**

b) Blockgröße = 2^7 = 128
   - Subnetz 1: 10.0.0.0 – 10.0.0.127
   - **Subnetz 2: Netzadresse 10.0.0.128, Broadcast 10.0.0.255, nutzbarer Bereich 10.0.0.129 – 10.0.0.254**

c) 2^7 − 2 = **126 nutzbare Hosts pro Subnetz**

</details>

## 7. Selbstcheck – kannst du das?

- [ ] Ich kann aus IP + Präfix ohne Rechner die Netz- und Broadcast-Adresse bestimmen
- [ ] Ich kann eine Hostanzahl-Anforderung in das passende Präfix übersetzen
- [ ] Ich kann ein Adressband in mehrere gleich große Subnetze aufteilen (VLSM)
- [ ] Ich kann die Subnetzmaske in Dezimalschreibweise umrechnen

## 8. Weiterführende Ressourcen

- **Subnetting-Rechner + Lexikon-Artikel** auf ausbildung-in-der-it.de – gut, um eigene Rechnungen gegenzuprüfen: https://ausbildung-in-der-it.de/
- **YouTube: "Subnetting deluxe" (IHK-Prüfungsvorbereitung)** – erklärt Subnetting speziell für die IHK-Prüfung inkl. Übungsaufgaben
- **YouTube: IPv4 Subnetting nach der "Ickler-Methode"** – alternative Rechenmethode, falls dir die klassische Methode nicht liegt
- Übe zusätzlich mit den **echten Altklausuren** in deinem Repo (GA2_SI_*.pdf) – die Netzplanungsaufgaben (meist Aufgabe 1 oder 2) enthalten fast immer Subnetting

---
*Nächste Lektion: Verschlüsselung & Hashing (GA1)*
