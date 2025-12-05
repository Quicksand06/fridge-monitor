CREATE TABLE items (
                       id SERIAL PRIMARY KEY,
                       name TEXT NOT NULL,
                       barcode TEXT NOT NULL UNIQUE
);