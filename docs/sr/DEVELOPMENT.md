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
Singl/dubl i formati se čuvaju kao podešavanja; žreb i mečevi još nisu
implementirani. Još nema rezultata, interfejsa za izvoz/oporavak
ili instalera. Razvojna verzija `0.1.0` nije objavljeno izdanje.

Tab Igrači prikazuje zajedničku listu. Dodaj/Izmeni otvara zasebne ekrane za
profile i fotografije, a čuvanje vraća na listu. Brisanje traži potvrdu i čuva
igrače sa postojećim prijavama. Prijave unutar turnira koriste tu bazu i mogu
da se povuku i vrate;
dolazak igrača važi kroz sve kategorije istog turnira. Šema verzije 9 dodaje uplate po igraču i zaštitu od ponavljanja naplate; pre migracije starijih baza pravi se konzistentan
`pre-v9-<uuid>.sqlite` backup.

Nazad/Napred prati istoriju ekrana. Home vraća na pregled izabranog modula, zatim
na izbor modula koji nema bočni meni. Čuvanje zaključava navigaciju do završetka.
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
Upis svih prijava i zaduženja je jedna transakcija. Žreb, Grupe i Kostur trenutno
prikazuju da su u pripremi; čisti nokaut nema tab Grupe.

Brisanje traži potvrdu: prazna kategorija briše se trajno, a kategorija sa prijavama
arhivira se i ostaje vidljiva u blagajni. Istorija navigacije objašnjava ako kategorija
više nije aktivna. Singl i dubl smeju da imaju isti naziv; duplikati iste discipline
nisu dozvoljeni.
