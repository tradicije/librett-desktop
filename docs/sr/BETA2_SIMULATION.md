# Simulacija turnira pred beta 2

Datum: 7. oktobar 2026.

## Rezultat

Prošlo je svih 41 testova jezgra (5 domena i 36 skladišta), kao i postojeće provere obračuna blagajne i formatiranja izveštaja. U izvršenim scenarijima nije pronađen novi kvar aplikacije. Dodata su dva ponovljiva scenarija u `crates/storage-sqlite/src/workflow_tests.rs`.

## Povezani turnir

- 12 igrača sa srpskim slovima u imenima, četiri kluba i tri kategorije.
- Open: tri grupe po četiri igrača, razdvojeni klubovi, po dva direktna prolaznika i dva automatska lucky loser-a; kompletan nokaut.
- Veterani: sedam igrača, BYE, polufinale, finale i meč za bronzu. Naziv kategorije služi scenariju; ovaj test ne proverava starosne izuzetke.
- Dubl: šest parova, neparna kotizacija od 500,01 RSD, predaja bez igre i kompletan nokaut.
- Potvrđeni dolasci, naplata za više kategorija, ponovljeni isti zahtev bez duple naplate, povraćaj i ponovna uplata.
- Rezultat grupe i bekap, zatvaranje veze sa bazom i ponovno otvaranje; stanje takmičenja i blagajna ostaju isti.
- Predaja tokom meča; promena pobednika polufinala odbijena bez potvrde, uz potvrdu poništeni zavisni rezultati i ponovo odigrani završni mečevi.
- Završavanje svih kategorija i turnira, ponovno otvaranje i završavanje.
- Uvoz završnog bekapa u drugu, novu bazu: isti igrači, konačni plasmani, blagajna i sačuvana istorija; SQLite `integrity_check` vraća `ok`.

Dodatni scenario pravi jednake lucky loser kandidate na granici: aplikacija ostavlja dva mesta nerešena, odbija prerano završavanje, prihvata ručni izbor i omogućava završetak turnira.

Cela test kolekcija dodatno proverava konflikte stolova/igrača između kategorija, tri pravila trećeg mesta, zaključavanje prijava, korpu, starije migracije i odbijanje neispravnih bekapa. Provere izveštaja obuhvataju srpska slova, CSV citiranje, zaštitu od formula i HTML escaping; ne proveravaju stvarnu štampu.

## Ponoviti proveru

```sh
cargo test --workspace --exclude librett-desktop --offline
cargo test -p librett-storage-sqlite beta2_full_event_simulation_on_disk --offline -- --nocapture
npm run test:cash
npm run test:reports
```

## Granice provere

Simulacija poziva stvarni aplikacioni i SQLite kod na privremenim podacima. Korisnikova aktivna baza nije korišćena niti menjana. Nije izvedeno klikanje kroz desktop interfejs, instaliranje paketa na svim operativnim sistemima, nasilno prekidanje procesa tokom upisa ni fizička štampa. Druga baza proverava prenos podataka, ali sama po sebi ne predstavlja probu na drugom OS-u. Zatvaranje/otvaranje SQLite veze proverava čuvanje stanja, ne kompletan restart desktop aplikacije.
