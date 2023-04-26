-- Add migration script here
CREATE virtual TABLE problems_fts USING fts5(
    title,
    description,
    content='problems',
    content_rowid='id',
);

CREATE TRIGGER problems_ai after INSERT ON problems BEGIN
    INSERT INTO problems_fts (rowid, title, description) VALUES (new.id, new.title, new.description);
END;


INSERT INTO problems_fts (rowid, title, description) SELECT id, title, description FROM problems;
