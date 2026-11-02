# Blue-Green Deployment

## Konzept

Beim **Blue-Green Deployment** werden zwei identische Produktionsumgebungen betrieben: **Blue** (aktuell live, stabile Version) und **Green** (neue Version). Nach erfolgreichen Tests wird der gesamte Live-Traffic von Blue auf Green umgeschaltet.

## Funktionsweise

1. **Ausgangssituation:** Blue läuft produktiv und empfängt den gesamten Traffic
2. **Deployment:** Neue Version wird in der Green-Umgebung aufgesetzt
3. **Testen:** Funktionstests/Smoke-Tests direkt in Green
4. **Umschaltung:** Load Balancer leitet den gesamten Traffic von Blue auf Green um
5. **Verifikation & Rollback:** Bei Stabilität bleibt Green live. Blue wird als schnelle Rollback-Option vorgehalten

## Vorteile

- **Nahezu keine Downtime**
- **Schneller Rollback** (einfaches Zurückschalten)
- **Test in produktionsnaher Umgebung** vor Freigabe
- **Geringeres Risiko**

## Nachteile

- **Doppelte Infrastruktur** → höhere Kosten
- **Datenbank-Migrationen** müssen abwärtskompatibel sein
- **Komplexität bei geteilten Ressourcen (Datenbank, Sessions)**

## Vergleich

| Strategie | Rollback | Ressourcenverbrauch | Downtime |
|---|---|---|---|
| **Blue-Green** | Sehr schnell (Umschaltung) | Hoch (doppelte Umgebung) | Minimal |
| **Canary** | Schnell | Geringer | Minimal |
| **Rolling** | Langsamer | Gering | Minimal bis gering |

## IHK-Merksätze

- **Blue = Live, Green = neue Version**
- **Umschaltung über Load Balancer**
- **Rollback = Zurückschalten auf Blue**
- **Hauptnachteil = doppelte Umgebung**

## Verwandt

- [Load Balancer](load-balancer.md)
- [HA-Hochverfügbarkeit](ha-hochverfuegbarkeit.md)
- [Skalierbarkeit-Elastizität](skalierbarkeit-elastizitaet.md)
