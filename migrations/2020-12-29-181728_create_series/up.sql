CREATE TABLE IF NOT EXISTS series (
    id CHARACTER(36) PRIMARY KEY NOT NULL,
    title VARCHAR(80) NOT NULL,
    description TEXT,
    image_url TEXT,
    link TEXT NOT NULL,
    status CHARACTER(1) CHECK(status IN ('A', 'C')) NOT NULL DEFAULT 'A',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER IF NOT EXISTS SeriesUpdateTimestamps AFTER UPDATE ON series
  FOR EACH ROW WHEN NEW.updated_at <= OLD.updated_at 
BEGIN 
  update series set updated_at=CURRENT_TIMESTAMP where id=OLD.id;  
END;
