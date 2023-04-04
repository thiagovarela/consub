CREATE SCHEMA analytics;

CREATE TABLE analytics.page_views_raw (
    id BIGSERIAL PRIMARY KEY,
    ts TIMESTAMP WITH TIME ZONE NOT NULL,
    account_id UUID NOT NULL,
    path TEXT NOT NULL,
    headers JSONB NOT NULL
);