// Copyright (C) 2026 Aleksa Dimitrijević. AGPL-3.0-or-later.
const sr = {
  title:'Baza igrača iz registra', intro:'Registar → lokalna baza. Lokalne izmene, kontakti i turniri se ne šalju. Istorijske prijave ostaju sačuvane.',
  source:'HTTPS adresa snapshot-a', fetch:'Preuzmi i pregledaj', file:'Uvezi JSON fajl', sources:'Sačuvani izvori', refresh:'Osveži', pending:'Sačuvani pregledi', resume:'Otvori pregled', cancel:'Otkaži pregled',
  unsigned:'Nepotpisan izvor: identitet i brojevi revizija su deklaracije izvora, bez potvrde autentičnosti.', policy:'Uslovi izvora', preview:'Pregled pre potvrde', checkpoint:'Kontrolna tačka',
  remote:'Javni profil', local:'Lokalni profil', mapping:'Poveži sa lokalnim igračem', defaultMapping:'Sačuvano mapiranje ili novi igrač',
  name:'Lokalno ime', club:'Lokalni klub / sažetak članstava', year:'Godina rođenja', country:'Država',
  useRegistry:'Prihvati vrednost iz registra i ukloni lokalni izbor', keepLocal:'Lokalni izbor ostaje sačuvan', skip:'Preskoči ovaj profil',
  invalid:'Dopuni ili ispravi lokalno ime/klub i godinu rođenja, ili preskoči profil.', photo:'Preuzmi i kropuj javnu fotografiju',
  refine:'Ažuriraj pregled sa izborima', confirm:'Potvrdi lokalni uvoz', reviewed:'Pregledao/la sam podatke, mapiranja i uslove izvora.',
  next:'Sledeća strana', previous:'Prethodna strana', done:'Uvoz je sačuvan lokalno.', noSource:'Nema sačuvanih izvora.',
  withdrawals:'Povučeni profili u izvoru ostaju lokalno.', changes:'Lokalni izbori u sukobu sa izvorom', fields:'Javno ponuđene vrednosti',
  saved:'Sačuvano lokalno; naredne izmene ostaju na ovom računaru.', omitted:'Izostavljena javna polja ne brišu lokalne vrednosti. Fotografije se preuzimaju samo na zahtev. Neispravna/nedostajuća godina zahteva dopunu. Pregled važi jedan sat.',
  noPhoto:'Nema javne fotografije', photoReady:'Fotografija je pripremljena za lokalni uvoz.', export:'Sačuvaj preuzeti JSON',
  upstream:'Povezan sa registrom', withdrawn:'Povučen u izvoru; lokalni profil je sačuvan',
};
type RegistryMessages = { [K in keyof typeof sr]: string };
const en:RegistryMessages = {
  title:'Players from a registry', intro:'Registry → local database. Local edits, contacts and tournaments are never uploaded. Historical registrations are preserved.',
  source:'HTTPS snapshot URL', fetch:'Download and preview', file:'Import JSON file', sources:'Saved sources', refresh:'Refresh', pending:'Saved previews', resume:'Open preview', cancel:'Cancel preview',
  unsigned:'Unsigned source: identity and revision counters are source declarations without authentication.', policy:'Source terms', preview:'Review before confirmation', checkpoint:'Checkpoint',
  remote:'Public profile', local:'Local profile', mapping:'Link to a local player', defaultMapping:'Saved mapping or new player',
  name:'Local name', club:'Local club / membership summary', year:'Birth year', country:'Country',
  useRegistry:'Accept registry value and clear the local override', keepLocal:'Local override is retained', skip:'Skip this profile',
  invalid:'Complete or correct the local name/club and birth year, or skip this profile.', photo:'Download and crop public photo',
  refine:'Update preview with choices', confirm:'Confirm local import', reviewed:'I reviewed the data, mappings and source terms.',
  next:'Next page', previous:'Previous page', done:'Import saved locally.', noSource:'No saved sources.',
  withdrawals:'Upstream withdrawal keeps local profiles.', changes:'Local overrides that conflict with the source', fields:'Publicly supplied values',
  saved:'Stored locally; subsequent edits stay on this computer.', omitted:'Omitted public fields never clear local values. Photos download only on request. Invalid/missing birth years require completion. A preview lasts one hour.',
  noPhoto:'No public photo', photoReady:'Photo prepared for local import.', export:'Save downloaded JSON',
  upstream:'Linked to a registry', withdrawn:'Withdrawn upstream; local profile retained',
};
export const registryMessages = {sr,en};
