# Tasks: List Features Command

**Input**: Design documents from `/specs/001-list-features/`
**Prerequisites**: plan.md (✓), spec.md (✓)

**Tests**: Tests are NOT requested in this specification - focusing on implementation only.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)
- Include exact file paths in descriptions

## Path Conventions

- **Specify CLI**: `src/specify_cli/` at repository root (specify/)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 Create feature_list.py utility module at src/specify_cli/utils/feature_list.py
- [ ] T002 Verify typer and rich dependencies are available in pyproject.toml

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T003 Implement FeatureMetadata dataclass in src/specify_cli/utils/feature_list.py
- [ ] T004 Implement scan_features_directory() function to discover feature directories in src/specify_cli/utils/feature_list.py
- [ ] T005 Implement parse_spec_metadata() function to extract metadata from spec.md files in src/specify_cli/utils/feature_list.py

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - View All Features at a Glance (Priority: P1) 🎯 MVP

**Goal**: Enable developers to quickly see all features that have been created with their numbers, names, and status

**Independent Test**: Run the list command in a project with existing features and verify all features are displayed in numerical order

### Implementation for User Story 1

- [ ] T006 [US1] Add list() command function to src/specify_cli/__init__.py
- [ ] T007 [US1] Implement get_all_features() function in src/specify_cli/utils/feature_list.py
- [ ] T008 [US1] Implement format_features_table() function using Rich library in src/specify_cli/utils/feature_list.py
- [ ] T009 [US1] Add empty state handling (no features message) in list() command
- [ ] T010 [US1] Add error handling for missing specs directory in list() command
- [ ] T011 [US1] Add error handling for malformed spec files in parse_spec_metadata()
- [ ] T012 [US1] Update CHANGELOG.md with new list command entry
- [ ] T013 [US1] Update version in pyproject.toml

**Checkpoint**: At this point, User Story 1 should be fully functional - developers can list all features

---

## Phase 4: User Story 2 - Filter Features by Status (Priority: P2)

**Goal**: Enable developers to filter features by their status to focus on relevant work

**Independent Test**: Create features with different statuses and verify the filter correctly shows only matching features

### Implementation for User Story 2

- [ ] T014 [US2] Add --status option parameter to list() command in src/specify_cli/__init__.py
- [ ] T015 [US2] Implement filter_by_status() function in src/specify_cli/utils/feature_list.py
- [ ] T016 [US2] Add validation for status values (Draft, In Progress, Complete, On Hold) in list() command
- [ ] T017 [US2] Update format_features_table() to show filter criteria in output header
- [ ] T018 [US2] Add empty state handling for no matching features
- [ ] T019 [US2] Update CHANGELOG.md with status filter feature
- [ ] T020 [US2] Update version in pyproject.toml

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T021 [P] Add --search option for filtering by feature name in src/specify_cli/__init__.py
- [ ] T022 [P] Implement filter_by_search() function in src/specify_cli/utils/feature_list.py
- [ ] T023 [P] Update README.md with list command documentation
- [ ] T024 [P] Add help text examples for list command usage
- [ ] T025 Code cleanup and refactoring of feature_list.py
- [ ] T026 Add logging for feature discovery operations

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Extends US1 but should be independently testable (can add status filter without breaking basic list)

### Within Each User Story

- Core functionality before error handling
- Implementation before documentation updates
- Version bumps at end of each story

### Parallel Opportunities

- T001 and T002 in Setup can run together
- T003, T004, T005 in Foundational can run together (all in same file, but different functions)
- Once US1 completes, US2 can start (extends functionality)
- All Polish tasks marked [P] can run in parallel

---

## Parallel Example: User Story 1

```bash
# These can be developed in parallel once T003-T005 are complete:
Task T006: "Add list() command function to src/specify_cli/__init__.py"
Task T007: "Implement get_all_features() function in src/specify_cli/utils/feature_list.py"
Task T008: "Implement format_features_table() function in src/specify_cli/utils/feature_list.py"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T002)
2. Complete Phase 2: Foundational (T003-T005) - CRITICAL
3. Complete Phase 3: User Story 1 (T006-T013)
4. **STOP and VALIDATE**: Test the list command with existing features
5. Ready for use - basic feature listing works!

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy (MVP! - basic listing works)
3. Add User Story 2 → Test independently → Deploy (status filtering added)
4. Add Polish tasks → Final refinements
5. Each story adds value without breaking previous functionality

### Single Developer Strategy

1. Complete Setup (T001-T002)
2. Complete Foundational (T003-T005)
3. Implement User Story 1 (T006-T013) → Test → Commit
4. Implement User Story 2 (T014-T020) → Test → Commit
5. Polish tasks (T021-T026) → Final commit

---

## Summary

- **Total Tasks**: 26
- **User Story 1**: 8 tasks (T006-T013)
- **User Story 2**: 7 tasks (T014-T020)
- **Setup & Foundational**: 5 tasks (T001-T005)
- **Polish**: 6 tasks (T021-T026)
- **Parallel Opportunities**: 8 tasks can run in parallel (marked with [P])
- **Suggested MVP Scope**: User Story 1 only (Phase 1-3)

---

## Notes

- All tasks follow strict checklist format: `- [ ] [ID] [P?] [Story?] Description with file path`
- Each user story is independently testable
- MVP delivers core value (listing features)
- Incremental delivery enables early feedback
- No tests requested in specification - focusing on implementation
- Version bumps required per Specify CLI practices
