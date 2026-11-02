# Subnetting IPv4 & IPv6

## In 30 Sekunden

- Subnetting kommt in **jeder GA2 seit 2021** dran: Netz-ID, Broadcast, erste/letzte Host-IP, Maske, Anzahl Hosts oder VLSM-Aufteilung.
- IPv4: **Hosts = 2^Hostbits − 2**, **Subnetze = 2^geliehene Bits**, **Blockgröße = 256 − Maskenwert** im „interessanten“ Oktett.
- **VLSM:** größtes Netz zuerst vergeben, dann absteigend – jedes Netz beginnt auf einem Vielfachen seiner Blockgröße.
- IPv6: Subnetze = **2^(neuer Präfix − alter Präfix)**; bei SLAAC muss jedes Subnetz ein **/64** sein.
- Rechenweg **immer aufschreiben** – die IHK vergibt Teilpunkte für den Weg, nicht nur für das Ergebnis.

## Kernwissen

### 1. Zweierpotenzen (auswendig!)

| 2^n | Wert | 2^n | Wert |
|---|---:|---|---:|
| 2^1 | 2 | 2^9 | 512 |
| 2^2 | 4 | 2^10 | 1.024 |
| 2^3 | 8 | 2^11 | 2.048 |
| 2^4 | 16 | 2^12 | 4.096 |
| 2^5 | 32 | 2^13 | 8.192 |
| 2^6 | 64 | 2^14 | 16.384 |
| 2^7 | 128 | 2^15 | 32.768 |
| 2^8 | 256 | 2^16 | 65.536 |

Bitwerte eines Oktetts: `128 64 32 16 8 4 2 1` → daraus ergeben sich die Maskenwerte `128, 192, 224, 240, 248, 252, 254, 255`.

### 2. Präfix-Tabelle IPv4

| CIDR | Maske | Blockgröße (Oktett) | Nutzbare Hosts |
|---:|---|---|---:|
| /8 | 255.0.0.0 | 1. Oktett | 16.777.214 |
| /12 | 255.240.0.0 | 16 im 2. Oktett | 1.048.574 |
| /14 | 255.252.0.0 | 4 im 2. Oktett | 262.142 |
| /16 | 255.255.0.0 | 3. Oktett | 65.534 |
| /19 | 255.255.224.0 | 32 im 3. Oktett | 8.190 |
| /20 | 255.255.240.0 | 16 im 3. Oktett | 4.094 |
| /22 | 255.255.252.0 | 4 im 3. Oktett | 1.022 |
| /23 | 255.255.254.0 | 2 im 3. Oktett | 510 |
| /24 | 255.255.255.0 | 256 (4. Oktett) | 254 |
| /25 | 255.255.255.128 | 128 | 126 |
| /26 | 255.255.255.192 | 64 | 62 |
| /27 | 255.255.255.224 | 32 | 30 |
| /28 | 255.255.255.240 | 16 | 14 |
| /29 | 255.255.255.248 | 8 | 6 |
| /30 | 255.255.255.252 | 4 | 2 |
| /31 | 255.255.255.254 | 2 | 2 (nur Punkt-zu-Punkt, RFC 3021) |
| /32 | 255.255.255.255 | 1 | 1 (Host-Route) |

### 3. Rechenweg: Netz-ID, Broadcast, Hostbereich

**Schema (Blockgrößen-Methode)**
1. Präfix → Maske. Das Oktett, in dem die Maske nicht 255 oder 0 ist, ist das **interessante Oktett**.
2. Blockgröße = 256 − Maskenwert in diesem Oktett.
3. Netz-ID = größtes Vielfaches der Blockgröße, das ≤ dem Wert im interessanten Oktett ist; alle Oktette dahinter = 0.
4. Broadcast = nächste Netz-ID − 1 (alle Hostbits = 1).
5. Erste Host-IP = Netz-ID + 1, letzte Host-IP = Broadcast − 1.

**Beispiel A – 4. Oktett: `203.0.113.180/27`** (GA2 Sommer 2022)
- /27 → Maske `255.255.255.224`, Blockgröße 256 − 224 = **32**.
- Vielfache von 32: 0, 32, 64, 96, 128, **160**, 192 … → 180 liegt im Block 160–191.
- Netz-ID `203.0.113.160`, erste Host-IP `.161`, letzte Host-IP `.190`, Broadcast `.191`.

**Beispiel B – 3. Oktett: `172.16.37.200/20`**
- /20 → Maske `255.255.240.0`, interessantes Oktett = 3., Blockgröße 256 − 240 = **16**.
- Vielfache von 16 im 3. Oktett: 0, 16, **32**, 48 … → 37 liegt im Block 32–47.
- Netz-ID `172.16.32.0`, Broadcast `172.16.47.255`.
- Hostbereich `172.16.32.1` – `172.16.47.254`, Hosts = 2^12 − 2 = **4.094**.

**Beispiel C – binär (zur Kontrolle, logisches UND)**
```
IP     192.168.1.70   = 11000000.10101000.00000001.01000110
Maske  /26            = 11111111.11111111.11111111.11000000
UND  → Netz-ID        = 11000000.10101000.00000001.01000000 = 192.168.1.64
Hostbits auf 1 → Bc   = 11000000.10101000.00000001.01111111 = 192.168.1.127
```

### 4. Gleich große Subnetze bilden

Formel: benötigte Subnetzbits n mit **2^n ≥ Anzahl Subnetze**; neuer Präfix = alter Präfix + n.

**Beispiel (GA2 Winter 2024): `172.16.0.0/12` in 4 gleich große Netze für VLAN 10–40**
1. 4 Netze → 2^2 = 4 → **2 Bits** leihen → /12 + 2 = **/14**, Maske `255.252.0.0`.
2. Blockgröße im 2. Oktett: 256 − 252 = **4**.
3. Netze: `172.16.0.0/14`, `172.20.0.0/14`, `172.24.0.0/14`, `172.28.0.0/14`.
4. Gateway als letzte nutzbare IP: `172.19.255.254`, `172.23.255.254`, `172.27.255.254`, `172.31.255.254`.

**Umgekehrt – Hosts vorgegeben:** Hostbits h mit **2^h − 2 ≥ Hosts**; Präfix = 32 − h.
50 Geräte → 2^6 − 2 = 62 ≥ 50 → h = 6 → **/26**.

### 5. VLSM – Schritt für Schritt

**Aufgabe (GA2 Winter 2021):** `172.16.102.0/24` für vier Abteilungen mit **80, 50, 20 und 10 Hosts** aufteilen, ohne Adressen zu verschwenden.

| Schritt | Abteilung | Hosts | Hostbits | Präfix | Netz-ID | Hostbereich | Broadcast |
|---:|---|---:|---:|---|---|---|---|
| 1 | Abt. 1 | 80 | 7 (126) | /25 | 172.16.102.0 | .1 – .126 | .127 |
| 2 | Abt. 2 | 50 | 6 (62) | /26 | 172.16.102.128 | .129 – .190 | .191 |
| 3 | Abt. 3 | 20 | 5 (30) | /27 | 172.16.102.192 | .193 – .222 | .223 |
| 4 | IT | 10 | 4 (14) | /28 | 172.16.102.224 | .225 – .238 | .239 |
| – | frei | – | – | /28 | 172.16.102.240 | .241 – .254 | .255 |

**Regeln**
1. Bedarf **absteigend sortieren** (größtes Netz zuerst).
2. Für jedes Netz Hostbits bestimmen (Gateway mitzählen, wenn gefordert!).
3. Nächstes Netz beginnt direkt nach dem Broadcast des vorherigen.
4. Prüfen: Netz-ID muss ein Vielfaches der eigenen Blockgröße sein.
5. Am Ende prüfen, ob alles in den Ausgangsblock passt (hier: 128 + 64 + 32 + 16 = 240 ≤ 256).

### 6. Sonderfälle, die geprüft wurden

| Fall | Erklärung | Prüfung |
|---|---|---|
| **/31 (RFC 3021)** | Punkt-zu-Punkt-Links ohne Netz-/Broadcastadresse → beide Adressen nutzbar. `172.20.0.0/16` in /31 → 2^(31−16) = 2^15 = **32.768** Netze | 2022S |
| **/30** | Klassischer Transfer-/WAN-Link: 2 nutzbare Adressen (z. B. `.65` und `.66`, Bc `.67`) | 2024S, 2025W |
| **/29 DMZ** | `203.0.113.64/29` → Hosts .65–.70, Bc .71 | 2022W |
| **Zweierpotenz-Darstellung** | `10.0.0.0/10` → 2^22 − 2 nutzbare Adressen | 2025W |
| **RFC 1918** | `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16` – Netz und Broadcast nennen können (z. B. 172.31.255.255) | 2024W, 2026S |

### 7. Routenzusammenfassung (Supernetting)

Mehrere zusammenhängende Netze lassen sich zu einer Route zusammenfassen, wenn sie auf einer gemeinsamen Bitgrenze liegen:
`192.168.0.0/24` + `192.168.1.0/24` + `192.168.2.0/24` + `192.168.3.0/24` = **`192.168.0.0/22`** (4 Netze = 2 Bits weniger).

### 8. IPv6-Subnetting

**Grundformeln**
- Anzahl Subnetze = **2^(neuer Präfix − alter Präfix)**
- Adressen pro Subnetz = 2^(128 − Präfix); ein /64 hat 2^64 ≈ 1,8 · 10^19 Adressen.
- Keine Netz-/Broadcastadresse abziehen – IPv6 kennt **keinen Broadcast**.
- Eine Hexziffer = 4 Bit (**Nibble**). Präfixe auf Vielfachen von 4 (/48, /52, /56, /60, /64) sind am einfachsten.

**Beispiel A – SLAAC-Subnetze aus /56** (2022S, 2024W, 2025S)
- `2001:db8:9876::/56` → bis /64 sind 64 − 56 = **8 Bit** frei → 2^8 = **256 Subnetze**.
- Die 8 Bit sind die letzten zwei Hexziffern der 4. Gruppe:

| Subnetz | Adresse |
|---|---|
| 1. | `2001:db8:9876::/64` (= `2001:db8:9876:0000::/64`) |
| 2. | `2001:db8:9876:1::/64` |
| vorletztes (255.) | `2001:db8:9876:fe::/64` |
| letztes (256.) | `2001:db8:9876:ff::/64` |

**Beispiel B – 4 gleich große Netze aus /48** (2024S)
1. `2001:db8:cafe::/48` → 4 Netze = 2 Bit → **/50**.
2. Die 2 Bit sind die obersten Bits der 4. Gruppe: `00xx`, `01xx`, `10xx`, `11xx` → Hexwerte `0`, `4`, `8`, `c` als erste Ziffer.

| Nr. | Bits (4. Gruppe) | Subnetz |
|---|---|---|
| 1 | 00 00 0000 0000 0000 | `2001:db8:cafe::/50` |
| 2 | 01 00 0000 0000 0000 | `2001:db8:cafe:4000::/50` |
| 3 | 10 00 0000 0000 0000 | `2001:db8:cafe:8000::/50` |
| 4 | 11 00 0000 0000 0000 | `2001:db8:cafe:c000::/50` |

**Beispiel C – /48 komplett in /64**: 2^(64−48) = 2^16 = **65.536** Subnetze (die komplette 4. Gruppe `0000`–`ffff`).

## Prüfungsfrage

**Frage (angelehnt an GA2 Winter 2021):** Das Netz `192.168.40.0/24` soll per VLSM für Vertrieb (100 Hosts), Lager (25 Hosts), Verwaltung (12 Hosts) und einen Router-Transferlink (2 Hosts) aufgeteilt werden. Geben Sie für jedes Netz Netz-ID, Präfix, Broadcast und Gateway (erste nutzbare Adresse) an.

**Musterantwort:**
- Vertrieb: 100 Hosts → 2^7 − 2 = 126 → **/25**: Netz `192.168.40.0`, Bc `.127`, GW `.1`.
- Lager: 25 Hosts → 2^5 − 2 = 30 → **/27**: Netz `192.168.40.128`, Bc `.159`, GW `.129`.
- Verwaltung: 12 Hosts → 2^4 − 2 = 14 → **/28**: Netz `192.168.40.160`, Bc `.175`, GW `.161`.
- Transfer: 2 Hosts → **/30**: Netz `192.168.40.176`, Bc `.179`, Adressen `.177` und `.178`.
- Rest ab `192.168.40.180` bleibt frei.

**Frage (angelehnt an GA2 Sommer 2025):** Der Provider vergibt `2001:db8:4711:a00::/56`. Wie viele /64-Netze sind möglich? Nennen Sie das erste und das letzte.

**Musterantwort:** 64 − 56 = 8 Bit → 2^8 = 256 Subnetze. Erstes: `2001:db8:4711:a00::/64`, letztes: `2001:db8:4711:aff::/64` (die letzten zwei Hexziffern der 4. Gruppe laufen von `00` bis `ff`).

## Falle

- **−2 vergessen** oder bei IPv6 fälschlich abgezogen → IPv6 hat keine Broadcastadresse.
- **Gateway nicht mitgezählt:** „30 PCs + Router“ = 31 Adressen → /27 reicht (30 nutzbar) **nicht**, /26 nötig.
- **VLSM nicht absteigend** vergeben → Netze liegen nicht auf ihrer Blockgrenze (z. B. `/26` ab `.32` ist ungültig).
- Blockgröße im **falschen Oktett** angewendet (bei /20 ist das 3. Oktett interessant, nicht das 4.).
- Broadcast bei /16 oder /12: **alle** Oktette hinter dem Netzanteil auf 255 (z. B. `172.31.255.255`).
- IPv6-/50-Netze: `2001:db8:cafe:1::/50` ist **falsch** – die Bits sitzen ganz vorn in der Gruppe (`4000`).
- SLAAC mit Präfix länger als /64 (z. B. /72) funktioniert nicht.

## Merksatz

**„Block = 256 minus Maske, Hosts = 2 hoch h minus 2, groß vor klein.“** – und bei IPv6: **„Bits zählen, 2 hoch Differenz, /64 ist Pflicht für SLAAC.“**

## Verwandt

- [IPv4](ipv4.md) – Adressaufbau, private Bereiche, Übertragungsarten
- [IPv6](ipv6.md) – Notation, Kürzungsregeln, Adresstypen
- [IPv6 SLAAC & DHCPv6](ipv6-slaac-dhcpv6.md) – warum /64 bei SLAAC
- [Routing](../Netzwerk-Administration/routing.md) – Routingtabellen mit den berechneten Netzen
- [VLAN](../Netzwerk-Administration/vlan.md) – ein Subnetz pro VLAN
- [Lektion 06 – IPv4 & Routing](../../analyse-der-exams/06_IPv4_Adressierung_Routing.md)
- [Lektion 08 – IPv6](../../analyse-der-exams/08_IPv6.md)
