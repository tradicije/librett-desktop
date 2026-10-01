const sr = {
  theme: 'Tema', themeLight: 'Svetla', themeDark: 'Tamna', themeSystem: 'Sistemska',
  players: 'Igrači i prijave', directory: 'Lokalna baza igrača', playerName: 'Ime i prezime', club: 'Klub (opciono)',
  addPlayer: 'Dodaj igrača', searchPlayers: 'Pretraži ime ili klub', noPlayers: 'Nema igrača za prikaz.',
  register: 'Prijavi učesnika', chooseCategory: 'Kategorija', firstPlayer: 'Igrač', secondPlayer: 'Partner za dubl', choosePlayer: 'Izaberi igrača',
  entries: 'Prijavljeni učesnici', noEntries: 'U ovoj kategoriji još nema prijava.', playerSaved: 'Igrač je sačuvan.', entrySaved: 'Prijava je sačuvana.',
  invalid_members: 'Singl zahteva jednog igrača, a dubl dva različita igrača.', already_registered: 'Jedan od izabranih igrača je već prijavljen u ovoj kategoriji.',
  tournaments: 'Turniri', leagues: 'Lige', later: 'U pripremi', subtitle: 'Takmičenje počinje ovde.',
  intro: 'Pripremi turnir, odredi kategorije i vodi sve sa svog računara.',
  newTournament: 'Novi turnir', tournamentName: 'Naziv turnira', create: 'Napravi turnir',
  empty: 'Tvoj prvi turnir', emptyText: 'Dodaj turnir, pa kategorije za singl, dubl ili starosne grupe.',
  categories: 'Kategorije', noCategories: 'Ovaj turnir još nema kategorije.',
  addCategory: 'Dodaj kategoriju', categoryName: 'Naziv kategorije', discipline: 'Disciplina',
  format: 'Format', singles: 'Singl', doubles: 'Dubl', knockout: 'Direktni nokaut',
  groups_knockout: 'Grupe (svako sa svakim) → nokaut', save: 'Sačuvaj kategoriju',
  saving: 'Čuvanje…', loading: 'Učitavanje…', local: 'Lokalni rad', retry: 'Pokušaj ponovo',
  preview: 'Pregled interfejsa u pregledaču. Za čuvanje turnira pokreni desktop aplikaciju.',
  error: 'Radnja nije uspela. Pokušaj ponovo.', name_required: 'Unesi naziv.',
  name_too_long: 'Naziv može imati najviše 120 znakova.', duplicate_category: 'Kategorija sa tim nazivom već postoji.',
  not_found: 'Turnir nije pronađen. Osveži pregled.', storage: 'Podaci nisu sačuvani ili učitani. Pokušaj ponovo.',
  count: 'turnira', selected: 'Izabrani turnir', back: 'Svi turniri', created: 'Turnir je sačuvan.',
  categorySaved: 'Kategorija je sačuvana.', language: 'Jezik',
};
type Messages = typeof sr;
const en: Messages = {
  theme: 'Theme', themeLight: 'Light', themeDark: 'Dark', themeSystem: 'System',
  players: 'Players and registrations', directory: 'Local player directory', playerName: 'Full name', club: 'Club (optional)',
  addPlayer: 'Add player', searchPlayers: 'Search name or club', noPlayers: 'No players to show.',
  register: 'Register entry', chooseCategory: 'Category', firstPlayer: 'Player', secondPlayer: 'Doubles partner', choosePlayer: 'Choose a player',
  entries: 'Registered entries', noEntries: 'This category has no registrations yet.', playerSaved: 'Player saved.', entrySaved: 'Registration saved.',
  invalid_members: 'Singles requires one player; doubles requires two distinct players.', already_registered: 'One of these players is already registered in this category.',
  tournaments: 'Tournaments', leagues: 'Leagues', later: 'Coming later', subtitle: 'Competition starts here.',
  intro: 'Prepare a tournament, define its categories, and manage everything locally.',
  newTournament: 'New tournament', tournamentName: 'Tournament name', create: 'Create tournament',
  empty: 'Your first tournament', emptyText: 'Add a tournament, then categories for singles, doubles, or age groups.',
  categories: 'Categories', noCategories: 'This tournament has no categories yet.',
  addCategory: 'Add category', categoryName: 'Category name', discipline: 'Discipline',
  format: 'Format', singles: 'Singles', doubles: 'Doubles', knockout: 'Direct knockout',
  groups_knockout: 'Round-robin groups → knockout', save: 'Save category',
  saving: 'Saving…', loading: 'Loading…', local: 'Local operation', retry: 'Try again',
  preview: 'Browser interface preview. Start the desktop application to save tournaments.',
  error: 'The action failed. Please try again.', name_required: 'Enter a name.',
  name_too_long: 'Names can contain at most 120 characters.', duplicate_category: 'A category with this name already exists.',
  not_found: 'Tournament not found. Refresh the overview.', storage: 'Data could not be saved or loaded. Please try again.',
  count: 'tournaments', selected: 'Selected tournament', back: 'All tournaments', created: 'Tournament saved.',
  categorySaved: 'Category saved.', language: 'Language',
};
export type Language = 'sr' | 'en';
export const messages = { sr, en };
export type MessageKey = keyof Messages;
export function errorKey(error: unknown): MessageKey {
  return typeof error === 'string' && ['name_required', 'name_too_long', 'duplicate_category', 'not_found', 'storage', 'invalid_members', 'already_registered'].includes(error)
    ? error as MessageKey : 'error';
}
export function savedLanguage(): Language {
  try { return localStorage.getItem('librett.language') === 'en' ? 'en' : 'sr'; } catch { return 'sr'; }
}
