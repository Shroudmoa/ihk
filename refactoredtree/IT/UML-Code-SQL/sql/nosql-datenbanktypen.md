# NoSQL-Datenbanktypen

## In 30 Sekunden

- **NoSQL** ("not only SQL") bezeichnet nicht-relationale Datenbanken ohne festes Tabellenschema.
- Vier Haupttypen: **Dokumenten-DB**, **Key-Value-DB**, **spaltenorientierte DB (Wide Column)**, **Graphen-DB**. Dazu oft **Zeitreihen-DB**.
- Stärken: horizontale Skalierung (Scale-out), flexible Schemata, sehr hohe Schreib-/Leselast.
- Schwächen: oft keine Joins, schwächere Konsistenz (**BASE** statt **ACID**), uneinheitliche Abfragesprachen.
- 2022W verlangte, **nicht-relationale Datenbanktypen** zu nennen.

## Kernwissen

| Typ | Datenmodell | Typischer Einsatz | Beispiele |
|---|---|---|---|
| **Dokumentenorientiert** | JSON-/BSON-Dokumente mit beliebiger Struktur | Webanwendungen, Produktkataloge, CMS | MongoDB, CouchDB |
| **Key-Value** | Schlüssel → Wert (wie ein Wörterbuch) | Cache, Sessions, Warenkorb | Redis, Riak, DynamoDB |
| **Spaltenorientiert (Wide Column)** | Zeilen mit dynamischen Spaltenfamilien | sehr große, verteilte Datenmengen, Logs | Apache Cassandra, HBase |
| **Graphen** | Knoten und Kanten mit Eigenschaften | soziale Netze, Empfehlungen, Netzwerktopologien | Neo4j |
| **Zeitreihen (Time Series)** | Messwert + Zeitstempel | Monitoring, IoT-Sensoren, CPU-Last | InfluxDB, Prometheus |

### Beispiel Dokument (MongoDB)

```json
{
  "_id": "K-1001",
  "name": "Müller GmbH",
  "ort": "Augsburg",
  "ansprechpartner": [
    { "name": "Frau Kurz", "telefon": "0821 123456" },
    { "name": "Herr Lang" }
  ]
}
```

Das zweite Element hat kein Feld `telefon` – in einer Dokumenten-DB erlaubt, in einer relationalen Tabelle wäre es NULL.

### Beispiel Key-Value (Redis)

```
SET session:8f3a "user=42;rolle=admin"   EX 1800
GET session:8f3a
```

### Relational vs. NoSQL

| Kriterium | Relational (SQL) | NoSQL |
|---|---|---|
| Schema | fest, vorab definiert | flexibel, schemafrei |
| Beziehungen | Fremdschlüssel, Joins | meist eingebettet oder per Anwendung |
| Konsistenz | **ACID** | häufig **BASE** (Basically Available, Soft state, Eventually consistent) |
| Skalierung | meist vertikal (Scale-up) | horizontal (Scale-out, Sharding) |
| Abfrage | standardisiertes SQL | herstellerspezifisch |

**CAP-Theorem:** Ein verteiltes System kann bei einer Netzwerkpartition nicht gleichzeitig **Konsistenz** und **Verfügbarkeit** garantieren. Viele NoSQL-Systeme wählen Verfügbarkeit und sind "eventually consistent".

## Prüfungsfrage

**Frage:** Ein Unternehmen speichert CPU- und RAM-Messwerte von 500 Servern im Minutentakt. Nennen Sie zwei nicht-relationale Datenbanktypen und begründen Sie, welcher sich für diesen Anwendungsfall eignet.

**Musterantwort:** Nicht-relationale Typen sind z. B. **Dokumentendatenbanken** (MongoDB) und **Key-Value-Datenbanken** (Redis), außerdem Graphen- und Zeitreihendatenbanken. Für Messwerte im Minutentakt eignet sich eine **Zeitreihendatenbank** (z. B. InfluxDB): Sie ist auf hohe Schreibraten mit Zeitstempel optimiert, komprimiert Messreihen stark, bietet Aggregationen über Zeitfenster (Mittelwert pro Stunde) und kann alte Daten automatisch ausdünnen oder löschen (Retention).

## Falle

- **NoSQL = "kein SQL":** Gemeint ist "not only SQL". Manche Systeme bieten SQL-ähnliche Sprachen (z. B. CQL bei Cassandra).
- **NoSQL ist immer schneller:** Nur für passende Zugriffsmuster. Komplexe Auswertungen mit vielen Beziehungen sind relational oft besser.
- **Spaltenorientiert mit relationaler Tabelle verwechseln:** Wide-Column-Stores haben dynamische Spalten je Zeile.
- **ACID bei NoSQL ausschließen:** Einige Systeme (z. B. MongoDB ab 4.0) bieten Transaktionen, aber nicht als Grundprinzip.

## Merksatz

**Dokument, Schlüssel, Spalte, Graph** – vier Wege, Daten ohne Tabellenzwang zu speichern. Relational für Beziehungen und Konsistenz, NoSQL für Masse und Flexibilität.

## Verwandt

- [ER-Modell & Kardinalitäten](er-modell-kardinalitaeten.md) – das relationale Gegenstück
- [Index, Transaktionen & Locking](index-transaktionen-locking.md) – ACID im Detail
- [Datenformate JSON, XML, CSV](datenformate-json-xml-csv.md) – JSON als Dokumentformat
- [Skalierbarkeit & Elastizität](../../FiSi-Administrator/Server/virtualisierung/skalierbarkeit-elastizitaet.md) – Scale-up vs. Scale-out
- [Lektion 28: GA1-Praxisthemen](../../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md) – nicht-relationale DB-Typen 2022W
