\pset format unaligned
\pset tuples_only on
\echo BATTLE378_BY_REACTION
SELECT e->>'reaction', SUM((e->>'damage')::bigint), COUNT(*) FROM battle_records br, jsonb_array_elements(br.reaction_timeline->'events') e WHERE br.id=378 GROUP BY 1 ORDER BY 2 DESC;
\echo META_378_341_339
SELECT id,created_at,reaction_timeline->>'schema_version',jsonb_array_length(COALESCE(reaction_timeline->'events','[]'::jsonb)) FROM battle_records WHERE id IN (378,341,339) ORDER BY id;
\echo EXCLUDED_WINDOW
SELECT COUNT(*) FROM battle_records WHERE created_at >= now() - interval '30 days' AND (reaction_timeline IS NULL OR reaction_timeline->>'schema_version' IS DISTINCT FROM 'reaction-timeline-v1' OR reaction_timeline->'events' IS NULL OR jsonb_array_length(reaction_timeline->'events')=0);
\echo ALL_AGG_30D
SELECT e->>'reaction',SUM((e->>'damage')::bigint),COUNT(*) FROM battle_records br, jsonb_array_elements(br.reaction_timeline->'events') e WHERE br.created_at >= now()-interval '30 days' AND br.reaction_timeline->>'schema_version'='reaction-timeline-v1' GROUP BY 1 ORDER BY 2 DESC;
\echo DONE
