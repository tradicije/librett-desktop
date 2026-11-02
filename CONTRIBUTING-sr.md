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

## Struktura koda i testovi

Prati [strukturu projekta i pravila modula](docs/sr/DEVELOPMENT.md#struktura-projekta-i-pravila-modula).

- Frontend ekrane i pomoćne funkcije drži u `apps/desktop/src/lib/features/<feature>/`,
  zajedničke kontrole u `components/`, dijaloge u `dialogs/`, a tipizovane IPC
  pozive u `api/<feature>.ts`. Između modula koristi `$lib/...` i uskladi Vite
  i TypeScript alias.
- Tauri komande pripadaju `src-tauri/src/commands/`, pokretanje `main.rs` fajlu,
  a zajedničko desktop stanje `state.rs` fajlu. Operacije i ugovori aplikacije
  pripadaju modulima uz eksplicitne javne izvoze iz `application/src/lib.rs`.
- Storage integracioni testovi pripadaju `crates/storage-sqlite/tests/`, koriste
  javni API i izolovane baze. Provere privatne konekcije ostaju jedinični testovi;
  nemoj širiti vidljivost produkcionog koda samo radi pristupa iz testova.
- Frontend regresione testove piši uz odgovarajući modul kao `*.test.ts` i
  pokreći kroz Vitest. Uvozi stvarni TypeScript kod bez skripti koje čitaju
  izvore. `npm test` pokreće sve testove, `npm run test:watch` prati izmene,
  a `npm run test:cash` i `npm run test:reports` pokreću pojedinačne grupe.
- Prihvatne specifikacije pripadaju `docs/specs/`, a arhitektonske odluke
  `docs/adr/`. Kada menjaš strukturu, pravila modula ili komande za proveru,
  ažuriraj razvojna uputstva i uputstva za doprinos na oba jezika.

Pre pregleda pokreni `npm run check`, `npm test`, `npm run build`,
`cargo fmt --all --check` i `npm run test:core`. Uz sistemske zavisnosti pokreni
i `cargo check --locked --workspace` i `cargo test --locked --workspace`.

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
