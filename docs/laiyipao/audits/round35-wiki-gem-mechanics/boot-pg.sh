#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
for i in $(seq 1 60); do
  docker info >/dev/null 2>&1 && break
  sleep 2
done
docker info >/dev/null 2>&1 && echo DOCKER_READY_AT=$i || echo DOCKER_NOT_READY
docker start lyp-qa-pg >/dev/null 2>&1; sleep 5
for i in $(seq 1 30); do
  docker exec lyp-qa-pg pg_isready -U postgres >/dev/null 2>&1 && break
  sleep 1
done
docker exec lyp-qa-pg pg_isready -U postgres 2>&1
docker exec lyp-qa-pg psql -U postgres -d laiyipao -tAc "select count(*),max(id) from battle_records;" 2>&1
