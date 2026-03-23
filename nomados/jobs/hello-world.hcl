# Copyright (c) HashiCorp, Inc.
# SPDX-License-Identifier: MPL-2.0
#
# NomadOS Learning Environment - Hello World Job
#
# This is the simplest possible Nomad job: it runs a short-lived task that
# prints "Hello from NomadOS!" and exits.
#
# Submit this job to learn how the Nomad scheduler works.
#
# Usage (with learning cluster running):
#   nomad job run /etc/nomad/jobs/hello-world.hcl
#   nomad job status hello-world
#   nomad alloc logs <alloc-id>

job "hello-world" {
  # Datacenter must match the `datacenter` value in nomad-client.hcl
  datacenters = ["dc1"]

  # "batch" type: run once and stop (like a cron job or one-shot systemd unit)
  type = "batch"

  group "hello" {
    count = 1

    restart {
      attempts = 0
      mode     = "fail"
    }

    task "say-hello" {
      # "raw_exec" runs the command directly on the NomadOS node,
      # with no container runtime required - just like running a binary
      # from a Nomad job on the real NomadOS image.
      driver = "raw_exec"

      config {
        command = "/bin/bash"
        args    = ["-c", "echo 'Hello from NomadOS! Nomad version:' && nomad version"]
      }

      resources {
        cpu    = 50
        memory = 32
      }
    }
  }
}
