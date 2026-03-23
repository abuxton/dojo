#!/usr/bin/env bash
# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: MPL-2.0
#
# NomadOS Learning Environment - Entrypoint
#
# This script simulates the role of nomadinit.c from jboero/nomados:
# it sets up the minimal runtime environment and then hands control to
# Nomad, which runs as PID 1 (or as the primary long-running process).
#
# In the real NomadOS project this logic is compiled into a static C binary
# (nomadinit.c) and placed at /sbin/init so the Linux kernel invokes it
# directly after mounting the root filesystem. In a Docker container we
# achieve the same conceptual result by making this script the ENTRYPOINT.

set -e

echo "==> NomadOS Learning Environment starting..."
echo "    Inspired by https://github.com/jboero/nomados"
echo ""

# ---- Hostname -----------------------------------------------------------------
# In nomadinit.c: sethostname("nomados", 7)
# Sets a recognisable default hostname so nodes identify themselves correctly.
HOSTNAME="${NOMAD_NODE_NAME:-nomados}"
hostname "$HOSTNAME" 2>/dev/null || true
echo "==> Hostname: $(hostname)"

# ---- Loopback interface -------------------------------------------------------
# In nomadinit.c: ifup("lo")
ip link set lo up 2>/dev/null || true

# ---- Configure Nomad ----------------------------------------------------------
# Allow environment variables to override config at runtime (useful for
# switching between dev mode, server mode, and client mode).
NOMAD_MODE="${NOMAD_MODE:-dev}"
NOMAD_CONFIG_DIR="${NOMAD_CONFIG_DIR:-/etc/nomad.d}"

echo "==> Nomad mode: ${NOMAD_MODE}"
echo "==> Nomad config dir: ${NOMAD_CONFIG_DIR}"
echo ""

# ---- Start Nomad as PID 1 equivalent -----------------------------------------
# In nomadinit.c:
#   system("/usr/bin/nomad agent -dev -config=/etc/nomad/init.json >/var/log/nomad.log 2>/var/log/nomad.err&");
#
# Here we use exec so that Nomad replaces this shell process and receives
# signals (SIGTERM, SIGINT) directly - the correct behaviour for PID 1.

if [ "$NOMAD_MODE" = "dev" ]; then
    # Dev mode: single node acts as both server and client.
    # Ideal for local learning - no separate server needed.
    echo "==> Starting Nomad in dev mode (server + client combined)..."
    exec nomad agent \
        -dev \
        -bind=0.0.0.0 \
        -log-level=INFO \
        2>&1 | tee /var/log/nomad/nomad.log
elif [ "$NOMAD_MODE" = "server" ]; then
    echo "==> Starting Nomad in server mode..."
    exec nomad agent \
        -config="${NOMAD_CONFIG_DIR}" \
        -log-level=INFO \
        2>&1 | tee /var/log/nomad/nomad.log
elif [ "$NOMAD_MODE" = "client" ]; then
    echo "==> Starting Nomad in client mode (NomadOS node)..."
    exec nomad agent \
        -config="${NOMAD_CONFIG_DIR}" \
        -log-level=INFO \
        2>&1 | tee /var/log/nomad/nomad.log
else
    echo "ERROR: Unknown NOMAD_MODE '${NOMAD_MODE}'. Use 'dev', 'server', or 'client'."
    exit 1
fi
