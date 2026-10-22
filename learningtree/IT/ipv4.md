# IPv4

## In 30 Sekunden

- IPv4 ist **32 Bit**, vier Oktette, aufgeteilt in Netz- und Hostanteil.
- Nutzbare Hosts: **2^Hostbits − 2** – Netz- und Broadcastadresse zählen nicht.
- Blockgröße im letzten Oktett = **256 − Maskenwert**.
- Privat nach RFC 1918: `10/8`, `172.16/12`, `192.168/16` – nicht im Internet routbar, NAT nötig.
- Rechnen kommt in **jeder GA2** dran, meist als „Subnetz für 50 Geräte“.

## Kernwissen

**Grundbegriffe**
| Begriff | Bedeutung |
|---|---|
| Adresse | 32 Bit, z. B. `192.168.1.10` |
| Präfix | Anzahl der Netzbites, z. B. `/26` |
| Maske | `255.255.255.192` bei `/26` |
| Hostbits | `32 − Präfix` |

**Die Präfixe, die wirklich drankommen**
| CIDR | Maske | Nutzbare Hosts | Blockgröße |
|---:|---|---:|---:|
| /24 | 255.255.255.0 | 254 | 256 |
| /25 | 255.255.255.128 | 126 | 128 |
| /26 | 255.255.255.192 | 62 | 64 |
| /27 | 255.255.255.224 | 30 | 32 |
| /28 | 255.255.255.240 | 14 | 16 |
| /29 | 255.255.255.248 | 6 | 8 |
| /30 | 255.255.255.252 | 2 | 4 |

**Rechnen in 3 Schritten**
1. Hostbits = 32 − Präfix.
2. Hosts = 2^Hostbits − 2.
3. Netzadresse = nächstes Vielfaches der Blockgröße im letzten Oktett.

**Beispiel `192.168.1.70/26`**
- Blockgröße 64 → Netze beginnene bei `.0`, `.64`, `.128`, `.192`.
- Netzadresse `192.168.1.64`, Hosts `.65`–`.126`, Broadcast `.127`.
- `.0` wäre falsch – die Adresse liegt im zweiten Block.

**Privat vs. öffentlich (RFC 1918)**
| Klasse | Bereich | Adressen |
|---|---|---:|
| A | `10.0.0.0/8` | 16.777.216 |
| B | `172.16.0.0/12` | 1.048.576 |
| C | `192.168.0.0/16` | 65.536 |

`172.16`–`172.31` ist privat, **`172.32` nicht**. Das ist eine der beliebtesten Fallen.

## Prüfungsfrage

**Frage:** Ein `/24`-Netz soll in gleich große Subnetze für 4 Abteilungen mit je mindestens 50 Geräten geteilt werden. Nenne Präfix, Maske und nutzbare Hosts pro Netz.

**Musterantwort:** 50 Geräte brauchen 6 Hostbits, weil 2^6 − 2 = 62 nutzbare Adressen ergibt. Der Präfix ist damit `/26` mit der Maske `255.255.255.192`. Aus dem `/24` werden vier `/26`-Netze mit je 62 nutzbaren Hosts.

## Falle

- Netz- und Broadcastadresse mitzählen → eine Antwort zu viel, Punkt weg.
- `172.32.0.0` als privat annehmen → falsch, privat endet bei `172.31`.
- Bei „Anzahl Subnetze“ die geliehenen Bits vergessen: `/24` → `/26` sind 2 Bits = 4 Netze.

## Merksatz

**„2 hoch Hostbits minus 2“** – und die Blockgröße ist immer **256 minus letzter Maskenwert**.

## Verwandt

- [IPv6](ipv6.md) – 128 Bit, SLAAC, kein Broadcast
- [MAC](mac.md) – die Schicht darunter
- [Netzwerktypen](netzwerktypen.md) – wo Subnetze eingesetzt werden
- [OSI](osi.md) – Einordnung auf Schicht 3
