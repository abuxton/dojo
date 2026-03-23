# Project N.O.M.A.D. — Local Learning Environment

[Project N.O.M.A.D.](https://github.com/Crosstalk-Solutions/project-nomad) (Node for Offline Media, Archives, and Data) is a self-contained, offline-first knowledge and education server.  This directory provides a Docker Compose setup so you can run it locally for learning and exploration **without touching your host system**.

## What's included

| Service | Image | Purpose | Port |
|---------|-------|---------|------|
| admin | `ghcr.io/crosstalk-solutions/project-nomad:latest` | Command-Center UI & API | 8080 |
| dozzle | `amir20/dozzle:v10.0` | Container log viewer | 9999 |
| mysql | `mysql:8.0` | Relational database | — |
| redis | `redis:7-alpine` | Cache / queue | — |
| updater | `…/project-nomad-sidecar-updater:latest` | In-app update sidecar | — |
| disk-collector | `…/project-nomad-disk-collector:latest` | Host disk-usage reporter | — |

All persistent data is stored in Docker-managed named volumes (`nomad-storage`, `nomad-mysql`, `nomad-redis`), so nothing is written outside this directory.

## Prerequisites

- [Docker Desktop](https://www.docker.com/products/docker-desktop/) ≥ 24 **or** Docker Engine + Compose plugin ≥ 2.x
- Access to the internet on first run (images are pulled from GitHub Container Registry and Docker Hub)
- The Docker socket (`/var/run/docker.sock`) must be accessible (standard on Linux/macOS)

## Quick start

```bash
# 1. Copy the example environment file and edit it
cp .env.example .env

# 2. Open .env and set the three required values:
#    APP_KEY  – any random string of ≥ 16 characters
#             (tip: openssl rand -hex 16)
#    URL      – leave as http://localhost:8080 for local use
#    DB_PASSWORD / MYSQL_PASSWORD – matching passwords for the database
#    MYSQL_ROOT_PASSWORD – root password for the MySQL container

# 3. Start all services in the background
docker compose up -d

# 4. Watch startup progress (optional – Ctrl-C to detach)
docker compose logs -f admin

# 5. Open your browser
open http://localhost:8080
```

The first launch may take a minute or two while MySQL initialises and the admin container runs its database migrations.

## Useful commands

```bash
# Check the status of all services
docker compose ps

# View logs for a specific service
docker compose logs -f admin
docker compose logs -f mysql

# Stop all services (data is preserved in volumes)
docker compose down

# Stop and remove all data (clean slate)
docker compose down -v

# Pull the latest images and restart
docker compose pull && docker compose up -d
```

## Accessing the UI

| URL | What you'll find |
|-----|-----------------|
| <http://localhost:8080> | N.O.M.A.D. Command Center |
| <http://localhost:9999> | Dozzle – live container logs |

## Learning notes

- **No authentication by default** — N.O.M.A.D. is designed for local/offline use.  Do not expose port 8080 directly to the internet.
- **AI tools are optional** — you can install Ollama and Qdrant from inside the Command Center after it starts; they are not included here because they require significant disk space and (ideally) a GPU.
- **Offline content** — Wikipedia ZIM files, Khan Academy (Kolibri), and offline maps can all be downloaded from inside the app; this compose file does not pre-download any content.
- **Updater sidecar** — the `updater` service watches for new releases and lets you upgrade from the N.O.M.A.D. UI.  In a pure learning environment you can safely remove it from the compose file.

## Troubleshooting

| Symptom | Likely cause | Fix |
|---------|-------------|-----|
| Admin container keeps restarting | `APP_KEY` missing or < 16 chars | Set a valid `APP_KEY` in `.env` |
| `Access denied` in MySQL logs | Password mismatch | Make sure `DB_PASSWORD` and `MYSQL_PASSWORD` are identical in `.env` |
| Port 8080 already in use | Another service on the host | Change the host port: `"8081:8080"` in `docker-compose.yml` |
| Disk-collector fails | Docker-in-Docker limitation | Remove the `disk-collector` service – it's cosmetic only |

## References

- [Project N.O.M.A.D. GitHub](https://github.com/Crosstalk-Solutions/project-nomad)
- [Official website](https://www.projectnomad.us)
- [Hardware requirements](https://www.projectnomad.us/hardware)
- [Discord community](https://discord.com/invite/crosstalksolutions)
