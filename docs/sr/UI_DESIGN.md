# Smernice za desktop interfejs

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

Radni prostor turnira koristi stalne kartice Pregled, Kategorije, Prijave i
Blagajna. Kartice Žreb, Mečevi i Rezultati nalaze se unutar kategorije.
Delovi koji još nisu implementirani jasno prikazuju da su u pripremi. Brojači
prikazuju stvarne podatke, a probni podaci za proveru interfejsa ostaju van
repozitorijuma.


## Sistemska istorija

Istorija ekrana i kartica koristi History API webview-a. Strelice i bočna dugmad
miša dele istoriju; na macOS-u je uključena i navigacija trackpadom. Tokom upisa
sistemska navigacija vraća trenutnu poziciju istorije pre promene ekrana.
