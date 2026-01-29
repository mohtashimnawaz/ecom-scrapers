-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create price_alerts table
CREATE TABLE IF NOT EXISTS price_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    url TEXT NOT NULL,
    current_price DECIMAL(10,2),
    target_price DECIMAL(10,2) NOT NULL,
    platform VARCHAR(50),
    product_name TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    last_checked TIMESTAMPTZ,
    is_active BOOLEAN DEFAULT true
);

-- Create price_history table
CREATE TABLE IF NOT EXISTS price_history (
    id SERIAL PRIMARY KEY,
    alert_id UUID REFERENCES price_alerts(id) ON DELETE CASCADE,
    price DECIMAL(10,2) NOT NULL,
    checked_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_price_alerts_user_id ON price_alerts(user_id);
CREATE INDEX IF NOT EXISTS idx_price_alerts_active ON price_alerts(is_active);
CREATE INDEX IF NOT EXISTS idx_price_history_alert_id ON price_history(alert_id);
CREATE INDEX IF NOT EXISTS idx_price_history_checked_at ON price_history(checked_at);
