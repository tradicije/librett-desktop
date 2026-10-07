# Smernice za desktop interfejs

Prijave koriste separator tabova kategorije, bez dodatnog gornjeg bordera i
razmaka na samoj sekciji, nezavisno od dijaloga između elemenata.

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
i centriraju se AppKit koordinatama. Linux i Windows koriste dugmad aplikacije
u istom redu koja pozivaju sistemske akcije prozora. Windows dugmad su
pravougaona i poravnata desno.
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

Starosna grupa koristi podrazumevano isključen prekidač i polja Od/Do u istoj
formi. Prijava van raspona ima jedan dijalog sa imenima, oznakama klubova,
starostima i eksplicitnim dugmetom Prijavi kao izuzetak.

Kategorije sa grupama imaju jedan izbor BYE/Lucky loser u zajedničkoj formi pravila.
Lucky loser mesta su ravan odeljak Podešavanja, sa dva stupca izbora po mestu:
Automatski / igrač ispod crte / BYE, i dugmadima Sve automatski / Sačuvaj mesta.
Statistika kandidata je u tabeli koja se otvara po potrebi. Kratka poruka objašnjava
izjednačenje na granici ili nevažeći ručni izbor. Promene zavisnih rezultata traže
potvrdu i objašnjavaju da istorija ostaje. Kostur pokazuje LL C3 ispred imena,
uz isti manji sivi prikaz kluba.

Rezultati imaju ravan prikaz napretka/statusa, kratak spisak nedovršenih koraka,
kartice pobednika/finaliste/zajedničkih trećih mesta i tabelu plasmana. Završetak
je ispod rezultata, a ponovno otvaranje ima zasebnu potvrdu. Pregled turnira ima
najviše tri kartice kategorija po redu, brojač završetaka i linkove do rezultata.
Završeno takmičenje onemogućava izmene, uz dostupnu navigaciju i blagajnu.
Kartice kategorija i turnira imaju oznaku Završeno.


Kratki sažeci kategorija i pravila ostaju u jednom redu i prelamaju se na uskim prozorima. Detalji igrača i napredak grupa/nokauta koriste zasebne označene redove gde to olakšava čitanje. Svaki odigrani set ima zasebnu oznaku rezultata.

## Uređivanje kostura i treće mesto

U kategoriji otvori **Žreb → Uredi**, pa klikni na igrača ili BYE u prvoj rundi. Biraj između prijava te kategorije; izbor već raspoređenog učesnika menja njihova mesta. Izmene se čuvaju posebno. Naredne runde prate pobednike mečeva i ne menjaju se nezavisno. Izmene direktnog nokauta prave novu verziju žreba i zadržavaju prethodne rezultate u istoriji. Nokaut posle grupa uređuje se kada su sve grupe završene i plasman razrešen; poništavanje zavisnih rezultata traži potvrdu.

Pravljenje i podešavanja kategorije imaju tri prekidača u jednom redu (na uskim prozorima jedan ispod drugog): starosna grupa, Lucky loser/BYE i jedno treće mesto. Lucky loser je dostupan za grupe → nokaut, dok direktni nokaut koristi BYE. Isključen prekidač trećeg mesta znači zajedničko treće mesto. Uključen otvara izbor meča za bronzu ili dodelu trećeg mesta polufinalisti koji je izgubio od kasnijeg pobednika; drugi poraženi polufinalista je četvrti. Meč za bronzu je u završnoj rundi Mečeva i ispod kostura; kategorija čeka njegov rezultat. Ako zbog BYE postoji samo jedan poraženi polufinalista, on je treći bez dodatnog meča.

Uključen Lucky loser prekidač otvara automatsko ili ručno popunjavanje. U ručnom režimu prazna mesta čekaju izbor učesnika ili BYE. Kartice baze igrača prikazuju samo ime/klub i starost (tekuća godina minus godište); ostali podaci su u profilu.

Izbor učesnika u kosturu koristi dropdown unutar toka dijaloga. Visina liste prilagođava se prozoru; lista se skroluje bez širenja apsolutno postavljenog menija izvan dijaloga.

Broj godina koristi glavnu boju teksta teme, a jedinica je siva. Prikaz koristi „godina“ za završetke 1 i 5–9/0, „godine“ za završetke 2–4, uz izuzetke 11–14 koji koriste „godina“.

Rezervne kopije su u bočnom meniju. Editor meča prikazuje broj stola sa zasebnim čuvanjem; dugmad za izveštaje su uz Grupe/Žreb, Rezultate i Blagajnu. Pregled štampe je u zasebnom prozoru.

## Istorija akcija

Istorija koristi zajednički heading, panel, Select, sekundarne dugmiće, oznake i prikaz imena igrača. Jedan panel filtera sadrži dropdown polja za turnir, vrstu istorije i akciju. Događaji su grupisani po lokalnom datumu, sa ikonicom vrste, jasnom oznakom istorije, akcijom, nazivom i kontekstom turnira/kategorije. Detalji pre/posle otvaraju se unutar reda, bez novih uokvirenih panela. Iste boje teme važe za svetli i tamni režim; uski prozori slažu filtere, vreme i kolone promena. Prethodno sačuvana istorija jasno je označena, bez izmišljanja akcija ili identiteta organizatora. Labele imaju jasan razmak iznad dropdown polja. Kompaktni redovi raspoređuju akciju i naziv, turnir/kategoriju i vrstu/vreme u tri kolone; datum ostaje u naslovu grupe. U uskim prozorima kontekst prelazi ispod naziva.

Osvežavanje i ponovno učitavanje koriste kružnu refresh ikonicu. Strelica za poništavanje označava vraćanje ili ponovno otvaranje stavke, a precrtani krug uklanjanje potvrde dolaska.

Započeta kategorija ne može da se ukloni. Čuvanje novog žreba u Podešavanjima traži potvrdu uz objašnjenje da rezultati i dodeljeni stolovi ne prelaze u novu verziju. Promena bodovanja nakon početka objašnjava da prethodni rezultati zadržavaju svoja pravila, a naredni unosi ili ispravke koriste nova; potvrda postoji u oba ekrana za uređivanje kategorije.

Kategorije sa grupama imaju switch Isti klub u grupi pri dodavanju i u podešavanjima, podrazumevano uključen. Isključivanje važi za nove automatske rasporede. Ako razdvajanje ne uspe, poruka upućuje na promenu nosilaca/grupa ili ručni raspored. Polje kluba igrača nudi postojeće nazive i dugme za prihvatanje sličnog naziva; tipfeler se ne ispravlja bez izbora organizatora.

Glavni levi meni prikazuje Turnire, Igrače, Rezervne kopije, Istoriju i na kraju Korpu, jednu stavku ispod druge sa nazivima. Vodič ostaje pri dnu iznad separatora i lokalnog rada. Smanjeni meni prikazuje ikonice bez okvira dugmića, sa dostupnim nazivima i hover opisima. Vodič ima numerisane korake, prečice do koraka, pitanja za česte probleme i rečnik; prečice skroluju bez dodavanja stavki istorije pregledača.

Diskretna horizontalna linija odvaja donju dugmad od lokalnog rada, sa po 18px razmaka iznad i ispod linije u proširenom i smanjenom meniju.

Navigacija levog menija počinje direktno stavkom Turniri, bez malog naslova iznad nje.


## Obaveštenje o besplatnom softveru

Prvi ekran sa izborom turnira ili lige ispod kartica prikazuje obaveštenje da je LibreTT besplatan zauvek i da korisnik koji je program kupio treba odmah da traži povraćaj novca od prodavca. Zvanični linkovi su `https://librett.org` i `https://github.com/tradicije/librett-desktop`. Desktop ih otvara u sistemskom pregledaču, uz dozvoljene samo te dve fiksne adrese; pregled interfejsa koristi standardne spoljne linkove. Obaveštenje prati srpski/engleski jezik aplikacije.

Obaveštenje zauzima celu širinu mreže kartica uz kompaktne razmake. Linkovi nose nazive **LibreTT Website** i **Source Code**. Sekcija ne preuzima globalne stilove bočnog menija.

Obaveštenje ima naslov od 22px, odvojene pasuse za besplatan softver i povraćaj novca i donji red sa oznakom zvaničnih linkova levo, a imenovanim linkovima desno. U uskom prozoru donji red prelazi u vertikalni raspored.

Prvi ekran centrira naslov, obe kartice i obaveštenje zajedno unutar prostora za sadržaj. Jedna flex kolona minimalne visine jednake prostoru za skrolovanje centrira sve sekcije zajedno i raste kada sadržaj zahteva skrolovanje; stara pravila fiksnog gornjeg razmaka su uklonjena.
