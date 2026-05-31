-- Migration 003: Expand Source Entity with Rich Attributes and Media Support
ALTER TABLE sources ADD COLUMN title VARCHAR(255);
ALTER TABLE sources ADD COLUMN author VARCHAR(255);
ALTER TABLE sources ADD COLUMN publication_era VARCHAR(100);
ALTER TABLE sources ADD COLUMN reliability_assessment TEXT;
ALTER TABLE sources ADD COLUMN media_links TEXT;
