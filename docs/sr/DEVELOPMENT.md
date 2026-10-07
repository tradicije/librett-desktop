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
Singl/dubl mečevi podržavaju rezultate po setovima, predaju i nedolazak.
Tabele grupa i kvalifikacije za nokaut se ažuriraju automatski. Dostupni su
bekap/vraćanje, ručna dodela stolova i štampa/izvoz. Verzija `0.1.0-beta.1` je
kandidat za prvu betu; vidi [uputstvo za instalere i izdanje](RELEASE.md).

Tab Igrači prikazuje zajedničku listu. Dodaj/Izmeni otvara zasebne ekrane za
profile i fotografije, a čuvanje vraća na listu. Brisanje traži potvrdu i čuva
igrače sa postojećim prijavama. Prijave unutar turnira koriste tu bazu i mogu
da se povuku i vrate;
dolazak igrača važi kroz sve kategorije istog turnira. Šema verzije 20 sadrži neizmenjive nacrte žreba i verzije pravila kategorije; pre migracije starijih baza pravi se konzistentan
`pre-v20-<uuid>.sqlite` backup.

Nazad/Napred prati istoriju aktivnog radnog taba. Home dugme u gornjoj traci
vraća na stalni početni ekran koji nema bočni meni. Čuvanje zaključava navigaciju do završetka.
Lokalna administracija za sada nema prijavu nalogom niti kontrolu korisničkih uloga.

## Naziv desktop aplikacije

Korisnički naziv je **LibreTT** na macOS-u, Linuxu i Windowsu. Tauri
`productName` je LibreTT, a Cargo ima eksplicitni izvršni fajl i podrazumevani
run target `LibreTT` (`LibreTT.exe` na Windowsu). Interni Cargo paket ostaje
`librett-desktop`, pa komande poput `cargo build -p librett-desktop` i dalje
rade. Naziv samog izvršnog fajla pokriva razvojni režim u kome operativni sistem
koristi ime fajla umesto podataka o proizvodu.

macOS dodatno ima CFBundleName/CFBundleDisplayName u Info.plist fajlu i naziv
NSProcessInfo procesa pre inicijalizacije AppKit-a. Linux postavlja GLib naziv
aplikacije LibreTT, uz isti GTK identitet `org.librett.desktop` za povezivanje
sa desktop zapisom. Razvojni desktop zapis koristi `Exec=LibreTT`; posle izmene
postojećeg zapisa na Linuxu ponovi `npm run desktop:install-icon`.
Native build prati Info.plist i ikonice na svim platformama. Posle native
promene potpuno zaustavi i ponovo pokreni `npm run desktop -- dev`.

## Teme i ikonice

Izbor teme podržava Svetla, Tamna i Sistemska. Sistemska prati promene OS teme
dok aplikacija radi. Tema i jezik su lokalne UI postavke; podaci turnira ostaju
u SQLite bazi. `assets/img/logo-light.png` i `logo-dark.png` se pakuju u aplikaciju
i rade offline.

Izvor desktop ikonice je `assets/img/app-icon.png`. Tauri PNG, ICO i ICNS
varijante su u `apps/desktop/src-tauri/icons/`: to su aplikacioni resursi,
a ne razvojni keš. Posle izmene izvorne slike, iz korena projekta pokreni:

```sh
npm run desktop:generate-icons
```

Komanda koristi instalirani Tauri CLI i kopira samo desktop PNG/ICO/ICNS
resurse u projekat za sve tri platforme: Linux PNG, Windows ICO/Appx i macOS
ICNS. Zamena izvornog PNG-a sama ne ažurira ove fajlove.
macOS u razvojnom režimu koristi `icons/icon.icns` za ikonicu u Dock-u;
ikonice prozora koriste generisane PNG slike. Potpuno zaustavi desktop proces
(Ctrl+C u terminalu gde je pokrenut), pa ponovo pokreni `npm run desktop -- dev`.
Frontend hot reload ne ažurira ugrađene native ikonice. Native `build.rs` prati
generisani folder `icons`, tako da Cargo ponovo kompajlira ugrađene ikonice
kada se ti fajlovi promene. Za distribuiranu `.app`
aplikaciju potreban je ponovni build sa novim ikonicama.

Generator za macOS ICNS dodaje transparentnu marginu od 100px sa svake strane
slike 1024px: motiv zauzima 824px, oko 80% širine. Time motiv koji u izvornom
PNG-u ide do ivica ne izgleda preveliko u Dock-u. Privremeni SVG koristi originalni
PNG, a margina se ponovo primenjuje pri svakoj zameni `app-icon.png` i generisanju.
Linux/Windows resursi koriste izvornu veličinu motiva.

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
ili nokaut kostura. Mečevi podržavaju unos; konačan plasman je u pripremi.

Brisanje traži potvrdu: prazna kategorija briše se trajno, a kategorija sa prijavama
arhivira se i ostaje vidljiva u blagajni. Istorija navigacije objašnjava ako kategorija
više nije aktivna. Singl i dubl smeju da imaju isti naziv; duplikati iste discipline
nisu dozvoljeni.

## Radni prostor turnira

Otvaranje turnira prikazuje stalne kartice Pregled, Kategorije, Prijave, Blagajna,
Kategorije sadrže aktivne kategorije i obrazac za novu kategoriju. Prijave vode do
radnog prostora izabrane kategorije, gde se uređuju učesnici, dolasci i status
prijave. Svaka kategorija ima svoje kartice Prijave, Žreb, Mečevi i Rezultati;
Mečevi podržavaju unos rezultata; Rezultati su za sada u pripremi. Blagajna prikazuje
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
zaključava navigaciju do potvrde ponavljanjem. Pre migracije postoji pre-v18 backup.
Potpun i važeći žreb određuje mečeve u tabu Mečevi. Unos rezultata i nokaut
napredovanje, tabele grupa i kvalifikacije su dostupni; životni ciklus
takmičenja ostaje u pripremi.

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

Podešavanja kategorije sadrže pravila, redosled nosilaca i automatski ili
ručni raspored. Pravila uključuju broj grupa, broj prolaznika, broj setova, poene,
razliku za pobedu i redosled kriterijuma: mini-tabela izjednačenih, odnos setova i
odnos poena. Kategorije sa grupama imaju poseban tab Grupe sa round-robin parovima i
karticama u najviše tri kolone (dve ili jedna na užem prozoru). Žreb prikazuje
ceo nokaut kostur sa horizontalnim skrolom. Nokaut kategorije prikazuju rezultate,
napredovanje pobednika i šampiona. Prolaznici grupa ostaju označena buduća mesta
dok se grupe ne završe i ne razreše izjednačenja.

SQLite šema 11 čuva verzije pravila i preuzima postojeća podešavanja grupa uz
pre-v11 backup. Vidi [ADR 0012](../adr/0012-category-rules-and-draw-view.md).


Skrol radnog prostora je u zasebnom kontejneru ispod gornje trake od 48px.
Dokument ne skroluje, a vertikalni skrol se ne prenosi na njega. Sidebar i toolbar
su izvan prostora sadržaja koji skroluje; promena taba vraća poziciju sadržaja.
Breadcrumbs otvaraju roditeljske ekrane i podržavaju otvaranje u novom tabu. Tako macOS
efekat istezanja na krajevima skrola ne pomera gornje tabove.


Info dugme skroz desno u gornjoj traci otvara prozor O aplikaciji sa logom,
opisom, verzijom iz metapodataka paketa, autorom i punim tekstom AGPL licence
koji je dostupan bez interneta. Escape ili Zatvori zatvara prozor.


## Zaštićeni upisi i naslovne slike turnira

Navigacija, Back/Forward, otkazivanje editora i učitavanje nacrta traže potvrdu
pre odbacivanja izmena. Zatvaranje prozora i Quit proveravaju sve radne tabove.
Upisi igrača imaju stalne ID-jeve, proveru originalnog profila i potvrde zahteva
za bezbedan ponovni pokušaj. Konflikt se rešava potvrđenim ponovnim učitavanjem.
Finansijske akcije šalju očekivane iznose; promenjeno stanje traži novu potvrdu.

Turniri se prikazuju kao kartice u najviše tri kolone. Dodaj turnir otvara poseban
editor sa opcionom naslovnom slikom 16:9. Broj prijavljenih je broj jedinstvenih
aktivnih igrača u aktivnim kategorijama. Parovi grupa računaju se tek pri
otvaranju, po jednom kolu i sa 32 para po strani. Dimenzije slika proveravaju se
pre dekodiranja, a backend u potpunosti dekodira JPEG uz ograničenje memorije.
Vidi [ADR 0013](../adr/0013-guarded-writes-and-bounded-images.md).

Podešavanja turnira omogućavaju izmenu naziva i naslovne slike. Čuvanje proverava
izvorne podatke u transakciji i odbija konfliktne izmene; ponavljanje već
primenjenog zahteva je bezbedno. Cela kartica otvara turnir klikom ili tastaturom,
a dugme na dnu podržava i otvaranje novog radnog taba.

`ImageCropDialog.svelte` prvo proverava sliku preko `readBoundedImage`, pa
otvara pregled za kropovanje. Fotografije igrača se čuvaju kao 512×512 (1:1),
a naslovne slike turnira kao 1024×576 (16:9), bez promene proporcija. Kadar
se pomera mišem, dodirom ili klizačima dostupnim tastaturom, uz zum 1×–4×.
Primeni čuva JPEG; Otkaži zadržava postojeću sliku. Nakon zatvaranja je moguće
ponovo izabrati istu datoteku.

## Tabele grupa i kvalifikacije

Tab Grupe prikazuje odigrane mečeve, pobede/poraze, setove, poene i plasman.
Izjednačenja se računaju preko mini-tabele i kriterijuma kategorije. Završena
i razrešena grupa popunjava svoja mesta u nokaut kosturu. Dugme Plasman nudi
ručni redosled i povratak na automatski obračun; novi rezultat grupe vraća
automatski redosled. Tab Mečevi omogućava izbor grupne ili nokaut faze.
Ispravke koje menjaju učesnike odigranih nokaut mečeva traže potvrdu pre
poništavanja tih rezultata. Nokaut kola nose nazive faza, umesto brojeva.

Šema 15 čuva ručni plasman grupa. Obračun statistike predaje/nedolaska,
zaštita upisa i osvežavanje prikaza opisani su u
[ADR 0015](../adr/0015-group-standings-and-qualification.md).

## Starosne grupe kategorija

Godište je obavezno u formi igrača i pri čuvanju kroz desktop komandu. Stari
profili bez godišta ostaju čitljivi; godište se dopunjava pri uređivanju.
Pravila kategorije čuvaju `age_enabled`, `age_min` i `age_max`. Prekidač je
podrazumevano isključen (sva godišta). Uključivanjem se otvaraju obavezna polja
Od/Do: cele godine 0–130, sa uključenim granicama i Od ≤ Do. Isključen prekidač
čuva prazne granice. Pravila su dostupna pri dodavanju, izmeni i u podešavanjima.
Stara JSON pravila dobijaju podrazumevane vrednosti bez migracije baze.

Pre prijave osvežavaju se pravila i profili. Starost se računa kao tekuća lokalna
kalendarska godina minus godište, bez datuma rođendana ili datuma turnira.
Upozorenje navodi igrače van raspona i one bez poznatog godišta, uključujući oba
člana dubla. Organizator može da potvrdi izuzetak; otkazivanje zadržava izbor.
Ista provera važi za vraćanje povučene prijave. Backend ne zabranjuje izuzetke,
a promena raspona ne briše postojeće prijave.

## Lucky loser popunjavanje kostura

U pravljenju/izmeni kategorije ili Podešavanjima izaberi BYE (podrazumevano,
kompatibilno sa postojećim pravilima) ili Lucky loser. Veličina kostura ostaje
sledeći stepen dvojke prema direktnim prolaznicima: 5 → 8, 7 → 8, 9 → 16.
Broj kandidata ne povećava kostur.

Kada sačuvaš raspored grupa, Podešavanja prikazuju Lucky loser mesta. Automatski
izbor popunjava sva moguća mesta tek kada su sve grupe završene i plasman
razrešen. Sve automatski uklanja ručne izmene; na svakom mestu možeš izabrati
igrača ispod crte ili BYE i sačuvati. Nema ponovljenih igrača ni direktnih
prolaznika među kandidatima. Ako kandidata nema dovoljno, ostatak postaje BYE.
Nerazrešeno mesto nikada ne daje slobodan prolaz.

Redosled kandidata: plasman u grupi rastuće, pa procenat pobeda, odnos setova i
odnos poena opadajuće. Poređenja koriste cele brojeve. Ako izjednačenje prelazi
granicu automatskog izbora, sva sporna mesta čekaju ručni izbor. Ovo je
dogovorena politika aplikacije, bez tvrdnje o pravilniku određenog saveza.
Pregled kandidata pokazuje statistiku. Kostur koristi LL i grupu/plasman,
sa očuvanjem oznake u kasnijim kolima.

Ručni izbor je vezan za sačuvani žreb. Ostaje ako je igrač i dalje ispod crte;
ako postane nevažeći, mesto čeka proveru. Novi žreb nema ručne izbore.
Prelazak na BYE čuva neaktivne izbore za eventualni povratak na Lucky loser,
uz ponovnu proveru kandidata.

`save_knockout_fillers` koristi neizmenjivi UUID zahteva, očekivane verzije
izbora/pravila, plasmana/rezultata grupa i svih rezultata mečeva. Zastareo upis
vraća `match_conflict`, a nepotvrđen upis ponavlja isti zahtev. Promena učesnika
zahteva potvrdu pre poništavanja zavisnih nokaut rezultata; u istoj transakciji
se dodaju null verzije, a istorija ostaje. Isto važi za pravila koja menjaju
učesnike sa sačuvanim nokaut rezultatima. Šema 16 i politika izbora opisani su
u [ADR 0016](../adr/0016-lucky-loser-knockout-filling.md).

Turnir i same prijave nemaju fiksno ograničenje broja igrača. Žreb podržava
2–4096 prijava u jednoj kategoriji; kod dubla prijava znači par. To je granica
u kodu, bez potvrde performansi za maksimalan broj učesnika.

## Konačni plasman i završavanje takmičenja

Tab Rezultati prikazuje napredak i razloge zbog kojih kategorija još nije spremna.
Potreban je važeći, potpun žreb, završene grupe sa razrešenim plasmanom,
popunjeni prolaznici/LL mesta i svi potrebni nokaut rezultati, uključujući finale.

Pobednik je prvi, poraženi finalista drugi, a poraženi polufinalisti dele treće
mesto bez meča za bronzu. Ostali dele raspone po nokaut fazi, prema stvarnom
broju prijava. Učesnici koji nisu prošli grupe dele preostali raspon; ne izmišlja
se redosled između različitih grupa. Dubl se rangira po paru. Imena i oznake
klubova koriste istorijski snimak učesnika iz žreba.

Završi kategoriju traži potvrdu, čuva snimak konačnog plasmana i vreme završetka,
pa zaključava prijave, pravila/podatke kategorije, žreb, ručni plasman grupa,
LL izbore i rezultate. Rezultati ostaju dostupni za čitanje. Pregled turnira
prikazuje napredak i pobednike kategorija. Završi turnir zahteva najmanje jednu
aktivnu kategoriju i potvrđen završetak svake aktivne kategorije. Nekorišćene
kategorije prethodno ukloni ili arhiviraj.

Zahtev za potvrdu kopira se iz običnog snimka Svelte stanja. Greške tokom
pripreme zahteva prikazuju se u dijalogu, umesto da neprimetno zaustave akciju.

Ponovno otvaranje traži zasebnu potvrdu. Prvo se otvara turnir, pa kategorija;
otvaranje turnira ne otključava završene kategorije automatski. Istorija
prethodnih završetaka i ispravki rezultata ostaje. Novo završavanje čuva novi
snimak. Blagajna, globalni profili igrača i naziv/naslovna slika turnira ostaju
dostupni; dolasci se zaključavaju nakon završetka celog turnira.

`change_completion` koristi neizmenjivi UUID zahteva, očekivanu verziju statusa
i deterministički token žreba/pravila/rezultata/grupnog plasmana/LL izbora i
statusa roditelja. Turnirski token uključuje listu aktivnih kategorija i njihove
verzije. Transakcija ponovo proverava uslove pre upisa; zastareli dijalog vraća
`completion_conflict`. Ponavljanje istog zahteva vraća sačuvan odgovor, čak i
posle kasnijih promena statusa. UI osvežava statuse kroz radne tabove, a backend
štiti i stare otvorene editore.

Šema 17 dodaje statuse, snimke i neizmenjivu tabelu `completion_history`.
SQL trigger-i dodatno zaključavaju takmičarske podatke, a aplikaciona zaštita
vraća `competition_closed`. Postojeći turniri/kategorije ostaju otvoreni nakon
migracije. Vidi [ADR 0017](../adr/0017-final-standings-and-completion.md).

## Uređivanje kostura i treće mesto

U kategoriji otvori **Žreb → Uredi**, pa klikni na igrača ili BYE u prvoj rundi. Biraj između prijava te kategorije; izbor već raspoređenog učesnika menja njihova mesta. Izmene se čuvaju posebno. Naredne runde prate pobednike mečeva i ne menjaju se nezavisno. Izmene direktnog nokauta prave novu verziju žreba i zadržavaju prethodne rezultate u istoriji. Nokaut posle grupa uređuje se kada su sve grupe završene i plasman razrešen; poništavanje zavisnih rezultata traži potvrdu.

Pravljenje i podešavanja kategorije imaju tri prekidača u jednom redu (na uskim prozorima jedan ispod drugog): starosna grupa, Lucky loser/BYE i jedno treće mesto. Lucky loser je dostupan za grupe → nokaut, dok direktni nokaut koristi BYE. Isključen prekidač trećeg mesta znači zajedničko treće mesto. Uključen otvara izbor meča za bronzu ili dodelu trećeg mesta polufinalisti koji je izgubio od kasnijeg pobednika; drugi poraženi polufinalista je četvrti. Meč za bronzu je u završnoj rundi Mečeva i ispod kostura; kategorija čeka njegov rezultat. Ako zbog BYE postoji samo jedan poraženi polufinalista, on je treći bez dodatnog meča.

Uključen Lucky loser prekidač otvara automatsko ili ručno popunjavanje. U ručnom režimu prazna mesta čekaju izbor učesnika ili BYE. Kartice baze igrača prikazuju samo ime/klub i starost (tekuća godina minus godište); ostali podaci su u profilu.

## Rezervne kopije, ručni stolovi i izveštaji

Šema 18 dodaje tournament_scheduling, match_assignments i schedule_writes. Sto se ručno unosi u editoru meča (1–128), uz posebno dugme Sačuvaj sto. Prazno polje + čuvanje uklanja dodelu. Transakcije proveravaju verzije i koriste neizmenjive potvrde upisa. Dodeljeni mečevi rezervišu igrače kroz sve kategorije, uključujući oba člana dubla. Rezultat ili promenjeni učesnici oslobađaju sto; broj stola ostaje zabeležen za završeni meč.

Stranica Rezervne kopije u bočnom meniju koristi SQLite Backup API za konzistentne snimke. Provera pri pokretanju i svakog sata pravi najviše jednu dnevnu kopiju, uz najviše 14 automatskih kopija; ručne kopije i kopije pre vraćanja ostaju. Vraćanje proverava šemu, identitet aplikacije, integritet, reference i turnire u privremenoj kopiji pre zaštitnog snimka i zamene baze. Uvoz iz fajla prihvata SQLite kopije do 256 MiB. Posle vraćanja frontend se ponovo učitava i tabovi zatvaraju, uz proveru nesačuvanih izmena.

Izveštaji su dostupni u Grupama/Žrebu, Rezultatima i Blagajni. CSV koristi UTF-8 BOM, navodnike i zaštitu od formula. HTML kodira korisničke vrednosti. Izveštaj kategorije čita stanje u jednoj transakciji. Poseban pregled otvara sistemski dijalog za štampu, uključujući čuvanje PDF-a. Listovi kostura imaju najviše 32 početna učesnika po bloku. Fajlovi idu u Downloads/LibreTT (rezervno u app data); dugme Otvori folder prikazuje lokaciju.

Provere: `npm run check`, `npm run build`, `npm run test:reports` i `cargo test --workspace --exclude librett-desktop --offline`. Sistemske dijaloge za štampu i otvaranje foldera treba ručno proveriti po platformama.

## Korpa za turnire

Šema 19 uvodi `tournament_trash`, vezanu za originalni turnir bez uklanjanja njegovih podataka. Aktivna lista i dohvat turnira isključuju turnire u korpi; posebna lista korpe zadržava ceo sažetak turnira. Premeštanje i vraćanje mogu bezbedno da se ponove, a završeni turniri zadržavaju status završetka. SQLite okidači odbijaju izmene sportskih podataka, metapodataka, rasporeda i blagajne turnira u korpi, uključujući zahteve iz ranije otvorenih radnih prostora. Zajednički profili igrača ostaju dostupni za izmenu. Otvoreni tabovi prikazuju obaveštenje kada njihov turnir nestane iz deljene aktivne liste. Rezervne kopije čuvaju korpu; kopije šeme 18 migriraju se i proveravaju pre vraćanja. Nema komande za trajno brisanje niti automatskog isteka.

## Tok dolaska i naplate

Dolazak važi za ceo turnir i ne menja pripremljeni žreb. Naplata proverava aktivnu prijavu/kategoriju i dolazak platioca unutar neposredne transakcije. Naplata cele prijave zahteva dolazak svih članova; pojedinačna naplata dubla samo dolazak platioca. Povraćaj ne zahteva aktivnu prijavu ni potvrđen dolazak. Uspešno upisani zahtevi ostaju idempotentni i nakon promene uslova; izbor više stavki se u celosti poništava ako jedna više ne ispunjava uslove.

Originalna zaduženja ostaju neizmenjivi istorijski zapisi. `cash-balance.ts` izvodi dug samo za prisutne aktivne igrače, procenu samo za nepotvrđene aktivne igrače, a zadržava neto uplate neaktivnih prijava radi povraćaja. Izvoz blagajne koristi isti račun i označava bruto zaduženja kao istorijska. Opoziv dolaska blokira nove naplate bez brisanja ranijih transakcija. Provere: `npm run test:cash` i Rust testovi radnog prostora.

Kategorija počinje prvim upisanim rezultatom ili početkom meča na dodeljenom stolu. Promene prijava tada su zaključane, dok dolasci i dalje mogu da se potvrde. Prvi rezultat proverava dolaske svih aktivnih učesnika; izričiti izuzetak `allow_unconfirmed_start` čuva se u potvrdi upisa i ne menja dolaske. Podrazumevana false vrednost zadržava format postojećih potvrda. Početak meča na stolu takođe proverava dolaske. Predaje i walkover čuvaju sportsku istoriju kroz postojeći tok unosa rezultata. Nije potrebna migracija šeme.

## Globalna istorija akcija

Istorija u levom meniju prikazuje sačuvane akcije svih turnira, najnovije prvo (vreme i stabilan redni broj). Organizator može da kombinuje vrste istorije i filtrira akciju, turnir i kategoriju. Rezultati prikazuju izmene setova, pravila svoje parametre, a žreb nosioce i raspored. Stariji zapisi učitavaju se po 50, bez pomeranja stranica kada pristignu novi zapisi. Promena filtera vraća prikaz na početak. Osveži ili ponovo aktiviraj tab Istorija za izmene iz drugih tabova.

Šema 20 dodaje neizmenjivu `action_history`; SQLite okidači upisuju promene u istoj transakciji kao turniri/kategorije/igrači, prijave/učesnici i dolasci, pravila, žreb, rezultati i poništavanja, plasman grupa/nokaut mesta, blagajna/raspodele, stolovi, završavanje/otvaranje i korpa. Kreiranje/vraćanje bekapa i izvoz fajla beleže se nakon uspešne operacije. Navigacija, neuspešni/poništeni upisi i ponovljeni zahtevi koji ne menjaju bazu ne stvaraju događaje izmene podataka. Slike se ne dupliraju; beleži se promena i prisustvo slike.

Migracija prenosi postojeće datirane žrebove, rezultate, blagajnu, završetke i korpu kao označenu prethodnu istoriju. Ranije izmene bez sačuvanog vremena/istorije ne mogu da se rekonstruišu; stari kontekst koristi nazive dostupne pri migraciji. Ne izmišljamo identitet organizatora: aplikacija je lokalna, bez prijavljenih naloga. Istorija je deo bekapa. Vraćanje bekapa vraća i njegov snimak istorije, uz novi događaj vraćanja; ovo nije izdvojen forenzički dnevnik koji preživljava sva vraćanja. Sigurnosna kopija pre vraćanja čuva prethodnu bazu i istoriju. Istorija nema komandu za brisanje ili izmenu.

### Provere grešaka pri učitavanju

Istorija resetuje prikazane zapise i paginaciju pri novom filtriranom upitu; neuspešan upit ne koristi kursor drugog filtera. Mečevi objavljuju stranicu, raspored i brojeve stolova zajedno, tek po uspešnom učitavanju svih odgovora. Regresioni testovi istorije pokrivaju nazive obrisanih kategorija u filterima, paginaciju zapisa sa istim vremenom i poništavanje istorije zajedno sa neuspelom transakcijom.
