# Objektorientierte Programmierung (OOP) – Grundlagen

## In 30 Sekunden

- **Klasse** = Bauplan mit Attributen (Daten) und Methoden (Verhalten). **Objekt** = konkrete Instanz einer Klasse (`new`).
- Vier Grundprinzipien: **Kapselung**, **Vererbung**, **Polymorphie**, **Abstraktion**.
- **Kapselung:** Attribute `private`, Zugriff nur über öffentliche Methoden/Properties (Getter/Setter) – schützt vor ungültigen Werten.
- **Vererbung:** Unterklasse übernimmt Attribute und Methoden der Basisklasse ("ist ein"), z. B. `Server : Geraet`.
- Beziehungen: **Assoziation**, **Aggregation** (Teil lebt weiter), **Komposition** (Teil stirbt mit dem Ganzen).
- Geprüft: **Datenkapselung vs. Vererbung** (2022W), **Aggregation vs. Komposition** (2023S).

## Kernwissen

### Klasse, Objekt, Konstruktor

```csharp
public class Geraet {
    private string inventarNr;          // Attribut, gekapselt
    private int baujahr;

    public Geraet(string nr, int jahr) { // Konstruktor
        inventarNr = nr;
        Baujahr = jahr;                  // nutzt die Prüfung im Setter
    }

    public int Baujahr {                 // Property mit Validierung
        get { return baujahr; }
        set {
            if (value < 1990 || value > DateTime.Now.Year)
                throw new ArgumentException("Ungültiges Baujahr");
            baujahr = value;
        }
    }

    public virtual string Beschreibung() {
        return "Gerät " + inventarNr + " (" + baujahr + ")";
    }
}

Geraet g = new Geraet("INV-0815", 2022);   // Objekt erzeugen
```

### Sichtbarkeiten

| C# | UML | Zugriff |
|---|---|---|
| `public` | `+` | von überall |
| `private` | `-` | nur innerhalb der Klasse |
| `protected` | `#` | Klasse und abgeleitete Klassen |
| `internal` | `~` | innerhalb derselben Assembly (UML: Paket) |

### Vererbung und Polymorphie

```csharp
public class Server : Geraet {               // Server "ist ein" Geraet
    public int RamGB { get; set; }

    public Server(string nr, int jahr, int ram) : base(nr, jahr) {
        RamGB = ram;
    }

    public override string Beschreibung() {  // Methode überschreiben
        return base.Beschreibung() + ", " + RamGB + " GB RAM";
    }
}

Geraet[] inventar = { new Geraet("INV-1", 2021), new Server("SRV-1", 2023, 256) };
foreach (Geraet x in inventar) {
    Console.WriteLine(x.Beschreibung());     // Polymorphie: jeweils passende Methode
}
```

- **Polymorphie** ("Vielgestaltigkeit"): Derselbe Methodenaufruf verhält sich je nach tatsächlichem Objekttyp unterschiedlich.
- **Überschreiben (Override):** gleiche Signatur in der Unterklasse ersetzt die Basismethode.
- **Überladen (Overload):** gleicher Methodenname, aber andere Parameter in derselben Klasse.
- **Abstrakte Klasse:** kann nicht instanziiert werden, gibt Methoden vor (`abstract`).
- **Interface:** reiner Vertrag ohne Implementierung (`interface IDruckbar`); eine Klasse kann mehrere Interfaces implementieren, aber in C# nur von einer Klasse erben.

### Beziehungen zwischen Klassen

| Beziehung | Bedeutung | UML-Symbol | Beispiel |
|---|---|---|---|
| Assoziation | kennt / nutzt | Linie | Mitarbeiter – Laptop |
| Aggregation | "hat", Teil existiert unabhängig | leere Raute am Ganzen | Team – Spieler, Gebäude – Mieter |
| Komposition | "besteht aus", Teil existiert nicht ohne Ganzes | gefüllte Raute am Ganzen | Gebäude – Raum, Rechnung – Rechnungsposition |
| Vererbung | "ist ein" | Linie mit leerem Dreieck zur Basisklasse | Server – Gerät |

### Vorteile der OOP

- **Wiederverwendbarkeit** durch Vererbung und Klassenbibliotheken.
- **Wartbarkeit:** Änderungen bleiben innerhalb einer Klasse.
- **Datensicherheit** durch Kapselung und Validierung.
- **Realitätsnähe:** Klassen entsprechen fachlichen Objekten.

## Prüfungsfrage

**Frage (angelehnt an 2022W):** Erläutern Sie die Begriffe Datenkapselung und Vererbung jeweils an einem Beispiel aus der IT-Inventarverwaltung.

**Musterantwort:** **Datenkapselung** bedeutet, dass die Attribute einer Klasse nach außen verborgen (`private`) sind und nur über öffentliche Methoden gelesen oder geändert werden können. Beispiel: Die Klasse `Geraet` speichert `baujahr` privat; der Setter `Baujahr` prüft, dass das Jahr nicht in der Zukunft liegt. So können keine ungültigen Werte gespeichert werden. **Vererbung** bedeutet, dass eine abgeleitete Klasse Attribute und Methoden einer Basisklasse übernimmt und erweitern kann. Beispiel: `Server` erbt von `Geraet` die Inventarnummer und das Baujahr und ergänzt das Attribut `RamGB`. Gemeinsamer Code muss nur einmal geschrieben werden.

## Falle

- **Klasse und Objekt verwechselt:** Die Klasse ist der Bauplan, das Objekt das gebaute Haus.
- **Kapselung = Verschlüsselung:** Nein, Kapselung ist Zugriffsschutz im Code.
- **Aggregation/Komposition vertauscht:** Frage "Kann das Teil ohne das Ganze existieren?" – ja = Aggregation, nein = Komposition.
- **Raute an der falschen Seite:** Sie sitzt immer am **Ganzen**.
- **Überladen und Überschreiben vertauscht:** Überschreiben braucht Vererbung, Überladen nicht.
- **Vererbung bei "hat ein":** Ein `Auto` erbt nicht von `Motor`, es **hat** einen Motor (Komposition).

## Merksatz

**"Kapseln schützt, Vererben spart, Polymorphie variiert."** Ist-ein → Vererbung, hat-ein → Aggregation oder Komposition.

## Verwandt

- [Klassendiagramm](../UML/Klassendiagramm.md) – Klassen und Beziehungen in UML
- [Datentypen](datentypen.md) – Werttyp vs. Referenztyp
- [Fehlerarten & Exceptions](fehlerarten-exceptions.md) – `NullReferenceException`, `ArgumentException`
- [ER-Modell & Kardinalitäten](../sql/er-modell-kardinalitaeten.md) – Klassen persistent speichern
- [Lektion 4: Programmierung](../../analyse-der-exams/04_Programmierung_Grundlagen.md) – Kapselung/Vererbung 2022W, Aggregation/Komposition 2023S
