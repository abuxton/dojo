# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: MPL-2.0
#
# NomadOS Learning Environment
#
# This Dockerfile creates a container that simulates the NomadOS concept:
# running HashiCorp Nomad as the primary process (analogous to PID 1 / init)
# rather than using systemd. Inspired by https://github.com/jboero/nomados
#
# The image provides a self-contained learning environment for understanding
# how Nomad can orchestrate workloads at the OS level without a traditional
# init system.
#
# Build:
#   docker build -t nomados-learn .
#
# Run (dev mode, single node):
#   docker run --privileged -p 4646:4646 -p 4647:4647 -p 4648:4648 nomados-learn
#
# Connect to the Nomad UI:
#   http://localhost:4646

FROM ubuntu:22.04

ARG NOMAD_VERSION=1.8.4
ARG NOMAD_ARCH=linux_amd64
ARG CNI_PLUGINS_VERSION=1.5.1

# Avoid interactive prompts during package install
ENV DEBIAN_FRONTEND=noninteractive
ENV NOMAD_ADDR=http://0.0.0.0:4646

# Install minimal dependencies
# - curl/unzip/gnupg: for downloading and verifying Nomad
# - iproute2: for `ip` command (mirrors nomadinit.c ifup behaviour)
# - procps: ps, kill utilities for job management
# - ca-certificates: for TLS connections from jobs
# - libcap2-bin: for setting capabilities (raw_exec driver)
RUN apt-get update && apt-get install -y --no-install-recommends \
        curl \
        unzip \
        gnupg \
        iproute2 \
        procps \
        ca-certificates \
        libcap2-bin \
        bash \
    && rm -rf /var/lib/apt/lists/*

# Install Nomad
# Download to /tmp using the original filename so the SHA256SUMS check works.
RUN cd /tmp \
    && curl -fsSL "https://releases.hashicorp.com/nomad/${NOMAD_VERSION}/nomad_${NOMAD_VERSION}_${NOMAD_ARCH}.zip" \
        -o "nomad_${NOMAD_VERSION}_${NOMAD_ARCH}.zip" \
    && curl -fsSL "https://releases.hashicorp.com/nomad/${NOMAD_VERSION}/nomad_${NOMAD_VERSION}_SHA256SUMS" \
        -o nomad_SHA256SUMS \
    && grep "nomad_${NOMAD_VERSION}_${NOMAD_ARCH}.zip" nomad_SHA256SUMS | sha256sum -c - \
    && unzip "nomad_${NOMAD_VERSION}_${NOMAD_ARCH}.zip" -d /usr/bin/ \
    && rm "nomad_${NOMAD_VERSION}_${NOMAD_ARCH}.zip" nomad_SHA256SUMS \
    && chmod +x /usr/bin/nomad

# Install CNI plugins (required for bridge networking in Nomad jobs)
RUN mkdir -p /opt/cni/bin \
    && curl -fsSL "https://github.com/containernetworking/plugins/releases/download/v${CNI_PLUGINS_VERSION}/cni-plugins-linux-amd64-v${CNI_PLUGINS_VERSION}.tgz" \
        -o /tmp/cni-plugins.tgz \
    && tar -xzf /tmp/cni-plugins.tgz -C /opt/cni/bin/ \
    && rm /tmp/cni-plugins.tgz

# Create the NomadOS-style directory layout.
# This mirrors the filesystem structure created by buildimg.sh in jboero/nomados:
#   /etc/nomad        - Nomad configuration (init.json in the original)
#   /var/lib/nomad    - Nomad data directory
#   /var/log          - Log output (nomad.log / nomad.err in the original)
#   /var/lib/nomad/data/plugins - Nomad driver plugins (e.g. nomad-driver-podman)
RUN mkdir -p \
        /etc/nomad.d \
        /var/lib/nomad/data/plugins \
        /var/lib/nomad/data/allocations \
        /var/lib/nomad/client \
        /var/log/nomad \
        /opt/cni/bin

# Copy Nomad configuration files
COPY config/nomad-client.hcl /etc/nomad.d/nomad.hcl

# Copy example job files for learners to explore
COPY jobs/ /etc/nomad/jobs/

# Expose Nomad ports:
#   4646 - HTTP API & UI
#   4647 - RPC (server-to-server communication)
#   4648 - Serf WAN (cluster gossip)
EXPOSE 4646 4647 4648

# Health check against the Nomad HTTP API
HEALTHCHECK --interval=10s --timeout=5s --start-period=30s --retries=5 \
    CMD curl -sf http://localhost:4646/v1/status/leader || exit 1

# entrypoint.sh sets up the environment and starts Nomad as PID 1,
# mirroring the role of nomadinit.c in the original NomadOS project.
COPY entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
