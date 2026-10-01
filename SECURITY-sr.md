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

Fotografije se dekodiraju i smanjuju u UI-ju i čuvaju kao ograničeni JPEG data
URL-ovi. Backend proverava base64, veličinu i početni/završni JPEG potpis, a ne
potpuno dekodiranje. Spoljni URL-ovi fotografija nisu dozvoljeni. Budući uvoz
fajlova i mrežnu sinhronizaciju treba tretirati kao nove granice poverenja.

Pre companion pristupa ili distribucije izdanja proveriti autentikaciju,
dozvole, zavisnosti, pakovanje/potpisivanje, nepoverljive uvoze i proces prijava.
Planirane mogućnosti nisu postojeće garancije.

[GitHub uputstvo za privatne prijave](https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/report-privately).
