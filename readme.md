# IHK Prüfung

Alle Infos werden als eigenes Thema abgelegt:

- `learningtree/IT/<titel>.md`
- `learningtree/Wirtschaft/<titel>.md`

`learningtree/IT.md` und `learningtree/Wirtschaft.md` sind nur die Übersicht
mit Links auf die Themen. Keine Nummerierung bei Dateien oder Lektionen.

## inputer

Rust-Tool. Jede Eingabe landet als Roh-Text in `archive/rawinfos/` und als
strukturierter Eintrag in `archive/jsonformat/`, daraus werden die
Themen-Dateien unter `learningtree/` gebaut.

```
cd inputer
cargo run            # neue Eingabe: Fach wählen, Titel, Text (Ende: ACATS)
cargo run -- build   # archive/jsonformat/ -> learningtree/ (alles neu bauen)
```

Bei jedem `cargo run` wird die Themen-Datei direkt angelegt und in
`IT.md`/`Wirtschaft.md` verlinkt. `build` ist nur zum Neu-Bauen da, wenn
du die JSON-Dateien manuell geändert hast.

Die Dateien unter `learningtree/` sind generiert — dort nicht manuell
editieren, sondern nur über `cargo run` (add) und `cargo run -- build`.
