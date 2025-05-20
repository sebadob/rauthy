-- Removes the left-over legacy rauthy logo for new deployments.
-- This has been taken care of with version migrations programmatically,
-- but has been forgotten for new deployments.
DELETE
FROM client_logos
WHERE client_id = 'rauthy'
  AND content_type = 'image/svg+xml'
  AND data LIKE '%viewBox="0 0 512 138"%';
