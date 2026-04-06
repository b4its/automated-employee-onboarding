-- Add up migration script here
-- Menyimpan balikan data dari IT System 
CREATE TABLE it_provisionings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    onboarding_id UUID NOT NULL UNIQUE REFERENCES onboardings(id) ON DELETE CASCADE,
    corporate_email VARCHAR(255) NOT NULL, 
    ad_account_id VARCHAR(100) NOT NULL, 
    hardware_ticket_id VARCHAR(100), 
    provisioning_status VARCHAR(50) NOT NULL, 
    completed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);