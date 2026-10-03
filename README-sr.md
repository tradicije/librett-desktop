<p align="left">
  <img src="assets/img/logo-dark.png" alt="LibreTT" width="256" style="max-width: 256px; width: 100%; height: auto;" />
</p>

[English](README.md)

LibreTT je besplatna aplikacija otvorenog koda za organizaciju stonoteniskih
turnira i liga, koja se razvija sa lokalnim radom kao osnovom. Namenjena je
klubovima, organizatorima, igračima i zajednici koja gradi ovaj sport.
LibreTT je naziv aplikacije i krovni identitet projekta.

**Status: rani razvoj.** Trenutno razvijamo desktop aplikaciju za turnire.
Nacrti žreba su dostupni. Mečevi, rezultati i instaleri za izdanja još nisu dostupni. Uputstvo za
pokretanje nalazi se u [razvojnoj dokumentaciji](docs/sr/DEVELOPMENT.md).

## Filozofija projekta

Stoni tenis nastaje radom ljudi: igrača koji treniraju, klubova koji ih okupljaju,
sudija, volontera i organizatora koji održavaju takmičenja. Osnovni alat za
organizaciju i beleženje tog rada treba da bude dostupan svima, bez obzira na
budžet kluba ili veličinu turnira. **LibreTT polazi od stava da takav alat treba
da bude besplatan i otvorenog koda.**

Jednostavan alat može da donese korist mnogo veću od svog tehničkog obima.
Pregledni rezultati i rasporedi mogu da približe takmičenje publici. Statistika
sačuvana kroz decenije može da pokaže razvoj igrača, rad klubova i istoriju
čitavih takmičenja. Dostupni, pouzdani podaci mogu da olakšaju praćenje sporta,
povećaju njegovu vidljivost i pomognu klubovima da predstave svoj rad budućim
sponzorima. To su mogućnosti koje želimo da otvorimo zajednici, a ne obećanje
koje zavisi samo od jednog programa.

Postoje drugi alati za organizaciju takmičenja i neki mogu veoma dobro da
obavljaju taj posao. LibreTT ne zasniva svoju svrhu na tvrdnji da je prvi ili
jedini. Želimo da ponudimo izbor u kojem su besplatan pristup, otvoren kod,
lokalni rad i trajna dostupnost podataka deo istog dogovora sa zajednicom.
Korišćenje aplikacije i pristup sopstvenoj istoriji ne treba uslovljavati
pretplatom, aktivacijom licence ili kupovinom paketa funkcija.

**Podaci takmičenja pripadaju zajednici koja ih stvara.** Rezultat nastaje na
stolu, radom igrača i klubova; aplikacija ga beleži. Zato istorija takmičenja
ne sme nestati kada se ugasi servis, prestane održavanje programa ili više
nema novca za pretplatu. Ono što zajednica stvara danas mora moći da sačuva,
pročita i prenese i za nekoliko decenija.

Iz tog stava proizlaze konkretni zahtevi: rad bez obaveznog interneta ili naloga,
lokalno čuvanje podataka, dokumentovani otvoreni formati i arhive čitljive bez
pokrenutog LibreTT-a i njegovog autora. Rezervna kopija sama nije dovoljna ako
za njeno čitanje mora da postoji isti servis. Otvoren kod omogućava zajednici
da proveri kako alat radi, prilagodi ga i nastavi razvoj kada prvobitni
održavaoci više ne mogu. Otvorenost koda ne znači javno objavljivanje ličnih
podataka igrača; zajednica koja ih čuva odlučuje o njihovom deljenju.

Ovo su obaveze koje vode razvoj. Lokalna baza već postoji; nezavisni arhivski
izvoz i interfejs za vraćanje kopije još su planirani. Očuvanje istorije mora
biti deo funkcionalnosti za rezultate, a ne naknadni dodatak.

## Šta trenutno radi

- Nezavisni radni tabovi sa stalnim Home ekranom, sačuvanim nacrtima i izborom,
  zasebnom istorijom i kontrolama mišem/tastaturom.
- Kreiranje više turnira i singl/dubl kategorija sa zasebnim izborom formata.
- Poseban editor kategorija sa pravilima, učesnicima i automatskim/ručnim rasporedom nosilaca; pregled grupa, round-robin parova i nokaut kostura sa skrolom.
- Zajednička lokalna baza igrača sa profilima, fotografijama i pretragom.
- Detalji kategorije sa čekiranjem više igrača za singl i sastavljanjem dubl
  parova; singl i dubl mogu imati isti naziv kategorije.
- Prijave po kategorijama uz istorijski snimak imena i kluba, povlačenje i
  vraćanje prijave, kao i potvrda dolaska igrača za ceo turnir.
- Kotizacija kategorije u RSD i automatsko zaduženje pri novoj prijavi;
  kod dubla iznos važi po paru, a 0 znači besplatno učešće.
- Blagajna sa pretragom, redom po igraču, kolonama kategorija, izborom stavki i dugmadima Naplati i Povraćaj.
  Dubl kotizacija deli se ravnopravno; dugovanje, neto primljeno i broj prijavljenih
  uzimaju u obzir postojeće uplate i popuste. Finansijska istorija ostaje sačuvana.
- Brisanje praznih kategorija i arhiviranje kategorija sa prijavama, uz
  očuvanje prijava i finansijske istorije.
- Srpski i engleski interfejs, svetla/tamna/sistemska tema, lokalno dostupni
  Libre Franklin font i Tabler ikonice.

Lige i telefonske aplikacije dolaze kasnije. Posebno podešavanje tarifa po osobi
u dublu i obrasci za ručne finansijske korekcije ostaju za kasnije. Lokalna administracija
trenutno nema naloge ni kontrolu korisničkih uloga. Za prijavu ranjivosti i
deljenje lokalnih baza pogledaj [bezbednosnu politiku](SECURITY-sr.md).

## Planirani obim

Aplikacija ima dva modula: **Turniri** i **Lige**. Razvoj počinje turnirima na
**Linuxu, macOS-u i Windowsu**. Telefonske companion aplikacije za sudije i
gledaoce dolaze kasnije.

- Više turnira, sa više kategorija unutar svakog: singl, dubl, veterani,
  starosne grupe i kategorije koje definiše organizator.
- Direktni jednostruki nokaut ili round-robin grupe pa jednostruki nokaut,
  nezavisno za svaku kategoriju.
- Lokalna baza igrača i prijavljivanje igrača u više kategorija.
- Evidencija najava, dolazaka, kotizacija, uplata i preostalog dugovanja.
- Automatski žreb sa nosiocima, ručno raspoređivanje ili izmena automatskog
  predloga pre potvrde.
- Rezultati, dodela stolova, rangiranje, prolaz i konačni plasman.
- Rezervne kopije, oporavak, uvoz/izvoz, materijal za štampu i prikaz na
  drugom ekranu.
- Srpski i engleski kroz korisnički interfejs i izlazne dokumente.

Buduća integracija sa bazom igrača na **stoni.rs** omogućiće pretragu i
preuzimanje podataka za kasniji offline rad. Baza i njen API još ne postoje.
Lokalni rad neće zavisiti od tog servisa.

## Arhitektura

Aplikaciju gradimo uz **Tauri 2, Rust, TypeScript/Svelte i SQLite**.
Izbor proveravamo ranim prototipom na sva tri sistema, uključujući offline
instalaciju, štampu, vraćanje rezervne kopije i drugi ekran.

Pravila takmičenja pripadaju jezgru nezavisnom od platforme. Desktop i budući
companion adapteri koriste iste slučajeve korišćenja. Potvrđeni rezultati su
izvor istine; tabele mogu ponovo da se izračunaju.

Postojeći LibreTT WordPress projekat i DimiPress Rally služe kao reference.
Ovaj repozitorijum opisuje novu samostalnu aplikaciju. Autor planira da
postojeći WordPress projekat preimenuje u `librett-wordpress`.

## Dokumentacija i doprinosi

- [Plan desktop aplikacije](docs/DESKTOP_TOURNAMENT_PLAN_SR.md)
- [Razvojno uputstvo](docs/sr/DEVELOPMENT.md)
- [Doprinos projektu](CONTRIBUTING-sr.md)
- [Licence fonta i ikonica](THIRD_PARTY_NOTICES.md)
- [Changelog — engleski](CHANGELOG.md)

Korisnička i tehnička dokumentacija na srpskom i engleskom razvijaće se uz
specifikaciju i implementaciju. Detaljan radni plan trenutno je na srpskom.

## Razvoj uz pomoć AI alata

LibreTT se razvija uz pomoć AI alata pri programiranju. Ideju projekta,
arhitekturu, odluke o implementaciji i testove osmislio je ili pregledao
i verifikovao Aleksa Dimitrijević. Ljudski pregled i provera sastavni su
deo razvoja; pomoć AI alata ne zamenjuje odgovornost za kod.

## Autor i licenca

Copyright (C) 2026 Aleksa Dimitrijević.

LibreTT je licenciran pod **GNU Affero General Public License, verzija 3
ili bilo koja kasnija verzija** (`AGPL-3.0-or-later`). Pogledaj [LICENSE](LICENSE).

LibreTT se pruža bez garancije; puni uslovi nalaze se u licenci.
