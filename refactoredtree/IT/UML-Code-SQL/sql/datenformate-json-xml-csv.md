# Datenformate: CSV, JSON, XML (und YAML)

## In 30 Sekunden

- **CSV** (Comma-Separated Values): Tabelle als Text, eine Zeile pro Datensatz, Trennzeichen Komma oder Semikolon. Einfach, klein, aber ohne Datentypen und Hierarchie.
- **JSON** (JavaScript Object Notation): Schlüssel-Wert-Paare, Objekte `{}` und Arrays `[]`, kennt Datentypen. Standard für REST-APIs.
- **XML** (Extensible Markup Language): Elemente mit Start- und End-Tag, Attribute, Namensräume, Validierung per XSD. Ausführlich, aber streng prüfbar.
- **YAML:** einrückungsbasiert, gut lesbar, häufig für Konfigurationen (Docker Compose, Ansible, Kubernetes).
- Geprüft: **CSV-Vorteile und -Nachteil** in **2024W und 2025W**.

## Kernwissen

### Dieselben Daten in drei Formaten

**CSV**

```csv
Hostname;IP;CPU_Last;Online
srv-db01;10.0.10.5;73;true
srv-web01;10.0.20.8;41;true
"srv-file;alt";10.0.10.9;0;false
```

Enthält ein Feld das Trennzeichen, muss es in Anführungszeichen stehen. In Deutschland ist das **Semikolon** üblich, weil das Komma Dezimaltrennzeichen ist.

**JSON**

```json
{
  "server": [
    { "hostname": "srv-db01",  "ip": "10.0.10.5", "cpuLast": 73, "online": true },
    { "hostname": "srv-web01", "ip": "10.0.20.8", "cpuLast": 41, "online": true }
  ]
}
```

JSON-Datentypen: String (`"…"`), Zahl, `true`/`false`, `null`, Objekt `{}`, Array `[]`. Schlüssel stehen immer in doppelten Anführungszeichen, Kommentare sind nicht erlaubt.

**XML**

```xml
<?xml version="1.0" encoding="UTF-8"?>
<serverliste>
  <server online="true">
    <hostname>srv-db01</hostname>
    <ip>10.0.10.5</ip>
    <cpuLast>73</cpuLast>
  </server>
</serverliste>
```

XML ist **wohlgeformt**, wenn es genau ein Wurzelelement hat, jedes Tag geschlossen ist und Tags korrekt verschachtelt sind. Es ist **gültig (valide)**, wenn es zusätzlich einem Schema (XSD oder DTD) entspricht.

**YAML**

```yaml
server:
  - hostname: srv-db01
    ip: 10.0.10.5
    cpuLast: 73
    online: true
```

### Vergleich

| Kriterium | CSV | JSON | XML |
|---|---|---|---|
| Struktur | flache Tabelle | hierarchisch | hierarchisch |
| Datentypen | keine (alles Text) | einfache Typen | per Schema (XSD) |
| Lesbarkeit | sehr gut | gut | mittel (viele Tags) |
| Overhead / Dateigröße | sehr gering | gering | hoch |
| Validierung | keine | JSON Schema (optional) | XSD, DTD |
| Typischer Einsatz | Export/Import, Excel, Massendaten | REST-APIs, Webanwendungen, NoSQL | Office-Formate, SOAP, E-Rechnung, Konfiguration |

### CSV: Vor- und Nachteile (Prüfungsantwort 2024W/2025W)

| Vorteile | Nachteile |
|---|---|
| reiner Text, plattform- und herstellerunabhängig | kein einheitlicher Standard für Trennzeichen, Escaping, Zeilenumbrüche |
| von Excel, LibreOffice, Datenbanken und Skripten direkt lesbar | keine Datentypen: Zahl, Datum und Text sind nicht unterscheidbar |
| sehr geringe Dateigröße, kein Metadaten-Overhead | keine Hierarchien oder Beziehungen abbildbar |
| leicht zu erzeugen (`Console.WriteLine` in eine Datei) | Zeichensatzprobleme (UTF-8 vs. ANSI, Umlaute) |
| ideal für große Mengen gleichartiger Messwerte | führende Nullen gehen beim Öffnen in Excel verloren (`01234` → `1234`) |

### Serialisierung

**Serialisierung** = Objekt im Speicher in ein übertragbares Format umwandeln (z. B. JSON), **Deserialisierung** = zurückwandeln. REST-APIs übertragen meist JSON über HTTP(S), der Header lautet `Content-Type: application/json`.

## Prüfungsfrage

**Frage (angelehnt an 2024W):** Ein Monitoring-Programm soll CPU-Messwerte in eine Datei schreiben, die später in einer Tabellenkalkulation ausgewertet wird. Nennen Sie zwei Vorteile des CSV-Formats und einen Nachteil. Nennen Sie ein Format, das sich besser eignet, wenn die Daten verschachtelt an eine Web-API übergeben werden sollen.

**Musterantwort:** **Vorteile:** CSV ist reiner Text und kann ohne Spezialsoftware von Excel, LibreOffice oder Skripten gelesen werden; außerdem ist die Datei sehr klein, weil keine Formatierungen oder Metadaten gespeichert werden. **Nachteil:** Es gibt keinen verbindlichen Standard für Trennzeichen und Sonderzeichen; enthält ein Wert das Trennzeichen, muss er maskiert werden, und Datentypen gehen verloren. Für verschachtelte Daten an eine Web-API eignet sich **JSON**, weil es Objekte und Arrays abbildet und von REST-Schnittstellen direkt verarbeitet wird.

## Falle

- **CSV "hat keine Struktur":** Es hat eine Struktur, aber nur eine flache (Zeilen und Spalten).
- **JSON mit einfachen Anführungszeichen** (`'name'`) ist ungültig.
- **Letztes Komma in JSON** (`{"a":1,}`) ist ein Syntaxfehler.
- **XML wohlgeformt = gültig:** Gültig ist es erst mit erfolgreicher Schema-Prüfung.
- **YAML-Einrückung mit Tabs:** YAML erlaubt nur Leerzeichen.

## Merksatz

**CSV für Tabellen, JSON für APIs, XML für strenge Verträge, YAML für Konfiguration.**

## Verwandt

- [NoSQL-Datenbanktypen](nosql-datenbanktypen.md) – Dokumenten-DBs speichern JSON
- [DDL, DML, DCL](ddl-dml.md) – Datentypen in Tabellen
- [Datentypen in der Programmierung](../Pseudocode/datentypen.md)
- [Lektion 28: GA1-Praxisthemen](../../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md) – CSV-Dateiformat 2024W
