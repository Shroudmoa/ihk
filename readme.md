# IHK AP2 – Lernunterlagen


Es wird empgohlen glow zu verwenden, um die markdownlernblätter in Terminal zu lesen und bearbeiten.

- https://github.com/charmbracelet/glow


Zwei Lerntrees, ein Thema pro Datei:

- [`learningtree/IT.md`](learningtree/IT.md) 
- [`learningtree/Wirtschaft.md`](learningtree/Wirtschaft.md) 


# IHK AP2 FiSi Lernmaterial (AO2020)

Dieses Repository/Lernverzeichnis ist auf die IHK-Abschlussprüfung AP2 für Fachinformatiker Systemintegration (FiSi) nach AO2020 ausgelegt.

**Die Prüfungen selber + Antworten sind unter https://github.com/Shroudmoa/IHK-Abschlusspruefungen-AP2-FiSi-AO2020 erhältlich**

## Inhalt

- [IT-Lernblätter](learningtree/IT.md) – Kompakte, prüfungsgerechte Lernblätter für den IT-Teil
- [Wirtschaft-Lernblätter](learningtree/Wirtschaft.md) – Lernblätter für den Wirtschaftsteil
- [Lernpfad-Verbindungen](learningtree/Lernpfad-Verbindungen.md) – Querverbindungen zwischen den Themen
- [Prüfungsfragen](learningtree/IT/fragen/) – Ausgewertete Originalprüfungen GA1 & GA2

## Ziel

Schnelle Wiederholung vor der Prüfung mit Fokus auf prüfungsrelevante Themen, Merksätze und typische Aufgabenstellungen.




-----------------------------------------------------











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
