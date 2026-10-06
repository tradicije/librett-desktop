-- Moving a tournament to trash preserves every sporting and financial record.
CREATE TABLE tournament_trash (
 tournament_id TEXT PRIMARY KEY REFERENCES tournaments(id),
 deleted_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);
CREATE TRIGGER tournaments_trash_insert BEFORE INSERT ON tournaments
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=NEW.id)
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER tournaments_trash_update BEFORE UPDATE ON tournaments
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=OLD.id) OR EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=NEW.id)
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER tournaments_trash_delete BEFORE DELETE ON tournaments
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=OLD.id)
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER categories_trash_insert BEFORE INSERT ON categories
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=NEW.tournament_id)
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER categories_trash_update BEFORE UPDATE ON categories
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=OLD.tournament_id) OR EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=NEW.tournament_id)
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER categories_trash_delete BEFORE DELETE ON categories
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=OLD.tournament_id)
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER entries_trash_insert BEFORE INSERT ON entries
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT tournament_id FROM categories WHERE id=NEW.category_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER entries_trash_update BEFORE UPDATE ON entries
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT tournament_id FROM categories WHERE id=OLD.category_id)) OR EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT tournament_id FROM categories WHERE id=NEW.category_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER entries_trash_delete BEFORE DELETE ON entries
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT tournament_id FROM categories WHERE id=OLD.category_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER entry_members_trash_insert BEFORE INSERT ON entry_members
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT tournament_id FROM categories WHERE id=NEW.category_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER entry_members_trash_update BEFORE UPDATE ON entry_members
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT tournament_id FROM categories WHERE id=OLD.category_id)) OR EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT tournament_id FROM categories WHERE id=NEW.category_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER entry_members_trash_delete BEFORE DELETE ON entry_members
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT tournament_id FROM categories WHERE id=OLD.category_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER player_attendance_trash_insert BEFORE INSERT ON player_attendance
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=NEW.tournament_id)
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER player_attendance_trash_update BEFORE UPDATE ON player_attendance
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=OLD.tournament_id) OR EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=NEW.tournament_id)
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER player_attendance_trash_delete BEFORE DELETE ON player_attendance
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=OLD.tournament_id)
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER cash_records_trash_insert BEFORE INSERT ON cash_records
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT c.tournament_id FROM entries e JOIN categories c ON c.id=e.category_id WHERE e.id=NEW.entry_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER cash_records_trash_update BEFORE UPDATE ON cash_records
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT c.tournament_id FROM entries e JOIN categories c ON c.id=e.category_id WHERE e.id=OLD.entry_id)) OR EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT c.tournament_id FROM entries e JOIN categories c ON c.id=e.category_id WHERE e.id=NEW.entry_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER cash_records_trash_delete BEFORE DELETE ON cash_records
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT c.tournament_id FROM entries e JOIN categories c ON c.id=e.category_id WHERE e.id=OLD.entry_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER cash_allocations_trash_insert BEFORE INSERT ON cash_allocations
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT c.tournament_id FROM cash_records cr JOIN entries e ON e.id=cr.entry_id JOIN categories c ON c.id=e.category_id WHERE cr.id=NEW.record_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER cash_allocations_trash_update BEFORE UPDATE ON cash_allocations
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT c.tournament_id FROM cash_records cr JOIN entries e ON e.id=cr.entry_id JOIN categories c ON c.id=e.category_id WHERE cr.id=OLD.record_id)) OR EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT c.tournament_id FROM cash_records cr JOIN entries e ON e.id=cr.entry_id JOIN categories c ON c.id=e.category_id WHERE cr.id=NEW.record_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER cash_allocations_trash_delete BEFORE DELETE ON cash_allocations
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT c.tournament_id FROM cash_records cr JOIN entries e ON e.id=cr.entry_id JOIN categories c ON c.id=e.category_id WHERE cr.id=OLD.record_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER cash_settlements_trash_insert BEFORE INSERT ON cash_settlements
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=NEW.tournament_id)
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER cash_settlements_trash_update BEFORE UPDATE ON cash_settlements
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=OLD.tournament_id) OR EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=NEW.tournament_id)
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER cash_settlements_trash_delete BEFORE DELETE ON cash_settlements
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=OLD.tournament_id)
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER category_draws_trash_insert BEFORE INSERT ON category_draws
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT tournament_id FROM categories WHERE id=NEW.category_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER category_draws_trash_update BEFORE UPDATE ON category_draws
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT tournament_id FROM categories WHERE id=OLD.category_id)) OR EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT tournament_id FROM categories WHERE id=NEW.category_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER category_draws_trash_delete BEFORE DELETE ON category_draws
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT tournament_id FROM categories WHERE id=OLD.category_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER category_configurations_trash_insert BEFORE INSERT ON category_configurations
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT tournament_id FROM categories WHERE id=NEW.category_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER category_configurations_trash_update BEFORE UPDATE ON category_configurations
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT tournament_id FROM categories WHERE id=OLD.category_id)) OR EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT tournament_id FROM categories WHERE id=NEW.category_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER category_configurations_trash_delete BEFORE DELETE ON category_configurations
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT tournament_id FROM categories WHERE id=OLD.category_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER tournament_scheduling_trash_insert BEFORE INSERT ON tournament_scheduling
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=NEW.tournament_id)
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER tournament_scheduling_trash_update BEFORE UPDATE ON tournament_scheduling
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=OLD.tournament_id) OR EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=NEW.tournament_id)
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER tournament_scheduling_trash_delete BEFORE DELETE ON tournament_scheduling
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=OLD.tournament_id)
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER match_assignments_trash_insert BEFORE INSERT ON match_assignments
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=NEW.tournament_id)
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER match_assignments_trash_update BEFORE UPDATE ON match_assignments
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=OLD.tournament_id) OR EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=NEW.tournament_id)
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER match_assignments_trash_delete BEFORE DELETE ON match_assignments
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=OLD.tournament_id)
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER completion_history_trash_insert BEFORE INSERT ON completion_history
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=CASE WHEN NEW.entity_kind='tournament' THEN NEW.entity_id ELSE (SELECT tournament_id FROM categories WHERE id=NEW.entity_id) END)
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER completion_history_trash_update BEFORE UPDATE ON completion_history
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=CASE WHEN OLD.entity_kind='tournament' THEN OLD.entity_id ELSE (SELECT tournament_id FROM categories WHERE id=OLD.entity_id) END) OR EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=CASE WHEN NEW.entity_kind='tournament' THEN NEW.entity_id ELSE (SELECT tournament_id FROM categories WHERE id=NEW.entity_id) END)
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER completion_history_trash_delete BEFORE DELETE ON completion_history
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=CASE WHEN OLD.entity_kind='tournament' THEN OLD.entity_id ELSE (SELECT tournament_id FROM categories WHERE id=OLD.entity_id) END)
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER match_results_trash_insert BEFORE INSERT ON match_results
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT c.tournament_id FROM category_draws d JOIN categories c ON c.id=d.category_id WHERE d.id=NEW.draw_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER match_results_trash_update BEFORE UPDATE ON match_results
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT c.tournament_id FROM category_draws d JOIN categories c ON c.id=d.category_id WHERE d.id=OLD.draw_id)) OR EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT c.tournament_id FROM category_draws d JOIN categories c ON c.id=d.category_id WHERE d.id=NEW.draw_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER match_results_trash_delete BEFORE DELETE ON match_results
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT c.tournament_id FROM category_draws d JOIN categories c ON c.id=d.category_id WHERE d.id=OLD.draw_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER group_orders_trash_insert BEFORE INSERT ON group_orders
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT c.tournament_id FROM category_draws d JOIN categories c ON c.id=d.category_id WHERE d.id=NEW.draw_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER group_orders_trash_update BEFORE UPDATE ON group_orders
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT c.tournament_id FROM category_draws d JOIN categories c ON c.id=d.category_id WHERE d.id=OLD.draw_id)) OR EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT c.tournament_id FROM category_draws d JOIN categories c ON c.id=d.category_id WHERE d.id=NEW.draw_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER group_orders_trash_delete BEFORE DELETE ON group_orders
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT c.tournament_id FROM category_draws d JOIN categories c ON c.id=d.category_id WHERE d.id=OLD.draw_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER knockout_fillers_trash_insert BEFORE INSERT ON knockout_fillers
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT c.tournament_id FROM category_draws d JOIN categories c ON c.id=d.category_id WHERE d.id=NEW.draw_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER knockout_fillers_trash_update BEFORE UPDATE ON knockout_fillers
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT c.tournament_id FROM category_draws d JOIN categories c ON c.id=d.category_id WHERE d.id=OLD.draw_id)) OR EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT c.tournament_id FROM category_draws d JOIN categories c ON c.id=d.category_id WHERE d.id=NEW.draw_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
CREATE TRIGGER knockout_fillers_trash_delete BEFORE DELETE ON knockout_fillers
WHEN EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=(SELECT c.tournament_id FROM category_draws d JOIN categories c ON c.id=d.category_id WHERE d.id=OLD.draw_id))
BEGIN SELECT RAISE(ABORT,'tournament_in_trash'); END;
PRAGMA user_version = 19;
