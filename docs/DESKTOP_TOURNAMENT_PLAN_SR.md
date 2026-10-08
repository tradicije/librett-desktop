# Plan desktop aplikacije za stonoteniske turnire

**Razvojno ažuriranje 2026-10-08:** Jednosmerni uvoz/osvežavanje registra implementirano je u kodu; vidi [obim/ograničenja](sr/REGISTRY_IMPORT.md). Autentifikovana replikacija, Desktop upload i automatsko spajanje identiteta nisu uključeni.

Datum: 2026-10-01. Status: predlog za zajednički pregled, pre implementacije.
Naziv aplikacije i krovni identitet: LibreTT. Autor će postojeći WordPress
projekat zasebno preimenovati u librett-wordpress.

## 1. Dogovoreni cilj

Desktop aplikacija za Linux, macOS i Windows. Prvo turniri. Telefoni kasnije
služe kao companion za sudije i gledaoce. Organizator može da pripremi i završi
takmičenje bez interneta, naloga, aktivacije ili obaveznog centralnog servisa.
Kod, dokumentovani formati i osnovne funkcije ostaju dostupni zajednici.

Glavna filozofija: podaci ostaju u rukama zajednice koja ih pravi. Rezultat
igrača ili kluba ne sme nestati ako aplikacija prestane da se održava ili radi.
Pre isporuke rezultata definišemo otvoreni verzionisani izvoz izvornih zapisa,
nezavisno čitljivu arhivu i postupak obnove bez aplikacije ili centralnog
servisa. Izvoz nosi pravilnike, stabilne ID-jeve i istoriju potrebnu za
tumačenje rezultata. Provera uključuje čitanje arhive bez LibreTT-a; privatni
podaci su odvojeni od javne sportske istorije.

Aplikacija ima dve funkcionalne grane: Turniri i Lige. To su moduli jednog
proizvoda, ne Git grane ili odvojene aplikacije. Sada specifikujemo i razvijamo
Turnire; Lige kasnije koriste zajedničke igrače, klubove i pravila meča.
Pregled podržava proizvoljan broj turnira bez nametnutog ograničenja.
Jedan turnir sadrži više kategorija: apsolutna singl, dubl, veterani, 40+, 50+
i druge kategorije koje organizator imenuje i konfiguriše.

Ovo je novi projekat. Postojeće WordPress projekte pregledamo kao reference;
ne menjamo njihove ugovore niti u njih ugrađujemo desktop aplikaciju.

## 2. Pregled referenci

Pregledani javni repozitorijumi, dokumentacija i reprezentativne implementacije:

- LibreTT: `7fd71498369e81ffdc763d454ddf90ff009a2862`.
- DimiPress Rally: `9856a17df4c3019a4d43bc964509b28defcee25d`.

Ovo je arhitektonski pregled, a ne potpuni audit ili potvrda ispravnosti projekata.

### LibreTT

[Repozitorijum](https://github.com/tradicije/librett) daje iskustvo stvarnog
kluba, tokove unosa, migracije, lokalizaciju i uvoz/izvoz sa pregledom pre potvrde.
Turnirski dodatak već razlikuje kategorije, prijave, učesnike, članove para,
grupe, mečeve i izvore mesta u kosturu. Te pojmove vredi preneti.

[BracketGenerator](https://github.com/tradicije/librett/blob/7fd71498369e81ffdc763d454ddf90ff009a2862/addons/tournaments/src/Domain/BracketGenerator.php)
je uprkos lokaciji u Domain sloju vezan za `$wpdb`, SQL i WordPress vreme.
Sortirane nosioce smešta redom u susedna mesta, briše prethodne mečeve pri
generisanju i napredovanje rešava direktnim upisima. Nije gotov nezavisan engine
za preuzimanje. Novi žreb mora imati razdvajanje nosilaca, eksplicitne bye veze,
transakcije i zaštitu već odigranih zavisnih mečeva pri ispravci rezultata.

### DimiPress Rally

[Arhitektura](https://github.com/tradicije/dimipress-rally/blob/9856a17df4c3019a4d43bc964509b28defcee25d/docs/architecture/ARCHITECTURE.md)
je bolji uzor za novo jezgro: Domain ne zna za bazu ili platformu; Application
orkestrira slučajeve korišćenja preko interfejsa; Infrastructure i platforma
su adapteri. Potvrđeni rezultati su izvor istine, tabele su obnovljive projekcije.

Konkretni `SetScore`, `IndividualMatch` i njihovi testovi daju korisne primere
validacije i završetka meča. Postojeći `IndividualMatch` ima fiksna tri osvojena
seta; novu verziju parametrizujemo pravilnikom kategorije. Specifikacije i
testne primere prenosimo u novi jezik, uz dopunu za turnire.

[ADR 0004](https://github.com/tradicije/dimipress-rally/blob/9856a17df4c3019a4d43bc964509b28defcee25d/docs/adr/0004-distributed-data-and-continuity.md)
daje stabilne ID-jeve, poreklo zapisa, istoriju ispravki, verzionisane izvoze i
razdvajanje javnih i privatnih podataka. To planiramo od početka. Potpisivanje,
federacija i oporavak autoriteta ostaju kasniji posebni projekti.

Rally je trenutno mali kostur sa implementiranim delom domene, ne kompletan
turnirski engine. Njegova ligaška politika rangiranja nije automatski pravilnik
pojedinačnog turnira. Mini-tabele i sledeći kriterijumi su i u dokumentaciji
delimično otvoreni; moramo ih precizirati za naš konkretan turnir.

## 3. Predlog tehnologije

| Deo | Predlog | Razlog |
| --- | --- | --- |
| Desktop omotač | Tauri 2 | Podržava ciljne platforme i web interfejs uz Rust backend |
| Jezgro i slučajevi korišćenja | Rust | Jedno mesto za validaciju, žreb, napredovanje i upise |
| Interfejs | TypeScript + Svelte + Vite | Pogodno za forme, tabele i kasniji companion web interfejs |
| Lokalni podaci | SQLite | Ugrađena relaciona baza, transakcije, bez posebne instalacije servera |
| Razmena | Verzionisani JSON paket i CSV prijave | Prenosivost, pregled i interoperabilnost |
| Companion transport, kasnije | Lokalni HTTP API + WebSocket | Telefon komunicira sa autoritativnim računarom |

Ovo je preporuka, ne zaključana odluka. Svelte je izbor za ovaj projekat,
ne tvrdnja da je objektivno bolji od Reacta. Rust znači dodatno učenje i dva
jezika; taj trošak prihvatamo samo ako tim želi da održava takvu arhitekturu.
Poslovna pravila ostaju u Rustu; TypeScript ima prikaz, forme i transportne tipove.
Tipove ugovora generišemo ili proveravamo zajedničkim fixture primerima da se
Rust i TypeScript ne raziđu.

Alternative:

- Electron + TypeScript: jak izbor ako je prioritet jedan jezik i Node ekosistem.
  Isporučuje Chromium/Node; za očekivanu desktop aplikaciju preferiram Tauri,
  ali veličinu i memoriju merimo prototipom, ne obećavamo brojke unapred.
- Flutter + Dart: podržava tri desktop platforme i vredi razmotriti ako puna
  mobilna aplikacija postane glavni proizvod. Ovde su desktop i web companion
  važniji za predloženi tok rada.
- PHP engine kao ugrađeni servis: moguć, ali uvodi pakovanje PHP runtime-a i
  platformskih adaptera. Obim trenutno prenosivog koda ne opravdava taj trošak.

Tauri koristi različite sistemske webview implementacije. Pre konačnog izbora
moramo proveriti instalaciju bez interneta, štampu, drugi ekran, fontove i
velike tabele na sva tri OS-a. Podržane minimalne verzije OS-a i Linux
distribucije definišemo tek nakon provere; naziv „Linux“ sam nije test matrica.
Instaleri moraju nositi ili dokumentovati potrebne runtime zavisnosti.

Zvanične reference: [Tauri](https://v2.tauri.app/start/),
[webview razlike](https://v2.tauri.app/reference/webview-versions/),
[distribucija](https://v2.tauri.app/distribute/),
[Electron](https://www.electronjs.org/docs/latest/tutorial/process-model),
[Flutter desktop](https://docs.flutter.dev/platform-integration/desktop).

## 4. Arhitektura i granice

```text
Desktop UI → Tauri adapter ─┐
                          ├→ Application → Domain
Companion → LAN adapter ──┘       ↓ portovi
                          SQLite / backup / export
```

Domain nema Tauri, SQL, HTTP ili UI zavisnosti. Application pokreće komande i
upite, transakcije i projektore preko portova. Adapteri prevode desktop i
mrežne zahteve u iste komande. UI nema direktan SQL pristup.

Predložena buduća struktura, bez kreiranja koda u ovoj fazi:

```text
apps/desktop/                 Svelte UI i Tauri adapter
apps/companion/               kasniji web/native companion
crates/domain/                pravila, entiteti, vrednosti
crates/application/           komande, upiti, portovi
crates/storage-sqlite/        repozitorijumi, migracije, projekcije
crates/lan-host/              kasniji API i uparivanje
packages/contracts/           transportne šeme i primeri
packages/ui/                  zajednički prikaz kad se potreba potvrdi
specs/                        pravilnici i prihvatni scenariji
docs/adr/                     odluke i obrazloženja
```

Počinjemo modularnim monolitom u jednom repozitorijumu. Ne uvodimo mikroservise,
generički jezik za pravilnike ili mrežu ravnopravnih autora bez stvarnog slučaja.

## 5. Podaci i pouzdanost

Početni model: Tournament, Category, Player, Club, Entry, EntryMember,
Registration, FeePayment, Stage, Group, Draw, Match, SetScore, ResultRevision,
Table, TableAssignment, RulesVersion i AuditEntry. Učesnik kategorije je
pojedinac ili par; osoba i prijava u kategoriju nisu isti identitet.

Registration predstavlja prijavu na turnir, a CategoryRegistration izbor
kategorije i odgovarajućeg Entry učesnika. Jedan igrač može biti prijavljen u
više kategorija istog turnira. Par čine dva postojeća igrača; ne pravimo kopije
njihovih profila. Nosilac i format pripadaju kategoriji, ne celom turniru.

Najava/prijava, potvrda dolaska i plaćanje su nezavisna stanja. Organizator
može unapred prijaviti igrača, a evidentirati dolazak i uplatu na dan turnira.
Neplaćena prijava se ne odbacuje automatski iz žreba; organizator odlučuje o
uslovima učešća. Blagajna razlikuje zaduženje, popust/oslobađanje, uplatu i
povraćaj, podržava delimično plaćanje i prikazuje dugovanje i neto primljeno.
Jedna uplata može pokriti više kategorija uz evidentiranu raspodelu. Dubl ima
izbor kotizacije po paru ili osobi. Prva blagajna je evidencija kotizacija,
ne računovodstveni ili fiskalni sistem.

Jedna lokalna baza po radnom prostoru, više turnira u njoj. Turnir se može
izvesti kao samostalan paket. ID-jevi nastaju lokalno i ostaju stabilni pri
prenosu. Klupska pripadnost i pravila korišćena na turniru imaju istorijski
snimak, tako da kasnije promene profila ne menjaju završeno takmičenje.

Upis rezultata, revizije i zavisnih promena je jedna transakcija. Tabele i
napredovanje mogu da se obnove iz potvrđenih podataka i verzije pravila.
Ne gradimo potpuni event-sourcing sistem: čuvamo normalizovane izvorne zapise,
revizije i audit. Novac je ceo broj najmanjih novčanih jedinica uz valutu.

Razlikujemo nacrt, zakazan meč, meč u toku, potvrđen rezultat, bye, nepojavljivanje,
predaju i administrativnu odluku. Bye nije izmišljeni odigrani rezultat.
Pravila bodovanja tih stanja preciziramo pre implementacije.

Ispravka ranijeg rezultata prvo prikazuje posledice. Ako je sledeći zavisni meč
počeo ili završen, automatsko prepisivanje učesnika se blokira i traži eksplicitnu
organizatorsku odluku sa razlogom. Undo je nova evidentirana ispravka.

Backup koristimo kroz konzistentan SQLite backup postupak, ne kopiranje živog
fajla baze. Automatski snimci imaju rotaciju; pre migracije pravimo backup.
Restore je proveren tok rada. Izvoz ima verziju formata, manifest i provere
integriteta; uvoz ima validaciju, pregled i transakciju. Privatni i javni izvoz
su odvojeni. [SQLite Backup API](https://www.sqlite.org/backup.html).

## 6. Prvi kompletan turnir

Dogovoreni obim turnirske grane uključuje singl i dubl, više kategorija i blagajnu.
Svaka kategorija zasebno bira: direktni jednostruki nokaut ili round-robin
grupe pa jednostruki nokaut. Kombinovani format znači prvo svako-sa-svakim u
grupi, zatim prolaz u kostur. Različite kategorije istog turnira mogu imati
različite formate, nosioce i broj setova za pobedu. Starosna ograničenja su
podešavanja sa jasno definisanim datumom obračuna, ne zaključak iz naziva „40+“.
Prve razvojne korake možemo proveravati singlom, ali dubl nije izbačen iz
dogovorenog proizvoda. Brojevi igrača za pilot nisu licencna ograničenja.

Tok organizatora:

1. Otvara pregled turnira; kreira, pretražuje ili otvara postojeći turnir.
2. Kreira kategorije, za svaku bira format i pravilnik, postavlja stolove.
3. Unosi/bira igrače iz lokalne baze i prijavljuje ih u kategorije.
4. Evidentira najave, dolaske, kotizacije i uplate preko blagajne.
5. Definiše nosioce, bira automatski ili ručni žreb, pregleda i potvrđuje ga.
6. Vodi grupe ako postoje, dodeljuje stolove i unosi rezultate.
7. Proverava rangiranje i potvrđuje prolaz u nokaut ako postoje grupe.
8. Završava kosture kategorija, objavljuje plasman i izvozi arhivu turnira.

Glavne kartice turnira su Pregled, Kategorije, Prijave i Blagajna. Unutar svake
kategorije nalaze se Prijave, Žreb, Mečevi i Rezultati. Pregled i blagajna mogu
prikazati ceo turnir; žreb, mečevi i rezultati vezani su za izabranu kategoriju.

Organizator uvek može da izabere automatski ili ručni žreb za kategoriju.
Automatski režim generiše grupe/kostur prema nosiocima i podešavanjima, a
organizator pregleda predlog pre potvrde. U direktnom nokautu bye mesta pripadaju
najjačim nosiocima; organizator može da ih dodeli i ručno. Ručni režim omogućava
raspoređivanje učesnika po grupama ili mestima kostura. Oba režima
koriste istu validaciju: duplikati, kapacitet, pripadnost kategoriji, struktura
kostura i očuvanje odigranih mečeva. U ručnom režimu odstupanje od pravila
nosilaca daje objašnjeno upozorenje i evidentiranu odluku. Pri istovremenom radu
više kategorija dodela stolova proverava da isti igrač ne igra dva meča odjednom,
uključujući njegov nastup u paru.

Uz to idu pretraga, rad tastaturom, jasna stanja čuvanja, štampani protokoli,
prikaz za drugi ekran, backup/restore i istorija ispravki. Srpski i engleski
su početni jezici; pisma i format datuma su eksplicitne postavke.

Za žreb čuvamo ulaz, algoritam/verziju, slučajni seed, ograničenja i izlaz.
Isti ulaz može da reprodukuje isti žreb. Razdvajanje kluba je ograničenje sa
vidljivim upozorenjem kad ga nije moguće zadovoljiti. Regenerisanje potvrđenog
žreba je posebna evidentirana radnja, a posle početka igre ograničeno pravilima.

Organizator ručno označava i poređa nosioce. Za grupe bira broj grupa i broj
učesnika koji prolaze iz svake; automatska raspodela pravi grupe približno iste
veličine i razdvaja nosioce.
Kriterijumi rangiranja podešavaju se po kategoriji. Podrazumevani
redosled je međusobni rezultat, mini-tabela samo između izjednačenih učesnika
kada ih je tri ili više, zatim odnos setova pa odnos poena. Nulti imenitelj i
nastavak razrešavanja delimičnog izjednačenja definisani su u
[specifikaciji žreba](../specs/draw-and-ranking.md). Pozitivan broj dobijenih
setova/poena uz nula izgubljenih daje najbolji odnos; nula prema nula ostaje
izjednačeno. Ako ostane više bye mesta nego nosilaca, preostala mesta se dodeljuju
nasumično i organizator može da ih izmeni.
Ne koristimo nasumičan ID kao nevidljivo konačno sportsko pravilo.

## 7. Telefoni i internet

Jedan desktop je autoritet turnira. Telefoni šalju komande; ne upisuju bazu i
ne odlučuju konačan plasman. Sudija dobija ograničen pristup dodeljenom meču
putem uparivanja; gledalac dobija samo javne podatke. Organizator može da
opozove uređaj i ispravi rezultat uz trag.

Lokalni ruter/LAN omogućava komunikaciju bez interneta. Bez ikakve mreže telefon
ne može uživo da prati drugi uređaj. Native companion može kasnije čuvati
predlog rezultata offline; status „poslato“ i „prihvaćeno“ ostaju odvojeni.
Zahtev sadrži ID operacije i očekivanu reviziju meča: ponavljanje ne duplira
rezultat, zastarela promena dobija konflikt umesto tihog prepisivanja.

Web companion je početni kandidat za publiku, dok sudijski offline rad može
zahtevati instaliranu aplikaciju. PWA service worker zahteva bezbedan kontekst;
običan HTTP na LAN adresi nije isto što i localhost. Zato instalaciju, TLS,
uparivanje i mobilna ograničenja proveravamo pre izbora companion pakovanja.
[Service Worker dokumentacija](https://developer.mozilla.org/en-US/docs/Web/API/Service_Worker_API).

Javna internet stranica kasnije prima samo dozvoljene projekcije. Pad javnog
servisa ne zaustavlja turnir. Cloud, federation i istovremeni offline upis sa
više organizatorskih računara nisu deo prve verzije.

### Buduća baza igrača na stoni.rs

Ovo je planirana integracija; baza i API još ne postoje. Kada bude internet,
organizator može pretraživati udaljeni katalog i preuzeti igrače ili paket baze,
pa izabrane igrače prijaviti na lokalni turnir. Posle preuzimanja isti podaci
su pretraživi offline. Preuzimanje profila nije automatska prijava na turnir.

Adapter koristi verzionisani API/izvoz, stabilni spoljašnji ID i poreklo profila.
Ne povezujemo se direktno na bazu stoni.rs. Ponovljeni uvoz ažurira postojeće
mapiranje umesto dupliranja; za prethodno ručno unetog igrača nudimo pregled
povezivanja, bez automatskog spajanja samo po imenu. Preuzeti podaci ne brišu
lokalne izmene niti menjaju istorijski snimak završenog turnira bez odluke.
Obim dostupnih podataka, preuzimanje cele baze, straničenje i pravila objave
definišu se kada API postoji. Turnir ostaje potpuno upotrebljiv bez tog servisa.

## 8. Faze i uslovi završetka

| Faza | Isporuka | Uslov završetka |
| --- | --- | --- |
| 0 — specifikacija | ADR za stack, rečnik, pravilnik, model stanja, skice ekrana | Dogovoreni primeri i granice MVP-a |
| 1 — provera platformi | Mali prototip na tri OS-a | Offline start/install, SQLite, backup, štampa, drugi ekran provereni |
| 2 — jezgro turnira | Rezultati, žreb, grupe, rangiranje, napredovanje | Automatizovani testovi stvarnih rubnih slučajeva |
| 3 — upotrebljiv desktop | Singl/dubl, kategorije, blagajna, ručni/automatski žreb, finale i arhiva | Probni turnir bez interneta i oporavak nakon prekida |
| 4 — pilot i stabilizacija | Instaleri, dokumentacija, popravljeni problemi iz sale | Ceo pravi turnir uspešno vođen |
| 5 — companion | LAN gledanje, uparivanje, sudijski unos | Testirani prekidi veze, duplikati, konflikt i opoziv |
| 6 — širenje | stoni.rs katalog kad postoji API, javni rezultati, LibreTT/Rally adapteri, lige | Posebno specifikovani slučajevi i migracije |

Faza 1 je prva implementacija, tek nakon faze 0. Ako Tauri ne zadovolji ključne
operativne zahteve, menjamo odluku pre gradnje ostatka aplikacije.

Obavezni scenariji: neparan broj učesnika, više bye mesta, nemoguće razdvajanje
kluba, tri izjednačena igrača, predaja/nepojavljivanje, ista osoba u više
kategorija, ispravka pre i posle narednog meča, prekid tokom upisa, neuspešna
migracija, restore na drugom OS-u i očuvanje srpskih slova pri uvozu/štampi.
Dodajemo: najavljen ali neplaćen igrač, delimična uplata/povraćaj, uplata za
više kategorija, ručni žreb, dubl i preklapanje igrača između kategorija,
ponovljeno preuzimanje udaljenog profila i rad posle gubitka interneta.

## 9. Jezici, autorstvo i projektna dokumentacija

Autor: Aleksa Dimitrijević. Licenca: GNU AGPL 3.0 ili bilo koja kasnija verzija
(`AGPL-3.0-or-later`), u skladu sa postojećim LibreTT i Rally projektima.
Puni tekst licence je u korenom `LICENSE` fajlu, a projektna izjava u README.

Od početka ceo korisnički proizvod podržava srpski i engleski: navigaciju,
forme, poruke validacije, štampu, prikaz rezultata i companion kada nastane.
Prevodni ključevi su stabilni i nezavisni od vidljivog teksta; ne čuvamo
prevedene statuse u bazi. Nazive turnira/kategorija koje korisnik unese ne
prevodimo automatski. Srpska latinica je početni predlog; ćirilicu dogovaramo.

Planirani fajlovi u novom projektu:

- `README.md` — engleski; `README-sr.md` — srpski, sa međusobnim linkovima.
- `CHANGELOG.md` — engleski, promene po verzijama.
- `LICENSE` — puni tekst izabrane AGPL 3.0 varijante i oznaka u metapodacima.
- `CONTRIBUTING.md` — doprinosi, razvoj, provere, prijava grešaka i prevodi;
  početni engleski dokument sa srpskim pandanom `CONTRIBUTING-sr.md`.
- `docs/en/` i `docs/sr/` — korisničko uputstvo, arhitektura, backup/restore,
  žreb, pravilnici, blagajna i buduće integracije.
- `docs/adr/` — arhitektonske odluke; `specs/` — proverljivi poslovni scenariji.

Dokumentacija prati ponašanje svake isporučene faze, sa označenim budućim
funkcijama. Ovaj radni plan je trenutno srpski; engleski pandan pripremamo pri
ustaljivanju specifikacije. U ovoj fazi ne pravimo README koji bi sugerisao
da aplikacija već postoji ili može da se instalira.

## 10. Šta dogovaramo pre implementacije

- Da li prihvatamo Rust + TypeScript/Svelte + Tauri kao početni stack?
- Koji stvarni turnir i pravilnik koristimo za prvi pilot?
- Izvor i redosled nosilaca, izuzeci pravilnika, treće mesto i detalji računanja
  seta/poena i nastavka rangiranja.
- Podržani OS minimumi, starost računara i raspoložive mašine za proveru.
- Srpsko pismo; naziv LibreTT i AGPL-3.0-or-later su zabeleženi u dokumentima.

Kalendar i procene dajemo nakon faze 0 i provere platformi. Najvažniji kriterijum
prve verzije je pouzdano završen turnir u sali, uključujući greške i oporavak.


## Trenutni napredak takmičarskog toka

Implementirani su automatski/ručni nacrti po kategoriji, nosioci, grupe i nokaut,
rezultati po setovima, predaja/nedolazak, mini-tabele za izjednačene, prolaznici,
BYE i automatski/ručni Lucky loser. Rezultati prikazuju konačan plasman sa
zajedničkim trećim mestima i rasponima prema fazi ispadanja.

Završavanje kategorije/turnira proverava uslove, čuva potvrđeni snimak rezultata
i zaključava takmičarske izmene. Ponovno otvaranje traži potvrdu, uz očuvanje
istorije. Šema 18 čuva i ručne dodele stolova. Uvedeni su backup/restore, izvoz/štampa,
meč za bronzu i tri pravila trećeg mesta. Stolovi se za sada ručno unose u
editoru meča. Dodati su testovi kompletnog takmičarskog toka. Ostaju automatski
raspored, drugi ekran, instalacioni paketi i ručna provera sistemskih dijaloga.
