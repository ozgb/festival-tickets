CREATE OR REPLACE FUNCTION timestamp_to_rfc3339_str(input_timestamp timestamptz)
RETURNS text AS $$
BEGIN
  -- Ideally, we'd like to respect the timezone in the original value
  -- rather than convert to UTC
  -- However, using "OF" in the format string doesn't give RFC3339 compliant offsets 
  -- So, we just convert to UTC and add a Z
  RETURN to_char(input_timestamp AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"');
END;
$$ LANGUAGE plpgsql IMMUTABLE;
