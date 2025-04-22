# 📜 Event System Overview (Bevy)

This document outlines all the major events used in the game, organized by category. Events are the primary way that gameplay logic is decoupled and driven in real-time.

---

## 🧠 Event Design Philosophy

The event system is centered around **generic `Actor*` events**, which are used universally across all in-world entities: **Player, NPCs, Enemies**, etc. These events allow systems to handle behaviors uniformly without knowing the specific type of actor.

However, in several situations—especially within UI or input systems—the code triggering the event already knows it's operating on the player and doesn't need to query for them. For this reason, specialized `Player*` events exist. These are thin wrappers that:

- **Contain only player-specific data** (e.g., `to_tile`, `defender`)
- Are **emitted directly** in systems that operate on the player
- Are **forwarded into their matching `Actor*` event**, so that shared game logic (combat, movement, etc.) continues to work generically

This separation simplifies logic in player-centric systems while keeping most gameplay logic consistent through `Actor*` events.

---

## 🧭 Movement Events

| Event | Description |
|-------|-------------|
| `ActorGridMoveEvent` | Fired when any actor moves from one tile to another. Used to trigger tile-based effects (e.g., traps, pressure plates, fog of war). |
| `PlayerGridMoveEvent` | Fired when the player moves to a new tile. Can be used for camera follow, UI updates, or discovery logic. Internally dispatches an `ActorGridMoveEvent`. |

---

## ⚔️ Combat Events

| Event | Description |
|-------|-------------|
| `ActorAttackEvent` | Fired when any actor initiates an attack against another entity. Typically used to resolve hit/miss logic. |
| `PlayerAttackEvent` | A specialized version for when the player initiates an attack. Can be used for UI highlighting or combat log display. Internally dispatches an `ActorAttackEvent`. |
| `ActorHitEvent` | Fired when an attack successfully hits. Contains data on attacker, defender, damage, and critical hit info. |
| `ActorMissEvent` | Fired when an attack fails to hit the target. Useful for animations, sound effects, or evasion effects. |
| `ActorDeathEvent` | Fired when an actor dies as a result of taking damage. Used to trigger death animations, despawn logic, and loot drops. |

---

## 🎒 Inventory Events

| Event | Description |
|-------|-------------|
| `ActorItemPickupEvent` | Fired when an actor picks up an item from the ground. Can be used to update UI or trigger equipment logic. |

---

## 🗨️ Dialogue Events

| Event | Description |
|-------|-------------|
| `ActorDialogInitiatedEvent` | Fired when two NPCs engage in dialog (or when AI tries to talk). |
| `PlayerDialogInitiatedEvent` | Fired when the player begins a dialog with an NPC. Triggers dialog UI or cinematic mode. Internally emits an `ActorDialogInitiatedEvent`. |

---

## 🧬 Actor Lifecycle Events

| Event | Description |
|-------|-------------|
| `ActorSpawnEvent` | *(Currently unused)* Potential hook for initialization logic when actors are spawned into the world. |

---

Let me know if you want to split this into separate `.md` files per category, generate a Table of Contents, or add links to relevant systems/components.
