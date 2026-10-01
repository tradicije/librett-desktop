# Doprinos projektu LibreTT

[English](CONTRIBUTING.md)

Dobrodošli su primeri stvarnih turnira, pravilnici, predlozi toka rada,
dokumentacija, prevodi, prijave grešaka i implementacija kada razvoj počne.
Projekat je u ranom razvoju; zahtevi i komande za provere nalaze se u
[razvojnom uputstvu](docs/sr/DEVELOPMENT.md).

Za ranjivosti prati [bezbednosnu politiku](SECURITY-sr.md), bez objavljivanja
osetljivih detalja u javnom issue-u.

## Pre izmene

Pročitaj [README](README-sr.md) i
[plan aplikacije](docs/DESKTOP_TOURNAMENT_PLAN_SR.md). Opiši problem, predloženo
ponašanje i konkretan primer. Buduće funkcije označi kao planirane; nemoj ih
predstavljati kao implementirane.

Za poslovna pravila prvo definiši ispravne, neispravne i rubne slučajeve.
Arhitektonske promene zabeleži odlukom u `docs/adr/` pre implementacije.

## Principi dizajna

- Osnovni rad turnira mora biti moguć bez interneta i naloga.
- Pravila domene ne zavise od Taurija, SQL-a, HTTP-a ili korisničkog interfejsa.
- Desktop i companion koriste iste proverene slučajeve korišćenja.
- Čuvamo potvrđene rezultate i istoriju ispravki; izvedene tabele obnavljamo.
- Čuvamo pravilnik i podatke igrača korišćene na istorijskom takmičenju.
- Prijava, dolazak i plaćanje imaju odvojenu evidenciju.
- Ručni i automatski žreb koriste iste provere strukture.
- Podaci su prenosivi; migracije, rezervne kopije i oporavak su dokumentovani.
- Pristup podacima zajednice ne sme zavisiti od opstanka LibreTT-a.

## Dokumentacija i jezici

Održavaj `README.md` i `README-sr.md` usklađenim. Uputstva za doprinose i
pogođenu korisničku dokumentaciju održavaj na engleskom i srpskom.
Izmene u `CHANGELOG.md` piši na engleskom, u odeljku `Unreleased` do izdanja.

Sve korisničke funkcije podržavaju srpski i engleski, uključujući validaciju
i štampu. Koristi stabilne prevodne ključeve; prevedeni statusi se ne čuvaju
kao domenski podaci. Nazivi koje korisnik unosi nisu prevodni tekstovi.

## Provera i pregled

Kada implementacija počne, dodaj smislene testove poslovnih pravila i regresija.
Promene čuvanja podataka proveri kroz migracije i oporavak. Izmene specifične
za platformu proveri na pogođenim operativnim sistemima. Razvojno okruženje i
tačne komande za provere dokumentuju se kada uvedemo alate.

Predložena izmena treba da objasni svrhu, novo ponašanje, izvršene provere i
preostala ograničenja. Uključi relevantnu dokumentaciju i changelog.
Nepovezane izmene izdvoji.

Za primere i prijave grešaka koristi izmišljene ili anonimizovane podatke
igrača. Navedi korake, očekivano i stvarno ponašanje, sistem i verziju.

## Licenca i autorstvo

Autor projekta LibreTT je Aleksa Dimitrijević. Licenca je
`AGPL-3.0-or-later`; pogledaj [LICENSE](LICENSE). Doprinosi moraju biti
kompatibilni sa tom licencom. Sačuvaj postojeće autorstvo i navedi poreklo
i licencu materijala trećih strana.
