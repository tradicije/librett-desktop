CREATE TABLE group_orders (
    draw_id TEXT NOT NULL REFERENCES category_draws(id),
    group_number INTEGER NOT NULL,
    revision INTEGER NOT NULL,
    payload TEXT NOT NULL,
    PRIMARY KEY(draw_id, group_number)
);
PRAGMA user_version = 15;
