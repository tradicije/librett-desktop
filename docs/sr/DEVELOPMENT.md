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
implementirani. Još nema evidencije dolazaka, blagajne, rezultata, interfejsa za izvoz/oporavak
ili instalera. Razvojna verzija `0.1.0` nije objavljeno izdanje.

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
