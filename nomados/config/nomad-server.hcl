# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: MPL-2.0
#
# NomadOS Learning Environment - Server Configuration
#
# This is the Nomad server configuration for the learning cluster.
# The server is the control plane: it schedules jobs onto client (NomadOS) nodes.
#
# Reference: https://developer.hashicorp.com/nomad/docs/configuration

datacenter = "dc1"
region     = "global"
data_dir   = "/var/lib/nomad"
log_level  = "INFO"
bind_addr  = "0.0.0.0"

# Server stanza: enable server mode
server {
  enabled          = true
  # Single-node bootstrap for the learning cluster.
  # In production use an odd number (3 or 5) for HA.
  bootstrap_expect = 1
}

# Disable client mode on the server node so jobs only run on client nodes.
client {
  enabled = false
}

# Advertise addresses so that client nodes can reach the server.
# These IPs match the static addresses in docker-compose.yml.
advertise {
  http = "172.20.0.10:4646"
  rpc  = "172.20.0.10:4647"
  serf = "172.20.0.10:4648"
}

# Ports used by Nomad
ports {
  http = 4646
  rpc  = 4647
  serf = 4648
}
