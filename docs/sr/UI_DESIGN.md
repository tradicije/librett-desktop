# Smernice za desktop interfejs

Prolaznici grupa u kosturu imaju oznaku grupe i plasmana ispred imena (A1, B2
i slično). Oznaka prati igrača kroz naredne runde; nerazrešena mesta prikazuju
istu oznaku uz kratko „Čeka prolaznika”.

LibreTT je radni alat organizatora. Tokom turnira najvažnije je brzo pronaći
prijavu, potvrditi dolazak ili evidentirati uplatu. Hijerarhija prati te zadatke.

- Zadržavamo LibreTT paletu u obe teme, Libre Franklin i Tabler ikonice.
  Primarna boja označava akciju, izbor ili fokus.
- Bočni meni ostaje uz prozor; traka na vrhu prikazuje kontekst. Početni izbor
  turnira ili liga ostaje bez bočnog menija.
- Imena, iznosi i akcije imaju prednost. Liste koriste kompaktne redove i
  razdelnike; paneli grupišu povezane podatke i forme.
- Polja profila grupisana su u profil, kontakt i dodatne podatke. Lista igrača
  prikazuje sažetak, a uređivanje daje pristup svim detaljima.
- Izbor, fokus, greška i čuvanje moraju biti jasni i preko teksta i ponašanja.
  Navigacija ostaje zaključana dok se ne potvrdi neizvestan finansijski upis.
- U manjim prozorima kolone prelaze jedna ispod druge. Podržani su tastatura,
  smanjeno kretanje, srpski i engleski, kao i svetla/tamna/sistemska tema.

Linear, Raycast, Figma, GitHub Desktop, Notion i Arc služe kao autorske smernice
za miran raspored, čitljive liste i jasne akcije. LibreTT koristi svoj vizuelni
identitet i raspored za turnirski rad. Reference i detalji su u
[engleskim smernicama](../en/UI_DESIGN.md).

Radni prostor turnira koristi stalne kartice Podešavanja, Pregled, Kategorije i
Blagajna. Kartice Žreb, Mečevi i Rezultati nalaze se unutar kategorije.
Delovi koji još nisu implementirani jasno prikazuju da su u pripremi. Brojači
prikazuju stvarne podatke, a probni podaci za proveru interfejsa ostaju van
repozitorijuma.


## Sistemska istorija

Istorija ekrana i kartica koristi History API webview-a. Strelice i bočna dugmad
miša dele istoriju; na macOS-u je uključena i navigacija trackpadom. Tokom upisa
sistemska navigacija vraća trenutnu poziciju istorije pre promene ekrana.


## Red radnih tabova

Fiksna traka od 48px počinje stalnim Home dugmetom, pa nezavisnim radnim tabovima
sa kratkim nazivom strane i punim kontekstom na zadržavanje miša. Nazad/Napred
ostaju u traci ispod. Home nije običan tab. macOS sistemski dugmići dele isti red
i centriraju se AppKit koordinatama. Ostali sistemi zadržavaju naslovnu traku.
Visina sadržaja uzima ovu traku u obzir; skrol postoji tek kada sadržaj ne staje.
Sakriveni radni prostori čuvaju lokalno stanje.

Radni tabovi imaju fiksnu širinu 180px bez obzira na dužinu naziva. Tekst bledi sa
desne strane samo kada zaista ne staje; ne koristi tri tačke. Pun naziv/kontekst
je dostupan na zadržavanje miša, a lista tabova se skroluje horizontalno.


Levi meni ima proširen i sužen režim od 68px. U proširenom režimu dugme za
sužavanje je desno od punog logotipa sa razmakom od 16px; u suženom se koristi kvadratna ikonica
brenda prema temi kao dugme za proširenje i navigacija samo ikonicama, sa
pristupačnim nazivima/opisima. Slike brenda ne vode na Home.
Zajednički izbor širine pamti se kroz radne tabove i restart aplikacije.

U dnu proširenog menija, ispod oznake Lokalni rad i iznad copyright-a, prikazuje
se LibreTT for Windows, LibreTT for MacOS ili LibreTT for Linux prema platformi.
Tekst je isti na oba jezika interfejsa i sakriven je u suženom meniju.


Kartice kategorija koriste iste razmake, veličinu ikonice, tipografiju i prilagodljivi
raspored akcija kao kartice igrača. Izmeni/Obriši imaju iste sekundarne dugmiće
sa ikonicama. Dodavanje/izmena ima poseban ekran; Grupe imaju poseban tab sa najviše tri kolone kartica; Žreb prikazuje samo kostur.


Podešavanja kategorije koriste jedan spoljašnji panel, sa sekcijama odvojenim
razmacima i linijama. Lista učesnika i prijavljivanje ostaju samo u tabu Prijave. Dodavanje
i izmena kategorije koriste isti stil grupa polja i akcija kao editor igrača.


Nosioci i raspored koriste numerisane redove nosilaca, ikonice za pomeranje i
uklanjanje, zajedničke dropdown kontrole i odvojene akcije pravljenja i čuvanja.
Status nacrta je iznad ručnih pozicija. Grupe koriste najviše tri kolone, a
nokaut protivnici stoje u paru jedan pored drugog, odnosno vertikalno na užem prozoru.

## Mečevi i prikaz igrača

Mečevi se filtriraju po grupi i kolu, uz najviše 32 po stranici. Kartica prikazuje
učesnike, rezultat setova i status. Dijalog podržava unos po setovima, predaju,
nedolazak i ispravke; promena nokaut pobednika traži posebnu potvrdu ako
poništava naredne rezultate. Završeni setovi imaju oznaku, a zbir se menja tokom unosa.

Ime igrača prati oznaka kluba u zagradi: prva tri slova/cifre, velikim slovima.
Oznaka koristi manji font i sivu boju. Zajednički `PlayerName.svelte` koristi se
u listama, prijavama, dolascima, nosiocima, padajućim menijima, grupama,
parovima, žrebu, mečevima i blagajni. Ime u bazi i polje za uređivanje imena
ostaju bez oznake. Igrači bez kluba nemaju sufiks.

Tabele grupa imaju uživo plasman i dijalog za ručni/automatski redosled.
Nokaut kola koriste nazive faza, a grupna kola brojeve.

Kostur nema spoljašnji okvir: strukturu daju kartice mečeva i vezne linije.
Klub se prikazuje samo kao oznaka pored imena, bez ponovljenog podnaslova.

Žreb ima kratak sažetak pravila, ikonicu za osvežavanje i dugme Uredi.
Kratka poruka o prolaznicima zamenjuje duplirane naslove i duga objašnjenja.
