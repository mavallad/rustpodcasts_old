CREATE TABLE IF NOT EXISTS episodes (
    id CHARACTER(36) PRIMARY KEY NOT NULL,
    id_serie CHARACTER(36),
    podcast_title VARCHAR(80) NOT NULL,
    title VARCHAR(80) NOT NULL,
    description TEXT,
    interviewed TEXT,
    publication_date DATE,
    image_url TEXT,
    link TEXT NOT NULL,
    duration_seconds INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT fk_series
        FOREIGN KEY (id_serie)
        REFERENCES series(id)
);

CREATE TRIGGER IF NOT EXISTS EpisodesUpdateTimestamps AFTER UPDATE ON episodes
  FOR EACH ROW WHEN NEW.updated_at <= OLD.updated_at 
BEGIN 
  update episodes set updated_at=CURRENT_TIMESTAMP where id=OLD.id;  
END;
