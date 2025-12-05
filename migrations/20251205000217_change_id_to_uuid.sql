-- Enable pgcrypto extension if not already present
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- 1. Add a temporary UUID column
ALTER TABLE items
    ADD COLUMN id_new UUID;

-- 2. Fill new column with generated UUIDs
UPDATE items
SET id_new = gen_random_uuid();

-- 3. Drop old primary key constraint
ALTER TABLE items
DROP CONSTRAINT items_pkey;

-- 4. Drop the old SERIAL column
ALTER TABLE items
DROP COLUMN id;

-- 5. Rename new column to id
ALTER TABLE items
    RENAME COLUMN id_new TO id;

-- 6. Add primary key constraint back
ALTER TABLE items
    ADD PRIMARY KEY (id);
