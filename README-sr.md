# LibreTT

[English](README.md)

LibreTT je planirana slobodna aplikacija otvorenog koda za organizaciju
stonoteniskih turnira i liga, sa lokalnim radom kao osnovom. LibreTT je naziv
aplikacije i krovni identitet projekta.

**Status: rani razvoj.** Prvi korak implementira lokalno kreiranje i pregled
turnira i dodavanje singl/dubl kategorija sa zasebnim izborom formata. Ceo tok
turnira i instaleri za izdanja još nisu dostupni. Pogledaj
[razvojno uputstvo](docs/sr/DEVELOPMENT.md) za pokretanje i provere.

## Svrha

**Podaci takmičenja pripadaju zajednici koja ih stvara.** Rezultat koji napravi
igrač ili klub mora da ostane dostupan i kada LibreTT prestane da se održava
ili radi. Aplikacija služi očuvanju te istorije, a pristup istoriji ne sme
zavisiti od opstanka aplikacije.

Lokalno vlasništvo, dokumentovan otvoreni izvoz i nezavisno čitljive arhive
su arhitektonski zahtevi. Rezervna kopija sama nije dovoljna: zajednica mora
moći da čita, prenese i obnovi zapise bez pokrenutog LibreTT-a i centralnog
servisa. Arhivski izvoz i vraćanje kopije su planirani, još nisu implementirani.

Klubovima i organizatorima dati alat koji radi lokalno, bez obaveznog naloga,
interneta, aktivacije licence i plaćenih paketa funkcija. Podaci takmičenja
ostaju prenosivi kroz dokumentovane formate i rezervne kopije.

## Planirani obim

Aplikacija ima dva modula: **Turniri** i **Lige**. Razvoj počinje turnirima na
**Linuxu, macOS-u i Windowsu**. Telefonske companion aplikacije za sudije i
gledaoce dolaze kasnije.

- Više turnira, sa više kategorija unutar svakog: singl, dubl, veterani,
  starosne grupe i kategorije koje definiše organizator.
- Direktni jednostruki nokaut ili round-robin grupe pa jednostruki nokaut,
  nezavisno za svaku kategoriju.
- Lokalna baza igrača i prijavljivanje igrača u više kategorija.
- Evidencija najava, dolazaka, kotizacija, uplata i preostalog dugovanja.
- Automatski žreb sa nosiocima, ručno raspoređivanje ili izmena automatskog
  predloga pre potvrde.
- Rezultati, dodela stolova, rangiranje, prolaz i konačni plasman.
- Rezervne kopije, oporavak, uvoz/izvoz, materijal za štampu i prikaz na
  drugom ekranu.
- Srpski i engleski kroz korisnički interfejs i izlazne dokumente.

Buduća integracija sa bazom igrača na **stoni.rs** omogućiće pretragu i
preuzimanje podataka za kasniji offline rad. Baza i njen API još ne postoje.
Lokalni rad neće zavisiti od tog servisa.

## Predlog arhitekture

Predložene tehnologije su **Tauri 2, Rust, TypeScript/Svelte i SQLite**.
Izbor proveravamo ranim prototipom na sva tri sistema, uključujući offline
instalaciju, štampu, vraćanje rezervne kopije i drugi ekran.

Pravila takmičenja pripadaju jezgru nezavisnom od platforme. Desktop i budući
companion adapteri koriste iste slučajeve korišćenja. Potvrđeni rezultati su
izvor istine; tabele mogu ponovo da se izračunaju.

Postojeći LibreTT WordPress projekat i DimiPress Rally služe kao reference.
Ovaj repozitorijum opisuje novu samostalnu aplikaciju. Autor planira da
postojeći WordPress projekat preimenuje u `librett-wordpress`.

## Dokumentacija i doprinosi

- [Plan desktop aplikacije](docs/DESKTOP_TOURNAMENT_PLAN_SR.md)
- [Doprinos projektu](CONTRIBUTING-sr.md)
- [Changelog — engleski](CHANGELOG.md)

Korisnička i tehnička dokumentacija na srpskom i engleskom razvijaće se uz
specifikaciju i implementaciju. Detaljan radni plan trenutno je na srpskom.

## Autor i licenca

Copyright (C) 2026 Aleksa Dimitrijević.

LibreTT je licenciran pod **GNU Affero General Public License, verzija 3
ili bilo koja kasnija verzija** (`AGPL-3.0-or-later`). Pogledaj [LICENSE](LICENSE).

LibreTT se pruža bez garancije; puni uslovi nalaze se u licenci.
