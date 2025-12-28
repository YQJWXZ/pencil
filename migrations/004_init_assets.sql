-- Create assets table
CREATE TABLE IF NOT EXISTS assets (
    id BIGSERIAL PRIMARY KEY,
    filename VARCHAR(500) NOT NULL,
    file_path VARCHAR(1000) NOT NULL,
    mime_type VARCHAR(255) NOT NULL,
    size BIGINT NOT NULL,
    uploader_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_assets_uploader_id ON assets(uploader_id);
CREATE INDEX IF NOT EXISTS idx_assets_mime_type ON assets(mime_type);
CREATE INDEX IF NOT EXISTS idx_assets_created_at ON assets(created_at DESC);
