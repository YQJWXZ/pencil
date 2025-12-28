-- Create categories table
CREATE TABLE IF NOT EXISTS categories (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create index on slug for faster lookups
CREATE INDEX IF NOT EXISTS idx_categories_slug ON categories(slug);

-- Insert default categories
INSERT INTO categories (name, slug, description)
VALUES
    ('Technology', 'technology', 'Articles about technology and programming'),
    ('Lifestyle', 'lifestyle', 'Articles about daily life and experiences'),
    ('Tutorial', 'tutorial', 'Step-by-step guides and tutorials')
ON CONFLICT (slug) DO NOTHING;
