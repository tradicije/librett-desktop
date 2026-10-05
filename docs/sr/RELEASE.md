# Beta instaleri i GitHub izdanja

[English](../en/RELEASE.md)

Prva beta nosi oznaku **0.1.0-beta.1**. Pravljenje instalera ne objavljuje izdanje.
Sudije i telefoni nisu deo prve bete. Provera na pravom turniru i ručna provera
instalacije na svakoj platformi još predstoje.

## Paketi

| Platforma | Fajl za izdanje | Instalacija |
| --- | --- | --- |
| macOS Apple Silicon | `LibreTT_0.1.0-beta.1_macos-arm64.dmg` | Otvori DMG i prevuci LibreTT u Applications |
| macOS Intel | `LibreTT_0.1.0-beta.1_macos-x64.dmg` | Otvori DMG i prevuci LibreTT u Applications |
| Windows x64 | `LibreTT_0.1.0-beta.1_windows-x64.exe` | Pokreni instalacioni čarobnjak |
| Linux x64 | `LibreTT_0.1.0-beta.1_linux-x64.deb` | Instaliraj sistemskim upravljačem paketa |
| Linux x64 | `LibreTT_0.1.0-beta.1_linux-x64.AppImage` | Omogući izvršavanje fajla i pokreni ga |

Windows paket uključuje offline instalaciju WebView2, pa je veći, ali ne mora
da preuzima runtime tokom instalacije. Linux i dalje zahteva kompatibilne
sistemske biblioteke; CI koristi Ubuntu 22.04. Za Linux i Windows pripremamo
x64 pakete, a za macOS odvojene Apple Silicon i Intel pakete.

macOS ima **ad-hoc potpis**, bez Apple Developer ID potpisa i notarizacije.
Windows nema sertifikat izdavača. Sistem može da blokira preuzetu betu ili
prikaže upozorenje. To navedi u opisu izdanja; na macOS-u za pouzdanu betu koristi
**Privacy & Security → Open Anyway**. Ne traži od korisnika da globalno isključi
zaštitu sistema. Potpisivanje za javnu distribuciju zahteva posebne sertifikate
i pristupne podatke. Pogledaj [Tauri macOS potpisivanje](https://v2.tauri.app/distribute/sign/macos/)
i [Windows potpisivanje](https://v2.tauri.app/distribute/sign/windows/).

## Lokalni build

Instaliraj razvojne zahteve iz [DEVELOPMENT.md](DEVELOPMENT.md), pa na odgovarajućem
sistemu pokreni iz korena repozitorijuma:

```sh
npm ci
npm run desktop:build -- --ci -- --locked
```

Paketi su u `target/release/bundle/`. Za kopiranje u folder za izdanje i SHA-256
kontrolne sume izaberi odgovarajuću arhitekturu:

```sh
npm run desktop:collect -- target/release/bundle macos-arm64
```

Ostale oznake su `macos-x64`, `windows-x64` i `linux-x64`. Pripremljeni fajlovi
su u `release-assets/<platform>/`, koji Git ignoriše. Mora da postoji tačno jedan
paket svakog očekivanog formata, a npm, Cargo i Tauri verzije moraju da se poklapaju.
Ako u build folderu ostanu stari paketi, koristi čist checkout/build folder.
Sa `--target <triple>` putanja je `target/<triple>/release/bundle/`; nju prosledi
komandi za prikupljanje paketa.

## Sve platforme preko GitHub Actions

1. Commituj i pushuj izmene za pakovanje.
2. Otvori **Actions → Build beta installers → Run workflow** i izaberi granu.
3. Sačekaj sva četiri platform job-a. Svaki dodaje artifact `LibreTT-<platform>`
   sa instalacionim fajlovima i `SHA256SUMS-<platform>.txt`.
4. Preuzmi i raspakuj sve artifact-e. Čuvaju se 30 dana. Proveri instalacije pre
   objavljivanja; uspešan CI potvrđuje build, ne rad na pravom turniru.

Push taga `v*` takođe pokreće build. Tag treba da odgovara verziji u aplikaciji,
npr. `v0.1.0-beta.1`. Workflow ima samo pravo čitanja repozitorijuma i ne pravi,
ne menja i ne objavljuje GitHub release. Dostupnost runner-a prati
[GitHub dokumentaciju](https://docs.github.com/en/actions/reference/runners/github-hosted-runners).

## Ručno objavljivanje

Napravi tag za isti commit od kog su uspešno napravljeni instaleri. U GitHub-u
otvori **Releases → Draft a new release**:

- Izaberi `v0.1.0-beta.1`, sa naslovom `LibreTT 0.1.0-beta.1`.
- Označi **Set as a pre-release**.
- Dodaj raspakovane DMG, EXE, DEB i AppImage fajlove i sve četiri kontrolne sume.
  Dodaj same instalere, ne ZIP arhive preuzete iz Actions-a.
- Opiši funkcije, ograničenja bete i instalaciju, uz sažetak changelog-a.
- Zadrži tag i arhive izvornog koda za odgovarajuće AGPL izdanje.

Predlog opisa:

> Prva javna beta za lokalnu organizaciju stonoteniskih turnira. Igrači,
> kategorije, prijave, žreb, grupe/nokaut, rezultati i plasmani, blagajna,
> ručna dodela stolova, rezervne kopije i štampa/izvoz. Sudije i telefoni nisu
> uključeni. Provera na pravom turniru još predstoji. macOS ima ad-hoc potpis
> bez notarizacije; Windows nije potpisan sertifikatom izdavača.

Identifikator ostaje `org.librett.desktop`: beta koristi isti lokalni folder
podataka kao razvojna aplikacija. Zaustavi dev aplikaciju i napravi ručni bekap
pre prelaska. Instaleri sadrže aplikaciju i licence, ne tvoju lokalnu bazu.
Na prvom turniru zadrži bekap i paralelnu evidenciju dok ne proveriš rad uživo.

Za sledeću betu zajedno ažuriraj `package.json`, `apps/desktop/package.json`,
`package-lock.json`, Cargo workspace/lock verzije i verziju u Tauri konfiguraciji.
