# Blue-Green-Deployment

## In 30 Sekunden

- Zwei **identische** Produktionsumgebungen: **Blue** (live) und **Green** (neue Version).
- Die neue Version wird in Green installiert und getestet, dann schaltet der **Load Balancer** den gesamten Verkehr um.
- **Rollback** = einfach zurückschalten. Vorteil: nahezu keine Downtime. Nachteil: doppelte Infrastruktur.
- Geprüft: Prinzip mit Schaubild (vor, während und nach dem Deployment) 2023S.

## Kernwissen

### Konzept

Beim **Blue-Green Deployment** werden zwei identische Produktionsumgebungen betrieben: **Blue** (aktuell live, stabile Version) und **Green** (neue Version). Nach erfolgreichen Tests wird der gesamte Live-Traffic von Blue auf Green umgeschaltet.

### Funktionsweise

1. **Ausgangssituation:** Blue läuft produktiv und empfängt den gesamten Traffic
2. **Deployment:** Neue Version wird in der Green-Umgebung aufgesetzt
3. **Testen:** Funktionstests/Smoke-Tests direkt in Green
4. **Umschaltung:** Load Balancer leitet den gesamten Traffic von Blue auf Green um
5. **Verifikation & Rollback:** Bei Stabilität bleibt Green live. Blue wird als schnelle Rollback-Option vorgehalten

### Vorteile

- **Nahezu keine Downtime**
- **Schneller Rollback** (einfaches Zurückschalten)
- **Test in produktionsnaher Umgebung** vor Freigabe
- **Geringeres Risiko**

### Nachteile

- **Doppelte Infrastruktur** → höhere Kosten
- **Datenbank-Migrationen** müssen abwärtskompatibel sein
- **Komplexität bei geteilten Ressourcen (Datenbank, Sessions)**

### Vergleich

| Strategie | Rollback | Ressourcenverbrauch | Downtime |
|---|---|---|---|
| **Blue-Green** | Sehr schnell (Umschaltung) | Hoch (doppelte Umgebung) | Minimal |
| **Canary** (erst wenige Nutzer, dann alle) | Schnell | Geringer | Minimal |
| **Rolling** (Server nacheinander aktualisieren) | Langsamer | Gering | Minimal bis gering |

## Prüfungsfrage

**Frage:** Erläutern Sie das Grundprinzip des Blue-Green-Deployments für das Update eines Webshops von Version 1.0 auf 1.1.

**Musterantwort:** Es werden zwei gleichartige Produktionsumgebungen betrieben. Vor dem Deployment leitet der Load Balancer den gesamten Verkehr auf die Umgebung Blue, auf der Version 1.0 läuft. Version 1.1 wird auf der Umgebung Green installiert und dort getestet, ohne dass Kunden betroffen sind. Ist der Test erfolgreich, schaltet der Load Balancer den Verkehr auf Green um. Version 1.1 ist damit ohne nennenswerte Ausfallzeit live. Blue bleibt mit Version 1.0 unverändert bestehen. Treten Fehler auf, wird einfach zurückgeschaltet (Rollback). Später dient Blue als Zielumgebung für das nächste Update.

## Falle

- Blue und Green als „Test und Produktion“ für immer festlegen. Die Rollen **wechseln** nach jedem Deployment.
- Die Datenbank vergessen. Schemaänderungen müssen mit beiden Versionen funktionieren, sonst ist kein Rollback möglich.
- Blue sofort abbauen. Dann ist der schnelle Rollback weg.

## Merksatz

**„Neue Version daneben aufbauen, testen, Schalter umlegen, alter Stand bleibt als Rückweg.“**

## Verwandt

- [Load Balancer](load-balancer.md)
- [HA-Hochverfügbarkeit](ha-hochverfuegbarkeit.md)
- [Skalierbarkeit-Elastizität](virtualisierung/skalierbarkeit-elastizitaet.md)
- [Container & Docker](virtualisierung/container-docker.md)
- [Lektion L28 GA1 Praxisthemen](../../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md)
