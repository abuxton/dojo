# ADR-001: Review and Integration of Envoy xDS Protocol in the Pheromone Project

**Status:** Proposed

**Date:** 2026-03-08

**Deciders:** abuxton

---

## Context

The pheromone project is exploring service-mesh and dynamic configuration management capabilities.
The Envoy Proxy project exposes a set of APIs collectively known as the **xDS (x Discovery Service)
protocol**, which is a well-established, vendor-neutral standard for transmitting configuration from
a management (control-plane) server to data-plane proxies such as Envoy.

The xDS protocol family (defined at
<https://www.envoyproxy.io/docs/envoy/v1.37.0/api-docs/xds_protocol>) includes the following
discovery services:

| Acronym | Full Name                  | Purpose                                               |
|---------|----------------------------|-------------------------------------------------------|
| LDS     | Listener Discovery Service | Dynamic listener (port/protocol) configuration        |
| RDS     | Route Discovery Service    | Dynamic HTTP route configuration                      |
| CDS     | Cluster Discovery Service  | Dynamic upstream cluster (backend) configuration      |
| EDS     | Endpoint Discovery Service | Dynamic endpoint (host/IP) configuration per cluster  |
| SDS     | Secret Discovery Service   | Dynamic TLS certificate and key material              |
| RTDS    | Runtime Discovery Service  | Dynamic runtime feature-flag configuration            |
| ECDS    | Extension Config Discovery | Dynamic filter/extension configuration                |

The protocol supports several delivery models:

- **State of the World (SotW)** – The management server sends a full snapshot of all resources on
  every update.
- **Incremental (Delta) xDS** – Only resource deltas (additions, modifications, removals) are
  streamed, reducing bandwidth for large configurations.
- **Aggregated Discovery Service (ADS)** – A single multiplexed gRPC stream carries all xDS
  resource types, removing ordering ambiguities between interdependent resources.

Transport is over **gRPC** (bidirectional streaming) or **REST/HTTP** (polling), using
**Protocol Buffers v3** as the wire format.

### Why this is relevant to pheromone

Pheromone aims to manage dynamic configuration distribution across a fleet of services. The xDS
protocol provides a battle-tested, extensible model for exactly this use-case, backed by the
[CNCF](https://www.cncf.io/) and adopted by projects including Istio, Consul Connect, and
AWS App Mesh. Evaluating xDS as the configuration transport layer for pheromone would enable
interoperability with the broader cloud-native ecosystem.

---

## Decision

Conduct a structured review of the Envoy xDS protocol specification (v1.37.0) and produce a
feasibility assessment for adopting xDS as the dynamic configuration transport in the pheromone
project. The review should cover:

1. **Protocol mechanics** – SotW vs. delta delivery, ADS multiplexing, ACK/NACK flow, and version
   management (nonces, resource versions).
2. **API surface** – Evaluate which xDS services (LDS, RDS, CDS, EDS, SDS, RTDS, ECDS) are
   relevant to pheromone's configuration model.
3. **Control-plane implementation** – Assess existing open-source control-plane SDKs
   (e.g., `go-control-plane`, `java-control-plane`) for use in pheromone's server component.
4. **Client-side integration** – Determine whether pheromone agents should embed a native xDS
   client or proxy through Envoy.
5. **Security** – Review SDS-based certificate lifecycle management and mTLS implications.
6. **Operational concerns** – Scalability of the management server, resource versioning strategy,
   and observability (xDS status endpoints).

The outcome of the review will be documented and used to decide whether to:

- **Adopt** xDS as pheromone's primary configuration protocol,
- **Adapt** a subset of xDS concepts while using a different transport, or
- **Reject** xDS in favour of an alternative approach.

---

## Rationale

Designing a bespoke configuration-distribution protocol would duplicate significant engineering
effort already solved by the xDS specification. Using an industry-standard protocol also lowers
the barrier for operators already familiar with Envoy-based service meshes, improves
interoperability with external tooling, and opens the possibility of running pheromone as a
drop-in control plane for existing Envoy fleets.

---

## Consequences

### Positive

- Alignment with CNCF/cloud-native ecosystem standards.
- Large body of existing documentation, tooling, and community experience to draw on.
- Mature gRPC transport with built-in bi-directional streaming, flow control, and TLS.
- Delta xDS reduces configuration-update overhead at scale.
- SDS removes the need to manage certificate files on disk in managed agents.

### Negative

- Protocol Buffers and gRPC introduce a more complex build and dependency graph compared to a
  simple REST/JSON approach.
- Full xDS compliance requires careful handling of ACK/NACK flows and nonce management; incorrect
  implementation leads to config-distribution stalls.
- The scope of the full xDS API surface is large; under-use may not justify the adoption cost.

### Neutral

- Pheromone would need to version its configuration schema in protobuf, which requires tooling
  discipline but also enables strong API contracts.
- Adopting ADS requires a stateful management server; SotW is simpler to implement but less
  efficient.

---

## Alternatives Considered

| Alternative                         | Notes                                                                                               |
|-------------------------------------|-----------------------------------------------------------------------------------------------------|
| Custom REST/JSON API                | Simpler to implement initially but diverges from ecosystem standards and requires bespoke clients.  |
| HashiCorp Consul Config Entries     | Consul-specific; good fit if the deployment targets Consul-only environments.                       |
| Kubernetes ConfigMap watch (list-watch) | Native K8s integration; limited to Kubernetes deployments and not suitable for non-K8s agents.  |
| gRPC server reflection + custom API | Retains gRPC benefits but lacks the ecosystem tooling and community support of xDS.                 |

---

## References

- Envoy xDS Protocol documentation (v1.37.0):
  <https://www.envoyproxy.io/docs/envoy/v1.37.0/api-docs/xds_protocol>
- CNCF `go-control-plane` SDK: <https://github.com/envoyproxy/go-control-plane>
- CNCF `java-control-plane` SDK: <https://github.com/envoyproxy/java-control-plane>
- xDS API working group: <https://github.com/cncf/xds>
- Envoy API reference (proto): <https://github.com/envoyproxy/envoy/tree/main/api>
