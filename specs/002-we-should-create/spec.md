# Feature Specification: MVP 1.0 - Core Survival Economy Loop

**Feature Branch**: `002-we-should-create`  
**Created**: 2025-09-08  
**Status**: Ready for Planning  
**Input**: User description: "we should create a starting phase where we could call it mvp 1.0, i would call it success when we have a small world map with some persons that I can command to cut wood, build houses that will bring more people, harvest fruits from bushes, bring things to stockpile and granary and consume from it for building and survive, it would be a good mvp 1.0, my idea is to keep the recipes simple as in stronghold 1 game"

## Execution Flow (main)
```
1. Parse user description from Input
   → If empty: ERROR "No feature description provided"
2. Extract key concepts from description
   → Identify: actors, actions, data, constraints
3. For each unclear aspect:
   → Mark with [NEEDS CLARIFICATION: specific question]
4. Fill User Scenarios & Testing section
   → If no clear user flow: ERROR "Cannot determine user scenarios"
5. Generate Functional Requirements
   → Each requirement must be testable
   → Mark ambiguous requirements
6. Identify Key Entities (if data involved)
7. Run Review Checklist
   → If any [NEEDS CLARIFICATION]: WARN "Spec has uncertainties"
   → If implementation details found: ERROR "Remove tech details"
8. Return: SUCCESS (spec ready for planning)
```

---

## ⚡ Quick Guidelines
- ✅ Focus on WHAT users need and WHY
- ❌ Avoid HOW to implement (no tech stack, APIs, code structure)
- 👥 Written for business stakeholders, not developers

### Section Requirements
- **Mandatory sections**: Must be completed for every feature
- **Optional sections**: Include only when relevant to the feature
- When a section doesn't apply, remove it entirely (don't leave as "N/A")

---

## User Scenarios & Testing *(mandatory)*

### Primary User Story
As a player, I want to start with a small settlement and a few workers that I can command to gather resources, construct buildings, and grow my population through a simple survival economy loop, so that I can experience the core gameplay of resource management and settlement building similar to Stronghold 1.

### Acceptance Scenarios
1. **Given** a new game starts, **When** the player views the world, **Then** they see a small map with initial workers and basic resources (trees, fruit bushes)
2. **Given** workers are idle, **When** the player commands them to cut wood, **Then** workers move to trees, harvest wood, and deliver it to the stockpile
3. **Given** workers are idle, **When** the player commands them to harvest fruit, **Then** workers collect fruit from bushes and deliver it to the granary
4. **Given** sufficient wood in stockpile, **When** the player places a house, **Then** workers construct the house and new population arrives
5. **Given** food in the granary, **When** time passes, **Then** population consumes food to survive
6. **Given** no food in granary, **When** time passes, **Then** population begins to leave or starve
7. **Given** resources in storage, **When** the player initiates construction, **Then** resources are consumed from stockpile according to building requirements

### Edge Cases
- What happens when all trees are cut down? (Trees regenerate over time)
- How does the system handle multiple workers assigned to the same resource?
- What occurs when stockpile or granary reaches capacity?
- How does population growth relate to available housing?
- What happens when workers are commanded to harvest depleted bushes?
- How are task priorities determined when multiple commands are given?

## Requirements *(mandatory)*

### Functional Requirements
- **FR-001**: System MUST generate a small world map with trees and fruit bushes at game start
- **FR-002**: System MUST provide initial workers (3-5) that player can command
- **FR-003**: Players MUST be able to command workers to cut wood from trees
- **FR-004**: Players MUST be able to command workers to harvest fruit from bushes
- **FR-005**: System MUST provide a stockpile for storing non-food resources (wood, stone)
- **FR-006**: System MUST provide a granary for storing food resources
- **FR-007**: Workers MUST automatically deliver harvested resources to appropriate storage
- **FR-008**: Players MUST be able to place houses for construction
- **FR-009**: System MUST consume wood from stockpile when constructing houses
- **FR-010**: System MUST increase population when new houses are completed
- **FR-011**: Population MUST consume food from granary at regular intervals
- **FR-012**: System MUST reduce population when food is unavailable (starvation/leaving)
- **FR-013**: System MUST display current resource quantities in stockpile and granary
- **FR-014**: System MUST display current population and available workers
- **FR-015**: Trees and bushes MUST regenerate resources over time after depletion
- **FR-016**: System MUST support simple building recipes (e.g., House = 5 Wood)
- **FR-017**: Workers MUST show visual feedback of current task (idle, harvesting, building, carrying)
- **FR-018**: System MUST prevent construction when insufficient resources available
- **FR-019**: System MUST support basic worker task assignment and pathfinding
- **FR-020**: Game MUST run in real-time with ability to pause

### Success Criteria for MVP 1.0
- Player can sustain a population of 10-20 people
- Complete resource loop: harvest → store → consume → build → grow population
- Simple, intuitive controls similar to Stronghold 1
- Stable gameplay for at least 30 minutes without critical issues
- Clear visual feedback for all player actions and system states

### Key Entities *(include if feature involves data)*
- **Worker**: Unit that performs tasks (cutting, harvesting, building, carrying) with current assignment and location
- **Tree**: Resource node providing wood when cut, with depletion and regeneration state
- **Fruit Bush**: Resource node providing food when harvested, with seasonal production
- **Stockpile**: Storage building for non-food resources with capacity and current inventory
- **Granary**: Storage building for food resources with capacity and current inventory
- **House**: Residential building supporting population growth, consuming resources to build
- **Resource**: Wood or food items with quantity, type, and storage location
- **Population**: Settlement inhabitants requiring food and housing, providing available workers

---

## Review & Acceptance Checklist
*GATE: Automated checks run during main() execution*

### Content Quality
- [x] No implementation details (languages, frameworks, APIs)
- [x] Focused on user value and business needs
- [x] Written for non-technical stakeholders
- [x] All mandatory sections completed

### Requirement Completeness
- [x] No [NEEDS CLARIFICATION] markers remain
- [x] Requirements are testable and unambiguous  
- [x] Success criteria are measurable
- [x] Scope is clearly bounded
- [x] Dependencies and assumptions identified

---

## Execution Status
*Updated by main() during processing*

- [x] User description parsed
- [x] Key concepts extracted
- [x] Ambiguities marked (none found)
- [x] User scenarios defined
- [x] Requirements generated
- [x] Entities identified
- [x] Review checklist passed

---

## MVP 1.0 Scope Summary

This specification defines a minimal but complete gameplay loop focusing on:
1. **Resource Collection**: Wood from trees, fruit from bushes
2. **Storage Management**: Stockpile for materials, granary for food
3. **Construction**: Simple houses using wood
4. **Population Growth**: New houses bring more people
5. **Survival Mechanics**: Population requires food to survive
6. **Simple Economy**: Straightforward recipes like Stronghold 1

The MVP deliberately excludes complex features like combat, trading, advanced crafting chains, or multiple resource types beyond wood and food, focusing instead on proving the core simulation loop works effectively.