# DDL, DML, DCL und TCL

## In 30 Sekunden

- **DDL (Data Definition Language)** legt die **Struktur** fest: `CREATE`, `ALTER`, `DROP`, `TRUNCATE`.
- **DML (Data Manipulation Language)** ändert **Daten**: `INSERT`, `UPDATE`, `DELETE`. `SELECT` wird oft zur DML gezählt, manchmal als eigene **DQL** (Data Query Language).
- **DCL (Data Control Language)** vergibt **Rechte**: `GRANT`, `REVOKE`.
- **TCL (Transaction Control Language)** steuert **Transaktionen**: `BEGIN`/`START TRANSACTION`, `COMMIT`, `ROLLBACK`, `SAVEPOINT`.
- **Constraints** sichern die Datenqualität: `PRIMARY KEY`, `FOREIGN KEY`, `NOT NULL`, `UNIQUE`, `CHECK`, `DEFAULT`.
- `UPDATE` und `DELETE` **ohne WHERE** treffen **alle** Zeilen.
- 2022W verlangte `CREATE TABLE` mit Primär- und Fremdschlüssel sowie die Zuordnung von SQL-Datentypen.

## Kernwissen

### SQL-Datentypen (2022W abgefragt)

| Datentyp | Verwendung | Beispielwert |
|---|---|---|
| `INT` / `INTEGER` | Ganzzahl, IDs, Mengen | `12345` |
| `BIGINT` | große Ganzzahl | Bytezähler |
| `DECIMAL(p,s)` | exakte Dezimalzahl, **Geld** (p Stellen gesamt, s Nachkommastellen) | `DECIMAL(10,2)` → `12345678.90` |
| `FLOAT` / `REAL` | Gleitkommazahl (ungenau) | Messwerte |
| `CHAR(n)` | Text **fester** Länge | PLZ `CHAR(5)` → `'01234'`, IBAN `CHAR(22)` |
| `VARCHAR(n)` | Text **variabler** Länge bis n | Name `VARCHAR(50)` |
| `TEXT` | langer Text | Beschreibung |
| `DATE` | Datum | `'2022-05-11'` |
| `TIME` / `DATETIME` / `TIMESTAMP` | Uhrzeit / Datum + Uhrzeit | `'2022-05-11 14:30:00'` |
| `BOOLEAN` | wahr/falsch | `true` |

**PLZ und Telefonnummer sind Text**, keine Zahlen: führende Nullen (`01234`) und Sonderzeichen (`0123/112233`) gingen sonst verloren. Mit ihnen wird nicht gerechnet.

### DDL: CREATE TABLE mit Constraints

```sql
CREATE DATABASE Inventar;

CREATE TABLE Abteilung (
    AbtID        INT          PRIMARY KEY,
    Bezeichnung  VARCHAR(50)  NOT NULL UNIQUE
);

CREATE TABLE Mitarbeiter (
    MaID         INT           NOT NULL,
    Name         VARCHAR(80)   NOT NULL,
    Email        VARCHAR(100)  UNIQUE,
    Gehalt       DECIMAL(10,2) CHECK (Gehalt >= 0),
    Eintritt     DATE          DEFAULT CURRENT_DATE,
    AbtID        INT,
    PRIMARY KEY (MaID),
    FOREIGN KEY (AbtID) REFERENCES Abteilung(AbtID)
        ON DELETE SET NULL
        ON UPDATE CASCADE
);

-- Zwischentabelle für m:n mit zusammengesetztem Primärschlüssel
CREATE TABLE Mitarbeiter_Projekt (
    MaID       INT,
    ProjektID  INT,
    Stunden    DECIMAL(6,2) DEFAULT 0,
    PRIMARY KEY (MaID, ProjektID),
    FOREIGN KEY (MaID)      REFERENCES Mitarbeiter(MaID) ON DELETE CASCADE,
    FOREIGN KEY (ProjektID) REFERENCES Projekt(ProjektID)
);
```

| Constraint | Wirkung |
|---|---|
| `PRIMARY KEY` | eindeutig **und** nicht NULL, genau einer pro Tabelle (kann aus mehreren Spalten bestehen) |
| `FOREIGN KEY … REFERENCES` | Wert muss im referenzierten Primärschlüssel existieren (**referentielle Integrität**) |
| `NOT NULL` | Pflichtfeld |
| `UNIQUE` | keine Duplikate, NULL meist erlaubt, mehrere pro Tabelle möglich |
| `CHECK (Bedingung)` | Wertebereich prüfen |
| `DEFAULT wert` | Standardwert, wenn beim INSERT nichts angegeben wird |

**Referentielle Aktionen beim Löschen/Ändern des Elterndatensatzes:**

| Aktion | Folge für Kinddatensätze |
|---|---|
| `RESTRICT` / `NO ACTION` (Standard) | Löschen wird verweigert, solange Kinder existieren |
| `CASCADE` | Kinder werden mitgelöscht bzw. mitgeändert |
| `SET NULL` | Fremdschlüssel der Kinder wird NULL |
| `SET DEFAULT` | Fremdschlüssel wird auf Standardwert gesetzt |

### DDL: ALTER, DROP, TRUNCATE

```sql
ALTER TABLE Mitarbeiter ADD Telefon VARCHAR(20);          -- Spalte hinzufügen
ALTER TABLE Mitarbeiter DROP COLUMN Telefon;              -- Spalte entfernen
ALTER TABLE Mitarbeiter RENAME COLUMN Name TO Nachname;   -- umbenennen (PostgreSQL, MySQL 8)
ALTER TABLE Mitarbeiter MODIFY Gehalt DECIMAL(12,2);      -- Typ ändern (MySQL)
ALTER TABLE Mitarbeiter ALTER COLUMN Gehalt DECIMAL(12,2);-- Typ ändern (MS SQL)
ALTER TABLE Mitarbeiter ADD CONSTRAINT uq_email UNIQUE (Email);
ALTER TABLE Mitarbeiter ADD CONSTRAINT fk_abt
    FOREIGN KEY (AbtID) REFERENCES Abteilung(AbtID);

CREATE INDEX idx_ma_name ON Mitarbeiter(Name);            -- Index für schnelle Suche

TRUNCATE TABLE Mitarbeiter_Projekt;   -- alle Zeilen weg, Struktur bleibt, sehr schnell
DROP TABLE Mitarbeiter_Projekt;       -- Tabelle samt Struktur weg
DROP DATABASE Inventar;               -- ganze Datenbank weg
```

| Befehl | Daten weg | Struktur weg | WHERE möglich | Kategorie |
|---|---|---|---|---|
| `DELETE FROM t` | ja (zeilenweise) | nein | ja | DML |
| `TRUNCATE TABLE t` | ja (komplett) | nein | nein | DDL |
| `DROP TABLE t` | ja | ja | nein | DDL |

### DML: INSERT

```sql
INSERT INTO Abteilung (AbtID, Bezeichnung) VALUES (10, 'IT');

-- mehrere Zeilen auf einmal
INSERT INTO Abteilung (AbtID, Bezeichnung)
VALUES (20, 'Vertrieb'), (30, 'Einkauf');

-- Spalten weglassen: Eintritt bekommt DEFAULT, Email wird NULL
INSERT INTO Mitarbeiter (MaID, Name, Gehalt, AbtID)
VALUES (1, 'Klein', 4200.00, 10);

-- INSERT aus einer Abfrage (z. B. Archiv befüllen)
INSERT INTO Mitarbeiter_Archiv (MaID, Name)
SELECT MaID, Name FROM Mitarbeiter WHERE AbtID = 30;
```

`INSERT INTO Mitarbeiter (…, AbtID) VALUES (…, 99)` schlägt fehl, wenn Abteilung 99 nicht existiert: **Fremdschlüsselverletzung**.

### DML: UPDATE

```sql
UPDATE Mitarbeiter SET Gehalt = 4500.00 WHERE MaID = 1;

-- relative Änderung: 3 % mehr für die IT
UPDATE Mitarbeiter SET Gehalt = Gehalt * 1.03 WHERE AbtID = 10;

-- mehrere Spalten
UPDATE Mitarbeiter SET AbtID = 20, Email = 'klein@firma.de' WHERE MaID = 1;
```

### DML: DELETE

```sql
DELETE FROM Mitarbeiter WHERE MaID = 4;
DELETE FROM Mitarbeiter WHERE Eintritt < '2015-01-01' AND AbtID IS NULL;
```

Vor einem `UPDATE`/`DELETE` mit demselben `WHERE` ein `SELECT` ausführen und prüfen, welche Zeilen betroffen sind.

### DCL: GRANT und REVOKE

```sql
CREATE USER 'report'@'%' IDENTIFIED BY 'S1cher!Passwort';   -- MySQL-Syntax

GRANT SELECT ON Inventar.Mitarbeiter TO 'report'@'%';        -- nur lesen
GRANT SELECT, INSERT, UPDATE ON Inventar.* TO 'app'@'10.0.0.%';
GRANT ALL PRIVILEGES ON Inventar.* TO 'dbadmin'@'localhost' WITH GRANT OPTION;

REVOKE INSERT, UPDATE ON Inventar.* FROM 'app'@'10.0.0.%';  -- Rechte entziehen

-- Rollen (PostgreSQL, MS SQL, MySQL 8)
CREATE ROLE leser;
GRANT SELECT ON Mitarbeiter TO leser;
GRANT leser TO report;
```

`WITH GRANT OPTION` erlaubt dem Empfänger, das Recht weiterzugeben: nur sparsam einsetzen (**Least Privilege**). Rechte werden über Rollen statt einzeln an Personen vergeben (**RBAC**).

### TCL: Transaktionen

```sql
START TRANSACTION;                 -- MS SQL: BEGIN TRANSACTION
UPDATE Konto SET Saldo = Saldo - 500 WHERE KontoNr = 1;
UPDATE Konto SET Saldo = Saldo + 500 WHERE KontoNr = 2;
COMMIT;                            -- beide Änderungen dauerhaft
-- bei Fehler stattdessen:
-- ROLLBACK;                       -- beide Änderungen verwerfen
```

Eine Transaktion ist **atomar**: Entweder werden alle Anweisungen ausgeführt oder keine (ACID, siehe [Index, Transaktionen & Locking](index-transaktionen-locking.md)).

### Views

```sql
CREATE VIEW v_ma_it AS
SELECT MaID, Name, Email FROM Mitarbeiter WHERE AbtID = 10;   -- ohne Gehalt!

GRANT SELECT ON v_ma_it TO leser;   -- Leser sieht keine Gehälter
```

Eine View ist eine gespeicherte Abfrage. Sie vereinfacht komplexe Joins und schränkt sichtbare Spalten ein.

## Prüfungsfrage

**Frage (angelehnt an 2022W):** Erstellen Sie die Tabelle `Werbeaktion` mit den Attributen `WerbungsID` (Primärschlüssel), `Werbepartner` (Text bis 50 Zeichen), `Werbung_verschickt` (Datum) und `KundenNr`, die auf die Tabelle `Kunde` verweist. Ergänzen Sie anschließend eine Spalte `Kanal` und vergeben Sie dem Benutzer `marketing` ausschließlich Leserechte auf die Tabelle.

**Musterantwort:**

```sql
CREATE TABLE Werbeaktion (
    WerbungsID          INT NOT NULL,
    Werbepartner        VARCHAR(50),
    Werbung_verschickt  DATE,
    KundenNr            INT,
    PRIMARY KEY (WerbungsID),
    FOREIGN KEY (KundenNr) REFERENCES Kunde(KundenNr)
);

ALTER TABLE Werbeaktion ADD Kanal VARCHAR(20);

GRANT SELECT ON Werbeaktion TO marketing;
```

Der Fremdschlüssel stellt sicher, dass jede Werbeaktion einem existierenden Kunden zugeordnet ist (referentielle Integrität). `GRANT SELECT` setzt das Least-Privilege-Prinzip um.

## Falle

- **UPDATE/DELETE ohne WHERE** → alle Datensätze geändert bzw. gelöscht.
- **DELETE, TRUNCATE und DROP verwechselt:** Nur `DROP` entfernt die Tabelle selbst.
- **Fremdschlüssel vor der Elterntabelle anlegen:** `REFERENCES Abteilung` setzt voraus, dass `Abteilung` existiert. Reihenfolge: erst Eltern-, dann Kindtabelle anlegen; beim Löschen umgekehrt.
- **PLZ als INT:** `01234` wird zu `1234`. Richtig: `CHAR(5)`.
- **Geld als FLOAT:** Rundungsfehler. Richtig: `DECIMAL(10,2)`.
- **Spaltenliste beim INSERT weglassen:** `INSERT INTO t VALUES (…)` hängt von der Spaltenreihenfolge ab und bricht nach `ALTER TABLE`.
- **GRANT ALL für Anwendungen:** widerspricht Least Privilege. Die Anwendung bekommt nur die Rechte, die sie braucht.
- **REVOKE wirkt nicht auf Rollen-Rechte:** Wer das Recht über eine Rolle hat, behält es, bis die Rolle entzogen wird.

## Merksatz

**DDL baut das Haus, DML räumt die Möbel um, DCL verteilt die Schlüssel, TCL entscheidet, ob der Umzug gilt.**

## Verwandt

- [SQL-Grundlagen](grundlagen.md) – SELECT und Aggregation
- [SQL-Joins](joins.md) – Tabellen verknüpfen
- [Index, Transaktionen & Locking](index-transaktionen-locking.md) – ACID, Indizes, Sperren
- [ER-Modell & Kardinalitäten](er-modell-kardinalitaeten.md) – vom Modell zur Tabelle
- [Datentypen in der Programmierung](../Pseudocode/datentypen.md) – Wertebereiche
- [RBAC](../../FiSi-Administrator/Netzwerk-Administration/rbac.md) – Rechte über Rollen
- [Least Privilege](../../FiSi-Administrator/Sicherheit/least-privilege.md)
- [Lektion 14: Datenmodellierung, SQL & UML](../../analyse-der-exams/L14_Datenmodellierung_SQL_UML.md)
