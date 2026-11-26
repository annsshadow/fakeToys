#!/bin/bash
docker exec lyp-qa-pg pg_dump -U postgres -Fc laiyipao > "docs/laiyipao/audits/round16-admin-reaction-timeline/data/qa-before.dump"
sha256sum "docs/laiyipao/audits/round16-admin-reaction-timeline/data/qa-before.dump"
echo SNAPSHOT_DONE
