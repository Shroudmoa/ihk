# SQL Grundlagen

## SELECT

```sql
SELECT spalte1, spalte2 FROM tabelle;
SELECT * FROM tabelle;
```

## WHERE

```sql
SELECT * FROM kunde WHERE ort = 'Berlin';
```

## JOIN

```sql
SELECT k.name, a.artikelnummer 
FROM kunde k 
JOIN auftrag a ON k.kunden_id = a.kunden_id;
```

## GROUP BY

```sql
SELECT ort, COUNT(*) AS anzahl 
FROM kunde 
GROUP BY ort;
```
