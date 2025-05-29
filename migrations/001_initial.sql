CREATE TABLE IF NOT EXISTS links (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    short_code VARCHAR(10) UNIQUE NOT NULL,
    original_url TEXT NOT NULL,
    clicks BIGINT DEFAULT 0 NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    expires_at TIMESTAMPTZ
);

CREATE INDEX idx_short_code ON links(short_code);
CREATE INDEX idx_expires_at ON links(expires_at) WHERE expires_at IS NOT NULL; 