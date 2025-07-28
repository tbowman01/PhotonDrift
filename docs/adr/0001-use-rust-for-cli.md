---
title: Use Rust for CLI Implementation
status: accepted
date: 2023-12-01
deciders: ["development team"]
tags: ["language", "performance", "tooling"]
---

# Use Rust for CLI Implementation

## Status

Accepted

## Context

We need to choose a programming language for implementing the ADRScan CLI tool.

## Decision

We will use Rust for implementing the CLI tool.

## Consequences

- Performance benefits from compiled language
- Memory safety without garbage collection
- Strong type system and pattern matching
- Excellent CLI libraries available (clap)
- Good cross-platform support