# IHK AP2 – Lernunterlagen

Zwei Lerntrees, ein Thema pro Datei:

- [`learningtree/IT.md`](learningtree/IT.md) – 29 Themen in 7 Kategorien
- [`learningtree/Wirtschaft.md`](learningtree/Wirtschaft.md) – 4 Themen


## inputer

Rust-Tool unter `inputer/`. Es speichert jede Eingabe in
`archive/jsonformat/` und `archive/rawinfos/` und baut daraus
`learningtree/`.

```
cd inputer
cargo run            # neue Eingabe: Fach, Titel, Kategorie, Ein-Satz, Text
cargo run -- build   # archive/jsonformat/ -> learningtree/
cargo run -- check   # doppelte Titel, tote Links, fehlende Ein-Sätze, Waisen
```

- Text endet mit **ACATS** in einer eigenen Zeile.
- Die Kategorie wird aus einem festen Menü gewählt, nicht getippt.
- Bei jedem `add` wird die Themendatei sofort angelegt und im Index verlinkt.

Die Dateien in `learningtree/` sind **generiert**. Nicht dort editieren –
sonst ist die Änderung beim nächsten Build weg. Quelle ist das JSON.

## Ordner

| Pfad | Bedeutung |
|---|---|
| `learningtree/` | generierte Lerntrees (nur lesen) |
| `archive/jsonformat/` | Quelle: strukturierte Einträge (Änderungen hier) |
| `archive/rawinfos/` | Roh-Log aller Eingaben, wird nur angehängt |
| `archive/APT2/` | Kursfolien und Übungslinks |
| `archive/analyses/` | alte V1/V2/V3-Sammlungen (Archiv) |
