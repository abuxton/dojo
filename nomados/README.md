# NomadOS Learning Environment

> **Based on:** [jboero/nomados](https://github.com/jboero/nomados) — an experimental project that replaces systemd with HashiCorp Nomad as the Linux init system.

---

## Summary

**NomadOS** (also called *Nomad Init*) is an experimental operating system design by John Boero that turns the traditional cloud model upside-down. Instead of:

```
Terraform → provision OS → install systemd → deploy workloads
```

NomadOS does:

```
Boot kernel → Nomad (as PID 1 / init) → Nomad schedules all workloads
```

The OS boots a minimal Linux kernel, runs a tiny C binary (`nomadinit.c`) as `/sbin/init`, which mounts filesystems and brings up networking, then immediately executes **Nomad as the primary long-running process**. From that point on, every service on the machine — monitoring agents, container runtimes, application servers — is a Nomad job.

### Why This Matters

| Traditional OS | NomadOS |
|---|---|
| systemd manages services | Nomad manages services |
| Services configured per-machine via unit files | Services declared as Nomad jobs, applied cluster-wide |
| New service = edit files on each node | New service = `nomad job run service.hcl` |
| Machine-centric mental model | Workload-centric mental model |
| Separate orchestration layer (Kubernetes, Nomad) | Single layer: Nomad IS the OS |

This learning environment lets you explore the NomadOS concept without needing to build a QCOW2 image or a bare-metal machine.

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    NomadOS Cluster                       │
│                                                          │
│  ┌──────────────┐   ┌──────────────┐  ┌──────────────┐  │
│  │ nomad-server │   │ nomados-node-│  │ nomados-node-│  │
│  │ (control     │   │     1        │  │     2        │  │
│  │  plane)      │   │ (NomadOS     │  │ (NomadOS     │  │
│  │              │   │  client)     │  │  client)     │  │
│  │ :4646 UI/API │   │              │  │              │  │
│  │ :4647 RPC    │   │ Nomad = PID1 │  │ Nomad = PID1 │  │
│  │ :4648 Serf   │   │              │  │              │  │
│  └──────┬───────┘   └──────┬───────┘  └──────┬───────┘  │
│         │                  │                  │          │
│         └──────────────────┴──────────────────┘          │
│                    172.20.0.0/24                         │
└─────────────────────────────────────────────────────────┘
```

In the original NomadOS project, the `nomad-server` would also be a NomadOS node (or a standard Nomad server). The clients (`nomados-node-*`) are the machines that boot into Nomad directly — here simulated as containers.

---

## Quick Start

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/) ≥ 20.10
- [Docker Compose](https://docs.docker.com/compose/install/) ≥ 2.0

### Option 1 — Single-Node Dev Mode (fastest)

Spins up a single Nomad node that acts as both server and client. Perfect for exploring the Nomad API and running example jobs.

```bash
# Build the image
docker build -t nomados-learn .

# Run in dev mode (Nomad as PID 1)
docker run --rm --privileged \
  -p 4646:4646 \
  -e NOMAD_MODE=dev \
  --name nomados-dev \
  nomados-learn
```

Open the Nomad UI at **http://localhost:4646**.

Run a job:

```bash
# Copy and submit the hello-world job
docker exec nomados-dev nomad job run /etc/nomad/jobs/hello-world.hcl

# Check job status
docker exec nomados-dev nomad job status hello-world

# View allocation logs
docker exec nomados-dev sh -c 'nomad alloc logs $(nomad alloc list -json | python3 -c "import sys,json; print(json.load(sys.stdin)[0][\"ID\"])")'
```

### Option 2 — Multi-Node Cluster (full NomadOS simulation)

Runs one Nomad server and two NomadOS-style client nodes.

```bash
# Start the cluster
docker compose up -d

# Watch the nodes come online
docker compose logs -f
```

Once the cluster is healthy, submit jobs:

```bash
# Submit a batch job (runs once)
docker exec nomad-client-1 nomad job run /etc/nomad/jobs/hello-world.hcl

# Submit a system job (runs on EVERY node, like a systemd service)
docker exec nomad-client-1 nomad job run /etc/nomad/jobs/system-service.hcl

# List all running jobs
docker exec nomad-client-1 nomad job list

# Check node status
docker exec nomad-client-1 nomad node list
```

Nomad UI: **http://localhost:4646**

Tear down:

```bash
docker compose down -v
```

---

## Building the Real NomadOS Image

The original project produces a bootable QCOW2/VMDK image that can run on bare metal, QEMU, or in a cloud. The following steps are adapted from `buildimg.sh` in [jboero/nomados](https://github.com/jboero/nomados).

### Prerequisites (Fedora/RHEL)

```bash
sudo dnf install -y \
  qemu-img \
  qemu-kvm \
  extlinux \
  gcc \
  git \
  nomad        # from HashiCorp yum repo
```

Load the `nbd` kernel module:

```bash
sudo modprobe nbd
```

### Step 1 — Build the Linux Kernel

NomadOS uses a minimal kernel configured only for KVM guest use:

```bash
git clone --depth=1 git://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git
cd linux
make allnoconfig          # start from a clean slate
make kvm_guest.config     # add KVM guest essentials
make -j$(nproc)           # compile
```

The output kernel image will be at `arch/x86/boot/bzImage`.

### Step 2 — Compile nomadinit

`nomadinit.c` is the tiny C init binary that replaces `/sbin/init`. It mounts filesystems, brings up networking, and executes Nomad:

```bash
git clone https://github.com/jboero/nomados.git
cd nomados

gcc -I /usr/include nomadinit.c -static -o nomadinit
```

### Step 3 — Download Nomad

```bash
# Check the latest version at https://releases.hashicorp.com/nomad/
# and update NOMAD_VERSION accordingly, or match the version in the Dockerfile.
NOMAD_VERSION=1.8.4
curl -fsSL "https://releases.hashicorp.com/nomad/${NOMAD_VERSION}/nomad_${NOMAD_VERSION}_linux_amd64.zip" \
  -o nomad.zip
unzip nomad.zip
```

### Step 4 — Build the QCOW2 Image

```bash
# Set paths
export KERNEL_PATH=../linux/arch/x86/boot/bzImage
export BOOT_DEV=/dev/sda

# Run the build script (requires root for qemu-nbd and mkfs)
sudo bash buildimg.sh
```

The script will:
1. Create a 100 GB sparse QCOW2 file
2. Mount it via `qemu-nbd`
3. Format with ext4 and install the NomadOS filesystem layout
4. Copy the kernel, init binary, Nomad binary, and DHCP client
5. Install the extlinux bootloader
6. Compact and optionally export as VMDK

Output: `/tmp/hashios_x86_64.qcow2` and `/tmp/hashios_x86_64.vmdk`

### Step 5 — Run with QEMU

```bash
qemu-system-x86_64 \
  -m 2G \
  -smp cpus=4 \
  -nographic \
  -kernel bzImage \
  -append "console=ttyS0 root=/dev/sda selinux=0" \
  -net nic,model=virtio \
  -net bridge,br=virbr0 \
  -hda /tmp/hashios_x86_64.qcow2 \
  --enable-kvm
```

At boot you will see `nomadinit` mount filesystems, configure `eth0` via DHCP, then hand off to `nomad agent`.

---

## Running NomadOS in the Cloud

NomadOS images can be uploaded to cloud providers that accept custom disk images. The main constraint is bootloader compatibility:

| Cloud | Notes |
|---|---|
| **AWS** | Convert VMDK to AMI using VM Import/Export. Requires UEFI or BIOS GRUB/syslinux support. |
| **GCP** | Import as a custom image (raw disk). Use `--guest-os-features VIRTIO_SCSI_MULTIQUEUE`. |
| **Azure** | Upload VHD. Use Gen1 VM for BIOS boot. |
| **Bare metal / On-prem** | Best option — full BIOS/UEFI control. |

The original extlinux bootloader configuration (`config/extlinux.conf`):

```
UI /syslinux/vesamenu.c32
TIMEOUT 1
DEFAULT nomados
LABEL nomados
  KERNEL /syslinux/bzImage
  APPEND toram root=/dev/sda selinux=0
```

> **Note:** Some cloud providers require UEFI/secure boot, which is not supported in the original project. The experimental nature of the project means cloud support may require additional work.

---

## Key Concepts

### Nomad as Init System

In a standard Linux system the init process (PID 1) is:
- `systemd` on most modern distros
- `sysvinit`, `runit`, `openrc` on others

In NomadOS, the kernel launches `nomadinit` (a compiled C binary) which in turn starts `nomad agent` as the primary long-running process. Nomad then manages everything else via jobs.

### The raw_exec Driver

NomadOS relies on Nomad's `raw_exec` task driver, which runs commands directly on the host filesystem as a specified user (typically root). This is the equivalent of a systemd `ExecStart=` with `User=root`.

```hcl
task "my-service" {
  driver = "raw_exec"
  config {
    command = "/usr/bin/my-service"
    args    = ["--flag"]
  }
}
```

### System Jobs

A Nomad job with `type = "system"` runs one allocation on every eligible node, making it equivalent to a systemd unit that is enabled on every machine in the cluster:

```hcl
job "node-exporter" {
  type = "system"
  # ... runs on all NomadOS nodes automatically
}
```

### Converting systemd Units to Nomad Jobs

The repository includes helper scripts in `config/`:

```bash
# Convert a systemd unit file to a Nomad HCL job
bash config/systemd2nomad.hcl.sh /lib/systemd/system/nginx.service > nginx.hcl
nomad job run nginx.hcl
```

---

## File Structure

```
nomados/
├── Dockerfile              # NomadOS learning container (Nomad as PID 1)
├── docker-compose.yml      # Multi-node cluster (server + 2 NomadOS clients)
├── entrypoint.sh           # Container init (simulates nomadinit.c behaviour)
├── config/
│   ├── init.json           # Original NomadOS init config (from jboero/nomados)
│   ├── nomad-server.hcl    # Nomad server configuration
│   └── nomad-client.hcl    # NomadOS client configuration
└── jobs/
    ├── hello-world.hcl     # Batch job: run once and exit
    └── system-service.hcl  # System job: run on every NomadOS node
```

---

## References

- [jboero/nomados](https://github.com/jboero/nomados) — original NomadOS project
- [Nomad vs systemd (blog post)](https://medium.com/@boeroboy/nomad-vs-systemd-e0db80d34e8a) — design rationale by John Boero
- [HashiCorp Nomad Documentation](https://developer.hashicorp.com/nomad/docs)
- [Nomad raw_exec driver](https://developer.hashicorp.com/nomad/docs/drivers/raw_exec)
- [Nomad system jobs](https://developer.hashicorp.com/nomad/docs/job-specification/job#type)
- [NomadBSD](https://nomadbsd.org/) — unrelated project (FreeBSD-based), not to be confused with NomadOS
