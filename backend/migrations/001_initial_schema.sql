-- Sumbu Peradaban: Initial Schema
-- Dimension 4: Sources (Anchor + Witnesses)

CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE sources (
    source_id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    domain              VARCHAR(100) NOT NULL,
    reference           TEXT NOT NULL,
    interpretation_method VARCHAR(100),
    reliability_score   DECIMAL(3,2) CHECK (reliability_score BETWEEN 0.00 AND 1.00),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TYPE curation_tier AS ENUM ('draft', 'verified', 'reviewed', 'canonical');

CREATE TABLE audit_log (
    id              BIGSERIAL PRIMARY KEY,
    entity_type     VARCHAR(50) NOT NULL,
    entity_id       UUID NOT NULL,
    action          VARCHAR(20) NOT NULL,
    old_data        JSONB,
    new_data        JSONB,
    performed_by    VARCHAR(100) NOT NULL,
    performed_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_entity ON audit_log(entity_type, entity_id);
CREATE INDEX idx_audit_time ON audit_log(performed_at DESC);

CREATE TABLE discussions (
    id              BIGSERIAL PRIMARY KEY,
    entity_type     VARCHAR(50) NOT NULL,
    entity_id       UUID NOT NULL,
    author          VARCHAR(100) NOT NULL,
    content         TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_discussions_entity ON discussions(entity_type, entity_id);
