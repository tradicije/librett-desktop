# LibreTT Desktop integracija — razvoj

[English](../en/REGISTRY_IMPORT.md)

Implementirana u razvojnom kodu susednog [LibreTT Desktop](https://github.com/tradicije/librett-desktop) projekta; integration release nije objavljen. Pogledaj [stvarne provere](REGISTRY_VERIFICATION_2026_10_08.md).

Registar → Desktop je jednosmerno. HTTPS GET/uvoz fajla ne šalje lokalne profile, kontakte, beleške, dolaske, uplate i turnire. WordPress autoritet odvojen je od lokalnih Desktop identiteta i izmena.

Desktop Igrači imaju HTTPS preuzimanje, ograničen JSON uvoz, privatni pregled, izričito mapiranje postojećih igrača, dopunu, preskakanje, pregled sukoba i potvrdu. Ista imena ne spajaju automatski. Lokalno godište je obavezno, naziv/klub najviše 120 znakova; opciono godište izvora dopuni kada nedostaje. UUID članstva se čuvaju zasebno i sažimaju u postojeće lokalno polje kluba.

SQLite šema 22 čuva izvor, mapiranja, osnovne vrednosti, trajne lokalne izbore, članstva i potvrde. Postojeće baze dobijaju pre-v22 backup. Novi lokalni UUID nezavisan je od izvora. Jedan lokalni igrač ima jedno mapiranje u ovoj etapi. Lokalne izmene ostaju i ako se kasnije vrednosti poklope; izričit izbor registra uklanja taj lokalni prioritet. Izostavljena opciona polja ne brišu lokalne podatke. Lokalno brisanje ostavlja odvojenu vezu koja se podrazumevano preskače.

Osvežavanje odbija starije checkpoint-e, različite podatke istog checkpoint-a i regresiju/nestanak poznatih revizija/povlačenja. Pregled zastareva posle relevantnih lokalnih/izvornih izmena i traje sat vremena. Potvrda je transakciona i idempotentna. Povlačenje izvora menja evidenciju porekla i čuva lokalne igrače/istorijske prijave. Ovo ne dokazuje svežinu/autentičnost prvog kontakta: nepotpisani metapodaci su nepouzdani.

Fotografija se preuzima samo na izričit zahtev uz proveru hash-a, veličine, MIME-a i dimenzija, pa postojeći crop dijalog stvara lokalni JPEG. Postojeća slika ostaje dok je izričito ne zameniš. Mreža koristi samo HTTPS GET, javne fiksirane DNS adrese, ograničene DNS zahteve/vreme/telo, bez redirect/proxy/compression; privatni/rezervisani hostovi se odbijaju.

JSON: 32 MiB, dubina 8, ukupno 100.000 stavki nizova, 32 ključa po objektu i 32.768 bajtova po string tokenu. Šema/graf proveravaju UUID, reference, revizije i pojedinačne kolekcije. Potpisi, predlozi replike, automatsko otkrivanje/promena izvora i ponovni izvoz keširanog izvora nisu implementirani. Za promenjen domen izričito koristi novi HTTPS URL istog UUID-a registra.

## Upotreba

Otvori Igrači i panel Registra. Unesi javni HTTPS snapshot URL (obično `/wp-json/librett-registry/v1/snapshot`) ili izaberi JSON fajl. Pregledaj uslove baze/medija i napomenu o nepotpisanom izvoru. Pregled pokazuje 25 zapisa po strani; koristi nezavisne lokalne identitete ili izričito izaberi postojećeg igrača. Dopuni obavezna lokalna polja ili preskoči zapis. Izaberi vrednosti registra samo za polja kojima želiš da ukloniš lokalni prioritet. Fotografiju zatraži/iseci samo ako je želiš.

Primeni izbore pregleda pre potvrde; izmena izbora blokira potvrdu do novog pregleda. Označi pregled i uvezi. Pripremljeni pregledi se nastavljaju/otkazuju; najviše tri, važe sat vremena. Osveži sačuvan izvor ili izričito unesi njegov novi URL; identitet je UUID, ne hostname. Povlačenje čuva lokalne igrače i istoriju. Postojeći imenik i prijave i dalje koriste lokalne profile.
