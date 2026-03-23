# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: MPL-2.0
#
# NomadOS Learning Environment - System Service Job
#
# This job demonstrates the core NomadOS concept: using Nomad to manage
# long-running system services instead of systemd.
#
# The job is of type "system", meaning Nomad will schedule one instance
# on EVERY eligible client node - the same way systemd would start a
# service on every machine in a traditional OS.
#
# In the original NomadOS project (jboero/nomados), this is how you would
# bootstrap a service like sshd, a web server, or a monitoring agent across
# all NomadOS nodes without any configuration management tool.
#
# Usage (with learning cluster running):
#   nomad job run /etc/nomad/jobs/system-service.hcl
#   nomad job status system-logger
#   nomad alloc logs -f <alloc-id>

job "system-logger" {
  datacenters = ["dc1"]

  # "system" type: run on ALL client nodes in the datacenter.
  # This replaces a systemd unit that would be enabled on every machine.
  type = "system"

  group "logger" {

    # Restart policy mirrors a systemd Restart=on-failure unit option
    restart {
      attempts = 5
      interval = "10m"
      delay    = "15s"
      mode     = "delay"
    }

    task "write-logs" {
      # raw_exec: run the task directly on the NomadOS host filesystem.
      # In a real NomadOS deployment this lets Nomad act as a true service
      # manager with zero container overhead.
      driver = "raw_exec"

      config {
        command = "/bin/bash"
        args = [
          "-c",
          # Simulates a long-running service: logs a heartbeat every 10 s.
          "while true; do echo \"[$(date -u +%FT%TZ)] NomadOS system-logger running on $(hostname)\"; sleep 10; done"
        ]
      }

      resources {
        cpu    = 50
        memory = 32
      }

      # Nomad captures stdout/stderr; in NomadOS this replaces journald.
      logs {
        max_files     = 5
        max_file_size = 10
      }
    }
  }
}
