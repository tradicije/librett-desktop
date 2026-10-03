# Razvoj

[English](../en/DEVELOPMENT.md)

## Zahtevi

- Node.js 22 LTS i npm.
- Stabilni Rust sa Cargo, rustfmt i Clippy alatima.
- Sistemske razvojne zavisnosti iz
  [Tauri uputstva](https://v2.tauri.app/start/prerequisites/).

Testovi jezgra ne zahtevaju Linux WebKit/GTK razvojne biblioteke. Desktop
aplikacija ih zahteva. Windows i macOS imaju odgovarajuće sistemske alate.
Potpuna provera na sva tri sistema još predstoji.

## Instalacija i pokretanje

Iz korena repozitorijuma:

```sh
npm ci
npm run check
npm run build
npm run test:core
npm run desktop -- dev
```

`npm run dev` otvara pregled interfejsa na `http://127.0.0.1:1420`.
U pregledaču je čuvanje isključeno; Tauri desktop komanda koristi pravu bazu.
Desktop čuva `librett.sqlite` u sistemskom direktorijumu aplikacionih podataka
koji Tauri određuje za `org.librett.desktop`, van repozitorijuma.
Posle instalacije razvojnih zavisnosti lokalne operacije ne zahtevaju internet.

Trenutno podržavamo turnire, kategorije, lokalne igrače, pretragu po imenu/klubu
i prijavljivanje singl/dubl učesnika.
Singl/dubl i formati se čuvaju kao podešavanja. Nacrti žreba su dostupni; mečevi još nisu
implementirani. Još nema rezultata, interfejsa za izvoz/oporavak
ili instalera. Razvojna verzija `0.1.0` nije objavljeno izdanje.

Tab Igrači prikazuje zajedničku listu. Dodaj/Izmeni otvara zasebne ekrane za
profile i fotografije, a čuvanje vraća na listu. Brisanje traži potvrdu i čuva
igrače sa postojećim prijavama. Prijave unutar turnira koriste tu bazu i mogu
da se povuku i vrate;
dolazak igrača važi kroz sve kategorije istog turnira. Šema verzije 11 sadrži neizmenjive nacrte žreba i verzije pravila kategorije; pre migracije starijih baza pravi se konzistentan
`pre-v11-<uuid>.sqlite` backup.

Nazad/Napred prati istoriju aktivnog radnog taba. Home dugme u gornjoj traci
vraća na stalni početni ekran koji nema bočni meni. Čuvanje zaključava navigaciju do završetka.
Lokalna administracija za sada nema prijavu nalogom niti kontrolu korisničkih uloga.

## Teme i ikonice

Izbor teme podržava Svetla, Tamna i Sistemska. Sistemska prati promene OS teme
dok aplikacija radi. Tema i jezik su lokalne UI postavke; podaci turnira ostaju
u SQLite bazi. `assets/img/logo-light.png` i `logo-dark.png` se pakuju u aplikaciju
i rade offline.

Izvor desktop ikonice je `assets/img/app-icon.png`. Tauri PNG, ICO i ICNS
varijante su u `apps/desktop/src-tauri/icons/`: to su aplikacioni resursi,
a ne razvojni keš. Za promenu ikonice prozora restartuj desktop aplikaciju.

Na Linuxu/Wayland-u sistem pronalazi ikonicu preko `.desktop` zapisa koji odgovara
identitetu `org.librett.desktop`, umesto preko slike ugrađene u prozor. Uključena je
registracija GTK identiteta i GLib naziv pre inicijalizacije GTK-a, tako da
i Wayland prozor ima isti identitet. GTK dozvoljava jednu instancu aplikacije po sesiji sa
tim identitetom. Za razvojnu kopiju registruj ikonicu jednom (i ponovo kada je menjaš):

```sh
npm run desktop:install-icon
```

Komanda kopira PNG slike u `$XDG_DATA_HOME/icons/hicolor` i postavlja skriveni
`org.librett.desktop.desktop` zapis u `$XDG_DATA_HOME/applications` (podrazumevano:
`~/.local/share`). Osvežava KDE keš kada je dostupan. To su lokalni sistemski
fajlovi van repozitorijuma. Zapis povezuje pokrenute prozore sa ikonicom; ne dodaje
pokretač u meni. Zaustavi pa ponovo pokreni desktop razvojnu komandu. Postojeći
zapisi koji nisu razvojni ostaju netaknuti. Budući instaleri moraju samostalno
obezbediti isti identitet i ikonicu.
Pogledaj [Tauri GTK identitet](https://v2.tauri.app/reference/config/#enablegtkappid).

Koristimo zvanični `@tabler/icons-svelte` paket. Uvozimo samo potrebne komponente,
koje nasleđuju boju preko `currentColor` i pakuju se za offline rad. Dekorativne
ikonice prate tekstualne oznake. Spoljni folder sa bibliotekom nije potreban.
Sačuvaj [Tabler MIT napomenu](../licenses/tabler-icons-MIT.txt) pri distribuciji.
Posle novih zavisnosti restartuj razvojnu komandu ako ih Vite ne prepozna.

## Provere

```sh
cargo fmt --all -- --check
cargo test --workspace --exclude librett-desktop
cargo clippy --workspace --exclude librett-desktop -- -D warnings
```

Posle instalacije sistemskih zavisnosti proveri i desktop adapter:

```sh
npm run build
cargo check -p librett-desktop
```

Ručna provera: napravi turnir sa srpskim slovima, dodaj singl kategoriju sa
grupama pa nokautom i dubl sa direktnim nokautom, proveri odbijanje duplikata,
restartuj i proveri podatke. Promeni jezik: poruke se menjaju, uneti nazivi
ostaju isti. Isključi mrežu i ponovi. Ovo su potrebne provere, ne tvrdnja da
su već uspešno izvršene.

## Linux grafika

Ako kompajliranje prođe, ali prozor padne uz `Error 71 (Protocol error)
dispatching to Wayland display`, probaj WebKitGTK podešavanje za jedno pokretanje:

```sh
WEBKIT_DISABLE_DMABUF_RENDERER=1 npm run desktop -- dev
```

Time se isključuje brži DMABUF put za taj proces; ovo nije globalno podešavanje
aplikacije. Pogledaj [Tauri uputstvo](https://v2.tauri.app/develop/debug/linux-graphics/).
Ako ne pomogne, proveravamo grafički drajver i sesiju pre drugog rešenja.
Vite/esbuild `EPIPE` nakon gašenja može biti posledica zaustavljenog procesa.

## Čistoća repozitorijuma

Na GitHub idu izvorni kod, migracije, specifikacije, dokumentacija i lock fajlovi.
Zavisnosti, build izlazi, lokalne baze, keš, tajne i podešavanja editora/agenta
se ne objavljuju. `.gitignore` ih isključuje. Istraživanja i jednokratne alate
drži van repozitorijuma. Instalacija zavisnosti pravi ignorisani `node_modules/`,
a build ignorisani izlaz. Privremeni keš i build mogu se držati van projekta:

```sh
npm ci --cache /tmp/librett-npm-cache
CARGO_TARGET_DIR=/tmp/librett-target cargo test --workspace --exclude librett-desktop
```

Pogledaj [ADR 0001](../adr/0001-desktop-foundation.md) i
[prihvatne scenarije](../../specs/tournament-setup.md).

## Blagajna

Otvori turnir i izaberi Blagajna. Na vrhu su preostalo dugovanje, neto primljeno
i broj različitih aktivno prijavljenih igrača. Tabela ima pretragu, red po igraču,
posebnu kolonu za svaku kategoriju, ukupno dugovanje i dugmad Naplati i Povraćaj.
Ako igrač nije prijavljen u kategoriju, prikazuje se crtica. Arhivirane kategorije
i povučene prijave zadržavaju finansije i oznake, ali ne povećavaju aktivni broj.

Checkbox-evi počinju bez kvačica i biraju kategorije za naplatu ili povraćaj. Klik na Naplati upisuje preostale
iznose zajedno, bez unosa iznosa ili napomene. Dubl od 500 RSD po paru deli se
na 250 RSD po igraču. Uplata jednog člana ne označava partnera kao plaćenog.
Plaćene kategorije dobijaju oznaku Plaćeno i mogu da se izaberu za povraćaj.
Posle uplate izbor se prazni; status ostaje vidljiv i nakon ponovnog otvaranja.

Povraćaj otvara potvrdu sa kategorijama i iznosima i vraća primljeni novac
samo za izabrane stavke tog igrača. Povraćaj ne povlači prijavu; to se uređuje
odvojeno u kategoriji. Povučene prijave ostaju dostupne za povraćaj.

Ranije delimične uplate, popusti i povraćaji ulaze u računanje. Ovaj pojednostavljeni
ekran nema ručni unos zaduženja/popusta/delimičnih uplata ni prikaz istorije;
svi finansijski zapisi ostaju sačuvani u bazi. Neizvestan upis zadržava isti UUID
i zaključava kontrole do potvrde ponavljanjem zahteva. U manjim prozorima kolone
kategorija skroluju se horizontalno. Nove prijave i dalje automatski dobijaju
kotizaciju kategorije; 0 znači besplatno učešće.

## Detalji kategorije

Klik na kategoriju otvara tab Prijave. Za singl čekiraj više igrača i prijavi ih
zajedno. Za dubl označi dva igrača, dodaj par i ponovi za sledeći par pre zajedničke
prijave. Već prijavljeni ostaju označeni i nisu dostupni za ponovnu prijavu.
Upis svih prijava i zaduženja je jedna transakcija. Žreb omogućava nacrte grupa
ili nokaut kostura; Mečevi i Rezultati su u pripremi.

Brisanje traži potvrdu: prazna kategorija briše se trajno, a kategorija sa prijavama
arhivira se i ostaje vidljiva u blagajni. Istorija navigacije objašnjava ako kategorija
više nije aktivna. Singl i dubl smeju da imaju isti naziv; duplikati iste discipline
nisu dozvoljeni.

## Radni prostor turnira

Otvaranje turnira prikazuje stalne kartice Pregled, Kategorije, Prijave, Blagajna,
Kategorije sadrže aktivne kategorije i obrazac za novu kategoriju. Prijave vode do
radnog prostora izabrane kategorije, gde se uređuju učesnici, dolasci i status
prijave. Svaka kategorija ima svoje kartice Prijave, Žreb, Mečevi i Rezultati;
Mečevi i Rezultati su za sada označeni kao delovi u pripremi. Blagajna prikazuje
postojeći pregled uplata po igraču. Pregled je početni ekran turnira. Prelazak
između kartica ulazi u istoriju Nazad/Napred, kao i otvaranje kategorije.


## Nacrt žreba

Otvori kategoriju → Žreb. Izaberi automatski ili ručni režim i dodaj nosioce od
najjačeg ka slabijima. Za grupe izaberi broj grupa i prolaznika iz svake.
Napravi raspored pravi približno jednake grupe ili pozicije prvog nokaut kola.
Automatika daje bye najjačim nosiocima; preostali bye se dodeljuju nasumično.
Izbor prijave na poziciji omogućava ručnu izmenu; već raspoređena prijava menja
mesto sa prethodnom na toj poziciji. Podešavanja grupa/režima važe za sledeće
pravljenje rasporeda, a uređivanje nosilaca menja i prikazani nacrt.

Sačuvaj nacrt upisuje novu verziju. Ručni nacrt može biti nepotpun. Sačuvaj pre
napuštanja ekrana: nesačuvane izmene postoje samo dok je ekran žreba otvoren.
Učitaj trenutno stanje odbacuje lokalne izmene. Promena prijava zahteva novi
nacrt. Sukob verzija odbija zastareli upis; neizvestan upis zadržava isti UUID i
zaključava navigaciju do potvrde ponavljanjem. Pre migracije postoji pre-v11 backup.
Potvrđivanje žreba, mečevi, rezultati, rangiranje i prolazak u nokaut još nisu
implementirani. Čuvanje nacrta ne pokreće takmičenje.

## Navigacija mišem i trackpadom

Strelice u aplikaciji, bočna dugmad miša i istorija webview-a koriste istu
istoriju ekrana i kartica. Na macOS-u je uključena podrška WKWebView-a za
horizontalno prevlačenje na trackpadu, uz dozvoljeno sistemsko podešavanje
prevlačenja između stranica. Horizontalni skrol ostaje sistemski. Tokom upisa
ni sistemska navigacija ne može promeniti ekran. Ponovno učitavanje aplikacije
počinje novu istoriju.


## Radni tabovi

Home je stalni ekran iza kućice gore levo i ne računa se među tabove. Izbor turnira
sa Home ekrana otvara radni tab. Naziv taba prikazuje neposrednog roditelja i otvorenu stranu, npr.
Veterani | Žreb; zadržavanje miša prikazuje pun kontekst. Povratak na Home
zadržava otvorene radne tabove. + i Cmd/Ctrl+T otvaraju Home za izbor novog radnog
prostora, bez pravljenja praznog Home taba.

Običan klik menja stranu u trenutnom tabu. Cmd/Ctrl+klik, srednje dugme miša ili
desni klik → Otvori u novom tabu otvaraju odredište zasebno. Desni klik na radni
tab otvara njegovo trenutno odredište u drugom tabu; ne kopira nesačuvanu formu.
×, srednje dugme na tabu i Cmd/Ctrl+W zatvaraju radni tab. Home se ne zatvara.
Ctrl+Tab / Ctrl+Shift+Tab menjaju Home i radne tabove. Strelice, Home i End pomeraju
izbor kada je fokus na listi radnih tabova.

Neaktivni radni prostori ostaju montirani i sakriveni, čuvajući forme, pretragu,
izbor, nacrt žreba, fokus i poziciju skrola. Svaki ima zasebnu istoriju; dugmad miša
i macOS gestovi koriste aktivni prostor. History API koristi centrirani most za
sistemske gestove, tako da se istorije različitih tabova ne mešaju. Tokom upisa i
dok je modalni dijalog otvoren ne može se menjati tab. Zatvaranje sa nesačuvanim
profilom, redom prijava, formom turnira/kategorije ili nacrtom žreba traži potvrdu.
Tabovi važe samo za trenutnu sesiju: restart ne vraća tabove ni nesačuvane forme.
Sistemsko close dugme zadržava svoje postojeće ponašanje.

Po povratku u tab osvežavaju se podaci blagajne, prijava, baze igrača i žreba.
Važeći izbori ostaju, a nesačuvani nacrt se zadržava; promenjene prijave ili verzije
proveravaju se pri čuvanju.

Gornja traka od 48px je fiksna uz prozor. Visina radnog prostora i bočnog menija
oduzima njenu visinu, pa kratke strane nemaju nepotreban vertikalni skrol.
Na macOS-u Overlay i hiddenTitle postavljaju sistemske dugmiće uz Home i tabove;
AppKit koordinate centriraju stvarne dugmiće u istom redu pri otvaranju, promeni
veličine i fokusu. Za promene sistemske trake restartuj desktop proces. Ostali
sistemi zadržavaju standardnu naslovnu traku iznad radnih tabova. Prazan prostor
u redu omogućava prevlačenje prozora.

Radni tabovi imaju fiksnu širinu 180px bez obzira na dužinu naziva. Tekst bledi sa
desne strane samo kada zaista ne staje; ne koristi tri tačke. Pun naziv/kontekst
je dostupan na zadržavanje miša, a lista tabova se skroluje horizontalno.


## Sklopivi levi meni

Dugme desno od logotipa sužava meni na 68px; proširen meni ima 208px, odnosno 184px
na manjim desktop prozorima. Sužen meni prikazuje ikonice navigacije sa pristupačnim
nazivima i opisom na zadržavanje miša, kao i `icon-dark.png` u tamnoj temi ili
`icon-light.png` u svetloj. Klik na malu ikonicu logotipa proširuje meni, bez
zasebne ikonice za proširenje. Ni pun logo ni mala ikonica ne vode na Home. Izbor važi za Home
i sve radne tabove i pamti se u local storage-u kao `librett.sidebarCollapsed`,
i posle restarta aplikacije. Promena teme menja i malu ikonicu brenda. Sužavanje
menja raspored bez gubitka stanja otvorene strane.


## Podešavanje kategorije i pregled žreba

Turnir ima tabove Pregled, Kategorije i Blagajna. Lista kategorija ima Dodaj,
Izmeni i Obriši u istom stilu kao lista igrača. Dodavanje i izmena otvaraju poseban
ekran; klik na kategoriju otvara njene Prijave. Kategoriji sa prijavama nije moguće
menjati disciplinu ili format. Promena kotizacije važi za buduće prijave.

Podešavanja kategorije sadrže pravila, učesnike, redosled nosilaca i automatski ili
ručni raspored. Pravila uključuju broj grupa, broj prolaznika, broj setova, poene,
razliku za pobedu i redosled kriterijuma: mini-tabela izjednačenih, odnos setova i
odnos poena. Kategorije sa grupama imaju poseban tab Grupe sa round-robin parovima i
karticama u najviše tri kolone (dve ili jedna na užem prozoru). Žreb prikazuje
samo ceo nokaut kostur sa horizontalnim skrolom. Prolaznici grupa su označena buduća mesta; obračun
rezultata, rangiranje i napredovanje čekaju implementaciju mečeva.

SQLite šema 11 čuva verzije pravila i preuzima postojeća podešavanja grupa uz
pre-v11 backup. Vidi [ADR 0012](../adr/0012-category-rules-and-draw-view.md).


Skrol radnog prostora je u zasebnom kontejneru ispod gornje trake od 48px.
Dokument ne skroluje, a vertikalni skrol se ne prenosi na njega. Sidebar i toolbar
vezani su za vrh kontejnera; promena taba vraća njegovu poziciju skrola. Tako macOS
efekat istezanja na krajevima skrola ne pomera gornje tabove.
