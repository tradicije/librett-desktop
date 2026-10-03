# Bezbednosna politika

[English](SECURITY.md)

## Podržane verzije

LibreTT je u ranom razvoju. Još nema podržanih stabilnih izdanja ni objavljenih
instalera. Bezbednosne ispravke primenjuju se na trenutnu granu `main`; ažuriraj
razvojnu kopiju pre prijavljivanja već ispravljenog problema. Oznaka `0.1.0` ne
predstavlja stabilno izdanje niti potvrdu bezbednosne revizije.

## Prijavljivanje ranjivosti

Koristi **Report a vulnerability** na
[Security stranici repozitorijuma](https://github.com/tradicije/librett-desktop/security)
kada je uključeno privatno prijavljivanje na GitHub-u. Dostupnost zavisi od
podešavanja repozitorijuma; ova politika ne uključuje tu mogućnost niti tvrdi da
je ona već uključena. Ako privatno dugme nije dostupno, otvori kratak javni issue
sa zahtevom za privatni kontakt sa Aleksom Dimitrijevićem, bez detalja ranjivosti,
koda za iskorišćavanje ili privatnih podataka. Posebna bezbednosna e-pošta nije objavljena.

Privatno dostavi commit/verziju, operativni sistem, korake reprodukcije sa
izmišljenim podacima, očekivano i stvarno ponašanje i mogući uticaj. Dokaz
iskorišćavanja deli privatno. U javne prijave ne dodaj stvarne fotografije igrača,
kontakte, SQLite baze, rezervne kopije, pristupne podatke ili lične putanje
fajlova. Prijave mogu biti na srpskom ili engleskom. Za sada nema garantovanog
roka odgovora ni programa nagrađivanja prijava.

## Trenutne granice zaštite

Desktop aplikacija radi lokalno i veruje operateru i OS nalogu. Nema
prijavljivanja nalogom, kontrole administratorskih prava, šifrovanja baze unutar
aplikacije ni implementiranog LAN, companion ili cloud servisa. Vite razvojni
server je razvojni alat, podrazumevano vezan za loopback; nije administratorski
servis namenjen izlaganju javnoj ili klupskoj mreži.

Profili i fotografije čuvaju se u lokalnoj SQLite bazi van repozitorijuma.
Rezervne kopije pre migracije sadrže iste lične podatke. Zaštiti ih dozvolama OS
naloga, odgovarajućom zaštitom skladišta uređaja i kontrolisanim rezervnim kopijama.
Repozitorijum ignoriše lokalne baze i tajne.

Produkcioni web sadržaj ima podešen CSP, a SQL upisi koriste vezane parametre.
Domenska i aplikaciona validacija proveravaju imena, učesnike, duple prijave i
ograničenja profila. To nije bezbednosna revizija niti zaštita od kompromitovanog
OS naloga.

Slike imaju ograničenje veličine fajla, dimenzija i ukupnog broja piksela pre
browser dekodiranja. Fotografije i naslovne slike se smanjuju/kropuju u JPEG;
backend proverava dimenzije i radi potpuno JPEG dekodiranje uz ograničenje
memorije. Animirani PNG i prošireni/animirani WebP nisu prihvaćeni. Spoljašnji
URL-ovi za slike nisu prihvaćeni. Budući import i sinhronizacija zahtevaju novu
proveru granica poverenja.

Pre companion pristupa ili distribucije izdanja proveriti autentikaciju,
dozvole, zavisnosti, pakovanje/potpisivanje, nepoverljive uvoze i proces prijava.
Planirane mogućnosti nisu postojeće garancije.

[GitHub uputstvo za privatne prijave](https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/report-privately).

## Nalazi provere zavisnosti (2026-10-03)

Npm audit nije našao poznate ranjivosti. Rust zavisnosti uključuju
[problem bezbednosti iteratora u glib 0.18.5](https://rustsec.org/advisories/RUSTSEC-2024-0429.html)
u Linux GTK lancu; upstream ispravka zahteva glib 0.20 ili noviji. Aplikacija ne
poziva taj iterator, ali posredna upotreba nije isključena. Ovo ostaje ograničenje
Linux verzije. [proc-macro-error 1.0.4 se više ne održava](https://rustsec.org/advisories/RUSTSEC-2024-0370.html)
i ostaje posredna zavisnost pri kompajliranju. Izmene aplikacije ne rešavaju ove nalaze.
