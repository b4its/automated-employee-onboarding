-- Add up migration script here
-- Status enum berdasarkan System Flow BPMN [cite: 21, 31, 35, 39, 62]
CREATE TYPE onboarding_status AS ENUM (
    'INITIATED',
    'PENDING_REVIEW',
    'RESUBMISSION_REQUIRED',
    'IT_PROVISIONING',
    'COMPLETED'
);

CREATE TABLE onboardings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    personal_email VARCHAR(255) NOT NULL UNIQUE,
    department VARCHAR(100) NOT NULL,
    start_date DATE NOT NULL,
    status onboarding_status NOT NULL DEFAULT 'INITIATED',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);