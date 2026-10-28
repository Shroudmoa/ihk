# 1. Cloud-Computing

*Quelle: GA1 Aufgabe 1 (2021 Winter, 2022 Winter, 2023 Winter, 2024 Sommer,
2026 Sommer u. a.)*

> ✅ **Verifiziert am echten Prüfungskorpus:** Cloud Computing kam in
> **5 von 10 Jahrgängen** (2021W, 2022W, 2023W, 2024S, 2026S) vor.
> - **2021W:** SaaS/PaaS/IaaS-Definitionen, Public/Private-Argumente,
>   "Measured Service" (nutzungsgerechte Abrechnung).
> - **2022W:** "On Premises" vs. "Public Cloud" (je 3 Vorteile),
>   Latenz-Ursachen bei Cloud-Datenbank-Zugriff.
> - **2023W:** SaaS-Migration einer Private Cloud.
> - **2024S:** Vor-/Nachteile (Skalierbarkeit, Kostenreduktion vs.
>   Sicherheitsbedenken, Abhängigkeit).
> - **2026S:** Alle 3 Servicemodelle inkl. konkreter Beispiele (AWS EC2,
>   Azure App Service, Microsoft 365) **und** Deployment-Modelle
>   (Private/Public/Hybrid), Empfehlung Private Cloud bei hohen
>   Compliance-Anforderungen.

## 1.1 Vorteile einer Cloud-Lösung (oft 6–8 Punkte)

Lerne diese Standardargumente mit **je einem konkreten Grund**:

- **Skalierbarkeit:** Ressourcen (Rechenleistung, Speicher) können bei Bedarf
  schnell erweitert/reduziert werden, ohne eigene Hardware kaufen zu müssen.
- **Zeiteinsparung:** Keine Beschaffung, kein Rack-Einbau, kein OS-Setup –
  Dienste sind schnell bereit.
- **Kostenreduktion:** Keine eigenen Hardware-/Personalkosten;
  OPEX statt CAPEX; nur nutzungsabhängige Zahlung.
- **Weltweiter Zugriff:** 24/7-Zugriff für verteilte Standorte/Mitarbeiter.

## 1.2 Nachteile einer Cloud-Lösung

- **Sicherheitsbedenken:** Daten liegen beim Provider, Abhängigkeit von dessen
  Schutzmaßnahmen.
- **Zuverlässigkeit/Ausfallsicherheit:** Hängt am SLA des Providers; bei dessen
  Ausfall ist man selbst offline.
- **Datenschutz/Compliance:** Personenbezogene Daten müssen DSGVO-konform
  verarbeitet werden (z. B. Auftragsverarbeitungsvertrag nötig).

## 1.3 Public Cloud vs. Private Cloud

| | Spricht dafür |
|--|--|
| **Public Cloud** | Geringe Einstiegskosten, hohe Skalierbarkeit, kein eigener Serverraum nötig |
| **Private Cloud** | Volle Kontrolle/Datensouveränität, höheres Sicherheitsniveau, eigene Richtlinien durchsetzbar |

### 1.3.1 On Premises vs. Public Cloud (2022W – je 3 Vorteile gefordert)

| **On Premises** (eigene Infrastruktur) | **Public Cloud** |
|---|---|
| Volle Kontrolle über Hardware & Daten | Keine Investitionskosten (CAPEX entfällt) |
| Keine Abhängigkeit von Internetverbindung | Schnelle, flexible Skalierung bei Bedarf |
| Keine laufenden Kosten für externen Provider | Kein eigenes Personal für Wartung/Betrieb nötig |

**Latenz-Ursachen bei Cloud-Datenbank-Zugriff (2022W):** Physische Entfernung
zum Rechenzentrum, Anzahl der Netzwerk-Hops/Router, Auslastung der
Internetanbindung, ggf. Verschlüsselungs-Overhead (VPN/TLS).

### 1.3.2 Deployment-Modelle: Private / Public / Hybrid Cloud (2026S)

- **Private Cloud:** Infrastruktur exklusiv für eine Organisation (eigenes
  Rechenzentrum oder dediziert beim Provider).
- **Public Cloud:** Von einem Anbieter bereitgestellt, Ressourcen von
  mehreren Kunden geteilt (Multi-Tenant).
- **Hybrid Cloud:** Kombination aus Private und Public Cloud, z. B.
  sensible Daten in der Private Cloud, skalierbare Lastspitzen in der
  Public Cloud (Cloud-Bursting).

**Prüfungsempfehlung (2026S):** Bei **hohen Compliance-Anforderungen**
(z. B. Gesundheits-/Finanzdaten) wird eine **Private Cloud** empfohlen, da
volle Kontrolle über Datenstandort und Sicherheitsmaßnahmen besteht.

## 1.4 Service-Modelle (SaaS / PaaS / IaaS)

- **SaaS – Software as a Service:** Fertige Software im Browser (z. B.
  E-Mail-Hosting, Office 365, **Microsoft 365**). *Anwender verwaltet nichts.*
- **PaaS – Platform as a Service:** Entwicklungs-/Laufzeitumgebung (z. B.
  Datenbank-Plattform, App-Engine, **Azure App Service**). *Anwender liefert
  nur den Code.*
- **IaaS – Infrastructure as a Service:** Virtuelle Server/Storage/Netz
  (z. B. VM, Festplattenplatz, **AWS EC2**). *Anwender verwaltet OS und
  darüber.*

**Prüfungstipp (Migration):** E-Mail- und Datenbankdienste werden meist als
**SaaS** ausgelagert (schlüsselfertig). Eigenentwicklung eher PaaS/IaaS.

**2023W – Praxisbeispiel:** Migration einer bestehenden **Private Cloud**
auf eine **SaaS**-Lösung (z. B. E-Mail/Groupware wird nicht mehr selbst
betrieben, sondern als fertiger Dienst bezogen).

## 1.5 Measured Service (nutzungsgerechte Abrechnung)

Beispiel: Eine VM wird **pro Stunde Laufzeit** oder Speicher **pro GB/Monat**
abgerechnet – der Kunde zahlt exakt das, was er nutzt (statt einer
Pauschale/Investition).

## 1.6 Provider-Auswahl (Anforderungen an den Anbieter)

Nenne mindestens drei:

- Skalierbare Ressourcen (Flexibilität)
- Sicherheitsmaßnahmen & Compliance (ISO 27001, Verschlüsselung, Auth)
- Klare **SLAs** (Verfügbarkeit, Reaktionszeiten) + 24/7-Support
- Flexible Abrechnungsmodelle

## Typische Prüfungsaufgabe

> „Erläutern Sie, inwiefern durch eine Cloud-Lösung Zeiteinsparung,
> Skalierbarkeit und Kostenreduktion erzielt werden können."

→ Antworte **für jeden der drei Begriffe eine eigene Zeile mit Beispiel**.
Nur den Begriff nennen reicht nicht für volle Punktzahl.
