-- Your SQL goes here

-- CREATE TABLE "user" (
--     id SERIAL PRIMARY KEY,

--     identifier VARCHAR(256) NOT NULL UNIQUE,
--     password VARCHAR(256) NOT NULL,
--     salt VARCHAR(64) NOT NULL,

--     crud JSONB NOT NULL,

--     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
--     updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
-- );

CREATE TABLE "instance" (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR(64) NOT NULL UNIQUE,

    label TEXT,
    
    itype INT NOT NULL, -- instance type
    image INT NOT NULL, -- instance image

    mac VARCHAR(20) NOT NULL UNIQUE,
    ipv4 VARCHAR(20) NOT NULL UNIQUE,

    created_by VARCHAR(64) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()


    -- FOREIGN KEY (created_by) REFERENCES "user" (id)
);

-- CREATE TABLE "disk" (
--     id SERIAL PRIMARY KEY,
--     uuid VARCHAR(64) NOT NULL UNIQUE,

--     capacity INT,  -- {n}GiB
    
--     created_by INT NOT NULL,
--     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
--     updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    
--     FOREIGN KEY (created_by) REFERENCES "user" (id)
-- );