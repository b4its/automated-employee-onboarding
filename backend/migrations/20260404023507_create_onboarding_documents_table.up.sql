-- Add up migration script here
-- Menyimpan metadata dokumen yang diunggah kandidat 
CREATE TABLE onboarding_documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    onboarding_id UUID NOT NULL REFERENCES onboardings(id) ON DELETE CASCADE,
    document_type VARCHAR(50) NOT NULL, -- Contoh: "IDENTIFICATION"
    file_path TEXT NOT NULL, 
    uploaded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);