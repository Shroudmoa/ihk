# IDS und IPS

## In 30 Sekunden

- **IDS** (Intrusion Detection System) **erkennt und meldet** Angriffe – es greift nicht ein.
- **IPS** (Intrusion Prevention System) steht **inline** im Datenstrom und kann Pakete **aktiv verwerfen** oder Verbindungen trennen.
- Erkennung **signaturbasiert** (bekannte Muster) oder **anomaliebasiert** (Abweichung vom Normalverhalten).
- **NIDS/NIPS** überwachen das Netz, **HIDS** einen einzelnen Host.
- In modernen **NGFW** ist ein IPS meist integriert (Prüfung 2025S: zwei Vorteile einer NGFW).

## Kernwissen

| Merkmal | IDS | IPS |
|---|---|---|
| Position | passiv, bekommt eine **Kopie** des Verkehrs (Mirror-/SPAN-Port, TAP) | **inline** zwischen den Netzen |
| Reaktion | Alarm, Log, Meldung an SIEM | Paket verwerfen, Verbindung zurücksetzen, Quelle sperren |
| Risiko bei Fehlalarm | nur unnötige Meldung | **legitimer Verkehr wird blockiert** |
| Ausfall des Systems | Netz läuft weiter, Überwachung fehlt | Netz ggf. unterbrochen (Fail-open/Fail-closed festlegen) |

**Erkennungsmethoden**

| Methode | Funktionsweise | Stärke | Schwäche |
|---|---|---|---|
| **Signaturbasiert** | vergleicht Pakete mit bekannten Angriffsmustern | wenige Fehlalarme, schnell | erkennt keine unbekannten Angriffe (Zero-Day) |
| **Anomaliebasiert** | lernt Normalverhalten, meldet Abweichungen | findet auch Neues | mehr Fehlalarme, Lernphase nötig |

**Fehlerarten:** **False Positive** = Alarm ohne Angriff (stört den Betrieb), **False Negative** = Angriff ohne Alarm (gefährlich).

**Varianten**
- **NIDS/NIPS:** am Netzübergang oder vor wichtigen Serversegmenten.
- **HIDS/HIPS:** Software auf dem Server, überwacht Logs, Dateiintegrität, Prozesse.
- **WAF:** spezialisiert auf Webanwendungen (SQL-Injection, XSS).
- **EDR:** Erkennung und Reaktion direkt auf Endgeräten.

**Grenzen:** Verschlüsselter Verkehr ist ohne **TLS-Inspection** nicht einsehbar; Insider mit gültigen Rechten fallen oft nicht auf; Alarme nützen nur, wenn jemand sie auswertet (SIEM, Bereitschaft).

## Prüfungsfrage

**Frage:** Erläutern Sie den Unterschied zwischen einem IDS und einem IPS und nennen Sie je einen Nachteil.

**Musterantwort:** Ein IDS analysiert eine Kopie des Netzverkehrs, erkennt Angriffsmuster und meldet sie, greift aber selbst nicht ein. Ein IPS steht direkt im Datenstrom und kann erkannte Angriffe sofort blockieren. Nachteil des IDS: Es reagiert nicht selbst, ein Angriff läuft weiter, bis ein Administrator handelt. Nachteil des IPS: Fehlalarme blockieren legitimen Verkehr, und ein Ausfall des IPS kann die Verbindung unterbrechen.

## Falle

- „IDS blockiert Angriffe“ → nein, das macht nur das **IPS**.
- Signaturbasiert als Schutz gegen Zero-Day-Angriffe nennen → passt nicht, dafür eher Anomalieerkennung.
- IDS/IPS mit Firewall gleichsetzen: die Firewall prüft Regeln (Wer darf wohin?), das IPS prüft **Inhalte auf Angriffsmuster**.

## Merksatz

**„IDS = Alarmanlage, IPS = Türsteher.“**

## Verwandt

- [Firewall-Generationen und NGFW](firewalls-alle-generationen-sicherheitskonzepte-antivirus.md)
- [Firewall](firewall.md)
- [DMZ und Sicherheitszonen](dmz-sicherheitszonen.md) – wo der Sensor sitzt
- [AAA und SIEM](../aaa-siem.md) – Alarme zentral auswerten
- [TLS und HTTPS](../Verschluesselung/tls-https.md) – TLS-Inspection
- [Malware und Ransomware](../hacking/malware-ransomware.md)
- [Lektion 16 Firewall](../../../analyse-der-exams/L16_Firewall.md)
