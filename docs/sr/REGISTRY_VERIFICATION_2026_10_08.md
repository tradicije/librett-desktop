# Razvojne provere — 2026-10-08

[English](../en/REGISTRY_VERIFICATION_2026_10_08.md)

Autorizovana implementacija/testiranje svih šest koraka kataloga/Desktop integracije. Produkcioni release nije napravljen. Testovi koriste izdvojene sintetičke baze/fajlove; instalirana korisnička Desktop baza nije otvorena.

## Rezultati

| Provera | Stvarni rezultat |
| --- | --- |
| Registry PHPUnit unit | 47 testova / 80 assertions prošlo |
| Registry WordPress/MariaDB integracija | 41 test / 142 assertions prošlo |
| PHPStan | Maksimalni nivo, sve src klase, bez grešaka |
| PHP CS Fixer | 120 PHP fajlova, nema potrebnih izmena; podržani PER-CS 3.0, PER 3.1 ostaje pregledani cilj |
| Composer | Strict manifest/lock validacija prošla; audit pri update/install nije prijavio poznate advisories |
| Gettext | Srpski katalog msgfmt --check uspešno kompajliran |
| Desktop Rust workspace | 51 test prošao: application 2, domain 5, SQLite 43, native HTTPS zaštita 1; doc-tests prošli |
| Desktop native Linux | cargo check prošao uz DBus/GTK/WebKit biblioteke u Fedora 44 rootless kontejneru |
| Desktop TypeScript/Svelte | 0 grešaka / 0 upozorenja |
| Desktop Vite | Produkcioni frontend build prošao |
| Desktop Rustfmt | Primena i provera celog workspace-a |
| Desktop Clippy | Workspace/all-targets uspešan izlaz; postojeća upozorenja tournament/domain/backup koda ostaju. Strogi -D warnings prvo je pao na njima; novi registry moduli nemaju preostala Clippy upozorenja |
| Prenos između projekata | Stvarni sintetički WordPress izvoz uvezen u Desktop, dopunjeno godište, isti nazivi ostali zasebni, članstva sačuvana, restart i SQLite backup restore prošli |

Okruženje: PHP 8.5.11 / PHPUnit 13.4.1 / PHPStan 2.3.0 / PHP CS Fixer 3.95.27 / Composer 2.10.3 / gettext 0.21; WordPress 7.1.3 single-site / MariaDB 10.11.19 InnoDB / GD. Rust/Cargo/Clippy/Rustfmt 1.98.1 u Fedora 44; Vite 6.4.3 sa postojećim zaključanim frontend zavisnostima.

## Komande

Registry root:

```sh
tools/dev/run php vendor/bin/phpunit
tools/dev/run php vendor/bin/phpunit -c phpunit.integration.xml
tools/dev/run php vendor/bin/phpstan analyse --debug --memory-limit=512M
tools/dev/run php vendor/bin/php-cs-fixer check --sequential
tools/dev/run composer validate --strict --no-check-publish
tools/dev/run msgfmt --check --output-file=languages/librett-player-registry-sr_RS.mo languages/librett-player-registry-sr_RS.po
```

Desktop: `cargo test --locked --workspace`, `cargo check -p librett-desktop --locked`, `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets` u kontejneru sa native bibliotekama; `npm run check` i `npm run build` u apps/desktop. Verzionski praćen `tools/dev/registry-check` ponavlja kontejnerski tok; ovde je ista slika koristila postojeći Cargo cache.

## Obim provere

Identitet/setup, validacija/dozvole privatnog CRUD-a, zastarele revizije, iscrpljenje revizija, rollback audita, ograničena pretraga, arhiviranje/vraćanje i početne/draft migracije su provereni. Katalog pokriva nastavak/checksum dodatnih šema, POST/nonce/anonymous odbijanje, UUID veze, rollback neispravnih članstava, politiku/uzrast, isključenje privatnih podataka, no-op javne revizije, povlačenje i privatne medije/licence. JSON testovi izvršavaju primere šeme i neispravan/dupliran/UTF-8/dubina/string/kolekcija/reference unos; uvoz pokriva mapiranja, potvrde, ponavljanje/lokalne izmene, zastarelost i transakcioni neuspeh.

Desktop pokriva lokalne prioritete/izostavljene vrednosti, ručne profile, ista imena, lokalno brisanje, zastarele preglede/checkpoint-e, URL promenu, regresiju poznatih entiteta/nestanak povlačenja, rollback, stvarne nepromenljive entry-member snimke i cache backup/restart. Native mrežni test proverava odbijanje privatnih/rezervisanih/documentation adresa; javni HTTPS servis i redirect/decompression attack harness nisu izvršeni. Adresna politika pregledana je prema [IANA IPv6 registru](https://www.iana.org/assignments/iana-ipv6-special-registry/).

## Ograničenja i predaja

Interaktivni browser/GUI i macOS/Windows native provere nisu izvršeni. Host je prvobitno ostao bez DBus razvojnih fajlova; kontejnerska kompilacija rešava taj razvojni preduslov. MySQL 8.4, multisite, produkcioni kapacitet i operatorovo odobrenje podataka/prava nisu provereni. Ne tvrdi se postojanje potpisa/replika/recovery-ja, purge/private restore-a, čišćenja orphan fajlova, garancije poverenja/svežine izvora ili produkcionog instalera.

Lokalne izmene oba projekta spremne su za pregled/commit; ovaj zadatak nije pravio commit, push, novu verziju, objavu ni migraciju korisničke baze. Predlog commit poruka: registry `feat: implement single-site player catalogue and public snapshot workflows`; Desktop `feat: add one-way player registry import and refresh`.
