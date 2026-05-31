-- Sumbu Peradaban: User Management Schema
-- Handles user authentication and roles

CREATE TYPE user_role AS ENUM ('visitor', 'editor', 'admin');

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role user_role NOT NULL DEFAULT 'visitor',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed an initial super admin user (password is 'sumbu_admin', to be hashed via application later if needed, but we will seed via GraphQL register mutation or leave it empty here and create one manually)
-- Actually, let's let the application handle creating the first admin or provide a script.
