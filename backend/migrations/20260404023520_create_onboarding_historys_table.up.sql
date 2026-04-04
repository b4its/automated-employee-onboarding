-- Add up migration script here
-- Menyimpan riwayat perubahan status 
CREATE TABLE onboarding_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    onboarding_id UUID NOT NULL REFERENCES onboardings(id) ON DELETE CASCADE,
    previous_status onboarding_status,
    new_status onboarding_status NOT NULL,
    changed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    notes TEXT -- Bisa digunakan untuk menyimpan catatan review HR "notes": "ID matches profile." 
);