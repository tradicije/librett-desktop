CREATE TABLE category_configurations (
    category_id TEXT PRIMARY KEY NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    revision INTEGER NOT NULL CHECK(revision > 0),
    payload TEXT NOT NULL
);
-- Preserve the most recent group draft settings when upgrading existing categories.
INSERT INTO category_configurations(category_id, revision, payload)
SELECT c.id, 1, json_object(
    'group_count', COALESCE(json_extract(d.payload, '$.settings.group_count'), 2),
    'qualifiers_per_group', COALESCE(json_extract(d.payload, '$.settings.qualifiers_per_group'), 2),
    'best_of', 5, 'points_to_win', 11, 'win_by', 2,
    'ranking', json('["head_to_head","set_ratio","point_ratio"]'))
FROM categories c LEFT JOIN category_draws d ON d.category_id=c.id
AND d.revision=(SELECT MAX(revision) FROM category_draws WHERE category_id=c.id);
PRAGMA user_version = 11;
