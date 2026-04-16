-- Enable UUID support
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Recreate jobs table with UUID
CREATE TABLE IF NOT EXISTS jobs  (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  status TEXT NOT NULL,
  page_completed INT NOT NULL,
  total_pages INT NOT NULL,
  enqueue_left INT NOT NULL,
  file_url TEXT NULL
);

-- Create results table
CREATE TABLE  IF NOT EXISTS results (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  job_id UUID NOT NULL,
  data VARCHAR,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

  CONSTRAINT fk_job
    FOREIGN KEY (job_id)
    REFERENCES jobs(id)
    ON DELETE CASCADE
);
