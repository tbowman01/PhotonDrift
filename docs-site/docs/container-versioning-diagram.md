---
id: "container-versioning-diagram"
title: "ContAIner Versioning Diagram"
sidebar_label: "ContAIner Versioning Diagram"
sidebar_position: "1"
description: "Miscellaneous documentation and guides"
slug: "/misc/container-versioning-diagram"
tags: ["misc"]
last_update:
  date: "2025-07-28"
  author: "tbowman01"
---

---
title: "Container Versioning Flow Diagram"
sidebar_label: "Versioning Diagram"
sidebar_position: 1
description: "Visual representation of container versioning and tag generation flow for PhotonDrift"
tags: ["containers", "versioning", "docker", "deployment"]
---

# Container Versioning Flow Diagram

## Tag Generation Flow

```mermaid
graph TD
    A[Git Event] --> B{Event Type}
    
    B -->|Push to main| C[main branch tags]
    B -->|Push to develop| D[develop branch tags]
    B -->|Pull Request| E[PR tags]
    B -->|Git Tag/Release| F[Version tags]
    B -->|Schedule| G[Nightly tags]
    B -->|Feature Branch| H[Feature tags]
    
    C --> C1[latest]
    C --> C2[main]
    C --> C3[main-SHA]
    C --> C4[sha-XXXXXXXX]
    C --> C5[YYYYMMDD-SHA]
    
    D --> D1[edge]
    D --> D2[develop]
    D --> D3[develop-SHA]
    D --> D4[sha-XXXXXXXX]
    D --> D5[YYYYMMDD-SHA]
    
    E --> E1[pr-NNN]
    E --> E2[pr-NNN-SHA]
    E --> E3[sha-XXXXXXXX]
    
    F --> F1[X.Y.Z]
    F --> F2[X.Y]
    F --> F3[X]
    F --> F4[stable]
    F --> F5[latest]
    
    G --> G1[nightly]
    G --> G2[nightly-YYYYMMDD]
    G --> G3[edge]
    
    H --> H1[feature-NAME]
    H --> H2[feature-NAME-SHA]
    H --> H3[sha-XXXXXXXX]
```

## Multi-Architecture Build Flow

```mermaid
graph LR
    A[Source Code] --> B[Docker Buildx]
    
    B --> C[linux/amd64 Build]
    B --> D[linux/arm64 Build]
    
    C --> E[AMD64 Image Layer]
    D --> F[ARM64 Image Layer]
    
    E --> G[Manifest List]
    F --> G
    
    G --> H[Single Tag]
    
    H --> I[User Pull]
    I --> J{Platform?}
    J -->|x86_64| K[Pull AMD64]
    J -->|aarch64| L[Pull ARM64]
```

## Version Promotion Flow

```mermaid
graph LR
    A[Feature Branch] -->|PR| B[PR Build]
    B -->|Merge| C[Develop Branch]
    C -->|Nightly| D[Nightly Build]
    C -->|Promote| E[Main Branch]
    E -->|Tag| F[Release]
    
    B -.->|Tags| B1[pr-123<br/>pr-123-SHA]
    C -.->|Tags| C1[develop<br/>edge<br/>develop-SHA]
    D -.->|Tags| D1[nightly<br/>nightly-YYYYMMDD]
    E -.->|Tags| E1[main<br/>latest<br/>main-SHA]
    F -.->|Tags| F1[X.Y.Z<br/>X.Y<br/>X<br/>stable]
```

## Tag Immutability Matrix

```mermaid
graph TD
    A[Container Tags] --> B[Immutable Tags]
    A --> C[Mutable Tags]
    
    B --> B1[Version Tags<br/>X.Y.Z]
    B --> B2[SHA Tags<br/>sha-XXXXXXXX]
    B --> B3[Date-SHA Tags<br/>YYYYMMDD-SHA]
    B --> B4[Nightly Date<br/>nightly-YYYYMMDD]
    
    C --> C1[Branch Tags<br/>main, develop]
    C --> C2[Latest Tags<br/>latest, edge, stable]
    C --> C3[PR Tags<br/>pr-NNN]
    C --> C4[Feature Tags<br/>feature-XXX]
    C --> C5[Nightly<br/>nightly]
    
    style B1 fill:#90EE90
    style B2 fill:#90EE90
    style B3 fill:#90EE90
    style B4 fill:#90EE90
    style C1 fill:#FFB6C1
    style C2 fill:#FFB6C1
    style C3 fill:#FFB6C1
    style C4 fill:#FFB6C1
    style C5 fill:#FFB6C1
```

## Security and Attestation Flow

```mermaid
graph TD
    A[Build Complete] --> B[Generate SBOM]
    A --> C[Generate Provenance]
    A --> D[Security Scan]
    
    B --> E[Attach to Image]
    C --> E
    D --> F{Vulnerabilities?}
    
    F -->|None/Low| G[Sign Image]
    F -->|High/Critical| H[Block if PR]
    F -->|High/Critical| I[Warn if Release]
    
    G --> J[Push to Registry]
    H --> K[Fail Build]
    I --> J
    
    J --> L[Available for Pull]
    
    L --> M[User Pulls]
    M --> N[Verify Signature]
    M --> O[Check SBOM]
    M --> P[Verify Provenance]
```

## Tag Lifecycle Management

```mermaid
gantt
    title Tag Retention Timeline
    dateFormat YYYY-MM-DD
    
    section Version Tags
    X.Y.Z (Permanent)     :active, 2025-01-01, 3650d
    
    section Branch Tags
    latest (Rolling)      :active, 2025-01-01, 30d
    edge (Rolling)        :active, 2025-01-01, 30d
    main-SHA (90 days)    :active, 2025-01-01, 90d
    develop-SHA (30 days) :active, 2025-01-01, 30d
    
    section PR Tags
    pr-NNN (7 days)       :active, 2025-01-01, 7d
    
    section SHA Tags
    sha-XXXXX (90 days)   :active, 2025-01-01, 90d
    
    section Nightly Tags
    nightly (Rolling)     :active, 2025-01-01, 1d
    nightly-DATE (30 days):active, 2025-01-01, 30d
```

## Decision Tree for Tag Selection

```mermaid
graph TD
    A[Which tag to use?] --> B{Environment}
    
    B -->|Production| C{Deployment Type}
    C -->|Stable| D[Use: stable or X.Y.Z]
    C -->|Canary| E[Use: canary or SHA]
    C -->|Rollback| F[Use: previous X.Y.Z]
    
    B -->|Staging| G{Update Frequency}
    G -->|Auto-update| H[Use: latest or main]
    G -->|Controlled| I[Use: X.Y.Z or SHA]
    
    B -->|Development| J{Purpose}
    J -->|Latest Features| K[Use: edge or develop]
    J -->|Specific PR| L[Use: pr-NNN]
    J -->|Debugging| M[Use: sha-XXXXX]
    J -->|Feature Test| N[Use: feature-XXX]
    
    B -->|CI/CD| O{Build Type}
    O -->|Reproducible| P[Use: X.Y.Z or SHA]
    O -->|Latest| Q[Use: branch name]
    O -->|Nightly| R[Use: nightly-DATE]
```