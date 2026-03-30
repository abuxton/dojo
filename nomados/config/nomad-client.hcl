# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: MPL-2.0
#
# NomadOS Learning Environment - Client (Agent) Configuration
#
# This configuration mirrors the role of config/init.json from jboero/nomados.
# In NomadOS a machine boots directly into Nomad using this config; the agent
# connects to the server and waits for jobs to be scheduled onto it.
#
# Reference: https://developer.hashicorp.com/nomad/docs/configuration

datacenter = "dc1"
region     = "global"
data_dir   = "/var/lib/nomad"
log_level  = "INFO"
bind_addr  = "0.0.0.0"

# Client stanza: enable client (worker) mode
client {
  enabled = true

  # Join the server. In NomadOS the original init.json used:
  #   "retry_join": ["nomad", "192.168.2.4"]
  # Here we use the server's fixed compose network address.
  server_join {
    retry_join     = ["172.20.0.10:4647"]
    retry_max      = 10
    retry_interval = "5s"
  }

  # NomadOS used exec and raw_exec to run workloads directly on the host.
  # raw_exec lets Nomad run arbitrary commands as root - powerful but requires
  # the privileged: true flag set in docker-compose.yml.
  options = {
    "driver.allowlist"     = "exec,raw_exec"
    "raw_exec.enable"      = "1"
  }

  state_dir = "/var/lib/nomad/client"
}

# Disable server mode on client nodes
server {
  enabled = false
}

# Ports used by the agent
ports {
  http = 4646
  rpc  = 4647
  serf = 4648
}
