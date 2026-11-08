# Apple Intelligence Local Inspection Resources

## Knowledge

- [Apple Developer: Foundation Models](https://developer.apple.com/documentation/FoundationModels)
  Primary framework overview. Use for: the supported Foundation Models API,
  structured generation, tool calling, and the distinction between on-device and
  Private Cloud Compute models.
- [Apple Developer: Detecting available Apple Foundation Models](https://developer.apple.com/documentation/FoundationModels/detecting-available-apple-foundation-models)
  Primary availability guidance. Use for: `SystemLanguageModel`, AFM variants,
  readiness states, and a fallback design.
- [Apple Developer: Generating content and performing tasks with Foundation Models](https://developer.apple.com/documentation/FoundationModels/generating-content-and-performing-tasks-with-foundation-models)
  Primary implementation guidance. Use for: sessions, prompts, tool calling,
  context limits, and model suitability.
- [8rank: Anatomy of Apple's Personal Knowledge Graph](https://8rank.ai/game/apple-intelligence-personal-knowledge-graph)
  Third-party reverse engineering observed on macOS 26.6.2. Use for:
  hypotheses about the local Intelligence Platform store; do not treat as a
  public API contract.

## Wisdom (Communities)

- [Apple Developer Forums — Machine Learning & AI](https://developer.apple.com/forums/tags/machine-learning)
  Apple-hosted developer discussion. Use for: framework behavior that differs
  across SDK and OS releases.

## Gaps

- Apple does not publish a supported public API that establishes a query path
  from Foundation Models to the Intelligence Platform store described by 8rank.
