# Architecture Decision Records

This directory contains Architecture Decision Records (ADRs) for the pheromone project and other
significant architectural choices made in this repository.

## What is an ADR?

An ADR is a short document that captures an important architectural decision, the context in which
it was made, and its consequences. ADRs are immutable once accepted — superseded decisions get a
new ADR that references the old one.

## Format

Each ADR follows the template defined in [`.github/ISSUE_TEMPLATE/adr.md`](../../.github/ISSUE_TEMPLATE/adr.md).

## Status values

| Status     | Meaning                                                   |
|------------|-----------------------------------------------------------|
| Proposed   | Under discussion — not yet agreed upon                    |
| Accepted   | Decision agreed upon and in effect                        |
| Deprecated | No longer relevant but kept for historical reference      |
| Superseded | Replaced by a newer ADR (link to the superseding record)  |

## Index

| ADR                                                                   | Title                                              | Status   |
|-----------------------------------------------------------------------|----------------------------------------------------|----------|
| [ADR-001](ADR-001-xds-protocol-integration.md)                        | Review and Integration of Envoy xDS Protocol in the Pheromone Project | Proposed |

## Creating a new ADR

1. Open a GitHub issue using the **Architecture Decision Record (ADR)** issue template.
2. Discuss and refine the proposal in the issue.
3. Once agreed, add a new `ADR-XXXX-<slug>.md` file to this directory and update the index above.
4. Submit a Pull Request referencing the original issue.
