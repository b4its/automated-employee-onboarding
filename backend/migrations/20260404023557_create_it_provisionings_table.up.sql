-- Add up migration script here
-- Menyimpan balikan data dari IT System 
CREATE TABLE it_provisionings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    onboarding_id UUID NOT NULL UNIQUE REFERENCES onboardings(id) ON DELETE CASCADE,
    corporate_email VARCHAR(255) NOT NULL, -- [cite: 106]
    ad_account_id VARCHAR(100) NOT NULL, -- [cite: 107]
    hardware_ticket_id VARCHAR(100), -- [cite: 108]
    provisioning_status VARCHAR(50) NOT NULL, -- [cite: 109]
    completed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);