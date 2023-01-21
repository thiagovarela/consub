CREATE SCHEMA leads;

CREATE TABLE leads.leads (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts.accounts (id),
    assigned_to_id UUID NULL REFERENCES accounts.users (id),
    archived_at TIMESTAMP,
    status VARCHAR(255) NOT NULL DEFAULT 'new',
    data JSONB,       
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP    
);
CREATE INDEX leads_id_idx ON leads.leads (id);
CREATE INDEX leads_account_id_idx ON leads.leads (account_id);
CREATE INDEX leads_assigned_to_id_idx ON leads.leads (assigned_to_id);
SELECT setup_tgr_updated_at('leads.leads');

CREATE TABLE leads.lead_notes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES accounts.users (id),
    lead_id UUID NOT NULL REFERENCES leads.leads (id),
    body TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP    
);
CREATE INDEX lead_notes_id_idx ON leads.lead_notes (id);
CREATE INDEX lead_notes_lead_id_idx ON leads.lead_notes (lead_id);
CREATE INDEX lead_notes_user_id_idx ON leads.lead_notes (user_id);
SELECT setup_tgr_updated_at('leads.lead_notes');
