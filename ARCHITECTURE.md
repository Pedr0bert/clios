# Clios Shell – Architecture & Design Notes

## Purpose of This Document

This document describes the **architectural decisions, internal structure, and design trade-offs** of the Clios Shell. It is intended to complement the README by explaining *how* and *why* the system is built the way it is, rather than *what* features it provides.

Clios is a **technical study and Proof of Concept**, focused on systems programming, shell internals, and robustness.

---

## High-Level Architecture

At a high level, Clios is composed of five main subsystems:

1. **Input & Parsing Layer**
2. **Command Resolution Layer**
3. **Execution Engine**
4. **Job Control & Process Management**
5. **Extension Layer (Scripting & Plugins)**

Each subsystem is intentionally kept explicit and loosely coupled to improve debuggability and testability.

---

## 1. Input & Parsing Layer

The parsing layer is responsible for transforming raw user input into a structured command representation.

### Responsibilities

* Tokenization of user input
* Quote-aware argument parsing
* Detection of pipes (`|`), logical operators (`&&`, `||`), and redirections (`>`, `>>`, `2>`)
* Validation of malformed constructs (e.g. empty commands, invalid subshell syntax)

### Design Decisions

* Parsing is done explicitly rather than relying on external shell libraries
* Quoted segments are preserved during pipeline splitting to avoid semantic errors
* Parsing errors are reported early, before execution

This layer prioritizes **correctness and clarity** over permissiveness.

---

## 2. Command Resolution Layer

Once parsed, commands are classified and resolved.

### Command Types

* Built-in commands (e.g. `cd`, `alias`, `export`)
* External system binaries
* Scripted commands (Rhai execution)

### Resolution Strategy

* Built-ins are resolved first
* Aliases are expanded with recursion protection
* External commands are resolved using the system PATH

Recursive alias expansion is capped to prevent infinite loops.

---

## 3. Execution Engine

The execution engine is responsible for launching and coordinating processes.

### Capabilities

* Foreground and background execution
* Memory-based pipelines between processes
* I/O redirection for stdout and stderr
* Logical execution chaining (`&&`, `||`)

### Key Constraints

* Process spawning follows Unix semantics
* Errors are propagated explicitly and consistently
* Partial failures do not crash the shell

The engine favors **predictable behavior** over implicit recovery.

---

## 4. Job Control & Process Management

Clios implements basic job control to support interactive workflows.

### Supported Features

* Background execution using `&`
* Job suspension via `Ctrl+Z`
* Foreground resumption with `fg`

### Design Notes

* Job state is tracked internally with explicit process IDs
* Signal handling is minimal but deliberate
* The system avoids complex scheduling abstractions

This subsystem was implemented primarily as a learning exercise in Unix process control.

---

## 5. Extension Layer: Rhai Scripting

Clios integrates the **Rhai** scripting language as its extension mechanism.

### Motivation

* Avoid implementing a custom scripting language
* Provide safe, embeddable scripting with Rust integration
* Enable plugins without compromising system stability

### Plugin Model

* Plugins are loaded explicitly via `source`
* Only validated `.rhai` files are accepted
* Scripted functions are isolated from core shell state

This approach balances extensibility with safety.

---

## Error Handling Philosophy

Error handling in Clios follows three principles:

1. **Fail early** when input is invalid
2. **Fail loudly** with clear, consistent messages
3. **Fail locally** without cascading system instability

Errors are treated as first-class system events, not exceptional edge cases.

---

## Testing Strategy

Testing is divided into:

* **Unit tests** for core Rust components
* **Integration tests** simulating real shell usage

The goal is not exhaustive correctness, but confidence in core behaviors.

---

## Explicit Non-Goals

Clios intentionally avoids:

* Full POSIX shell compatibility
* Advanced shell features (e.g. shell expansion, globbing)
* Hidden automation or AI-driven behavior in the core system

These decisions keep the project focused and maintainable.

---

## Exploratory Work: Local AI Integration (Out of Tree)

Separate experimental work was conducted to evaluate **local language model integration** for command explanation and assistance.

### Observations

* Small local models (<1B parameters) exhibit hallucinations in command execution contexts
* Portuguese language comprehension is limited in lightweight models
* Larger models improve accuracy but reduce accessibility on embedded hardware

### Current Status

* AI integration is **not part of the Clios codebase**
* Experiments were conducted in isolated prototypes
* Future integration would require strict constraints, sandboxing, and opt-in usage

This exploration informs future research directions without impacting system reliability.

---

## Future Directions

Potential future work includes:

* Deeper integration with embedded Linux environments (e.g. Buildroot)
* Improved modularization of the execution engine
* Optional, constrained AI-assisted tooling as an external layer

All future features are evaluated against the core principles of explicitness, safety, and testability.

---

## Summary

Clios is a focused exploration of shell architecture using modern systems programming techniques. Its design reflects deliberate trade-offs made to maximize learning, reliability, and clarity rather than feature breadth.
