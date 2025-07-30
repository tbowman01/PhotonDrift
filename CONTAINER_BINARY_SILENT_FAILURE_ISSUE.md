# Critical: adrscan Binary Silent Failure in Container Environment

## 🚨 Issue Summary

**Priority:** Critical  
**Type:** bug, runtime  
**Component:** container, core, runtime  
**Phase:** Production Deployment  
**Severity:** Blocking container deployments

The adrscan binary fails silently when executed in container environments, producing no output or error messages despite internal panics and errors being present in the binary (visible via `strings` analysis).

## 🔍 Problem Description

### Observed Behavior
```bash
# Container execution - complete silence
/usr/local/bin $ ./adrscan
# <no output, no error, immediate return>

# Binary analysis reveals internal panics/errors
/usr/local/bin $ strings ./adrscan | grep -i panic
# <extensive panic and error strings found>
```

### Critical Symptoms
- **Silent Failure**: Binary executes but produces no output whatsoever
- **No Error Messages**: No stderr or stdout output despite failures
- **Immediate Exit**: Process returns immediately without performing expected operations
- **Hidden Panics**: `strings` analysis reveals internal Rust panics and runtime errors
- **Container-Specific**: Issue appears specific to containerized environment

## 📊 Evidence Analysis

### Panic Patterns Found in Binary
**Critical Runtime Errors:**
```rust
library/std/src/rt.rs
fatal runtime error: unwrap failed: CString::new("main")
fatal runtime error: assertion failed: thread_info.stack_guard.get().is_none()
fatal runtime error: global allocator may not use TLS
fatal runtime error: stack overflow
```

**Memory Management Issues:**
```rust
library/alloc/src/raw_vec.rs - capacity overflow
library/alloc/src/vec/mod.rs
out of memory
```

**Thread Local Storage (TLS) Issues:**
```rust
cannot access a Thread Local Storage value during or after destruction
library/std/src/thread/local.rs
use of std::thread::current() is not possible after thread's local data destroyed
```

**I/O and System Interface Issues:**
```rust
library/std/src/io/stdio.rs
failed to write whole buffer
formatter error
library/std/src/panic.rs
```

## 🎯 Root Cause Hypotheses

### 1. Thread Local Storage (TLS) Issues
**Likelihood**: High  
**Evidence**: Multiple TLS-related fatal errors in strings dump
**Root Cause**: Container environment may have different TLS behavior
**Impact**: Runtime initialization failure leading to silent exit

### 2. Signal Handling Problems
**Likelihood**: High  
**Evidence**: `fatal runtime error: assertion failed: signal(libc::SIGPIPE, handler) != libc::SIG_ERR`
**Root Cause**: Container signal handling differences causing panic abort
**Impact**: Process terminates before any output

### 3. Memory Allocator Issues
**Likelihood**: Medium  
**Evidence**: `fatal runtime error: global allocator may not use TLS`
**Root Cause**: Rust memory allocator incompatibility with container environment
**Impact**: Memory allocation failures during startup

### 4. Standard I/O Redirection Issues
**Likelihood**: Medium  
**Evidence**: `library/std/src/io/stdio.rs` errors and formatter issues
**Root Cause**: stdout/stderr not properly connected in container
**Impact**: Output suppression despite internal operations

### 5. Static Linking Issues
**Likelihood**: Medium  
**Evidence**: Multiple library path references and runtime initialization failures
**Root Cause**: Missing dynamic libraries or incorrect static linking
**Impact**: Runtime symbol resolution failures

## 🔧 Diagnostic Investigation Plan

### Phase 1: Environment Analysis (1-2 hours)
**Goal**: Understand container runtime environment

**Tasks**:
- [ ] **Container Runtime Analysis**
  ```bash
  # Inside container
  ldd /usr/local/bin/adrscan
  file /usr/local/bin/adrscan
  readelf -l /usr/local/bin/adrscan
  cat /proc/version
  ls -la /lib64/ld-linux-*
  ```

- [ ] **System Call Tracing** (if strace available)
  ```bash
  # Install debugging tools in container
  apk add --no-cache strace gdb
  strace -f -e trace=all ./adrscan 2>&1 | head -50
  ```

- [ ] **Library Dependencies Check**
  ```bash
  # Check for missing libraries
  objdump -p /usr/local/bin/adrscan | grep NEEDED
  find /lib* /usr/lib* -name "*.so*" | grep -E "(pthread|c|m|dl)"
  ```

### Phase 2: Rust Runtime Debugging (2-3 hours)
**Goal**: Identify specific Rust runtime initialization issues

**Tasks**:
- [ ] **Environment Variable Testing**
  ```bash
  # Test different Rust environment configurations
  RUST_BACKTRACE=1 ./adrscan
  RUST_BACKTRACE=full ./adrscan
  RUST_LOG=debug ./adrscan
  RUST_MIN_STACK=8388608 ./adrscan  # 8MB stack
  ```

- [ ] **Thread and TLS Testing**
  ```bash
  # Test single-threaded execution
  RUST_TEST_THREADS=1 ./adrscan
  # Test with different thread models
  LD_PRELOAD=/lib/libc.so.6 ./adrscan
  ```

- [ ] **Signal Handling Analysis**
  ```bash
  # Test signal behavior
  timeout 10s ./adrscan
  kill -SIGTERM $$ && ./adrscan
  ```

### Phase 3: Binary Analysis (1-2 hours)
**Goal**: Deep analysis of binary construction and linking

**Tasks**:
- [ ] **Symbol Analysis**
  ```bash
  nm -D /usr/local/bin/adrscan | grep -E "(main|panic|abort)"
  objdump -t /usr/local/bin/adrscan | grep -E "(main|_start)"
  readelf -s /usr/local/bin/adrscan | grep -E "(GLOBAL|WEAK)"
  ```

- [ ] **Section Analysis**
  ```bash
  readelf -S /usr/local/bin/adrscan
  objdump -h /usr/local/bin/adrscan
  size /usr/local/bin/adrscan
  ```

- [ ] **Dynamic Analysis** (if GDB available)
  ```bash
  gdb ./adrscan
  (gdb) set environment RUST_BACKTRACE=1
  (gdb) run
  (gdb) bt
  ```

### Phase 4: Container Environment Testing (1-2 hours)
**Goal**: Isolate container-specific issues

**Tasks**:
- [ ] **Base Image Testing**
  ```bash
  # Test on different base images
  docker run --rm -it alpine:latest /bin/sh
  docker run --rm -it debian:slim /bin/bash
  docker run --rm -it ubuntu:latest /bin/bash
  ```

- [ ] **Capability and Security Testing**
  ```bash
  # Test with different security contexts
  docker run --privileged --rm -it <image> ./adrscan
  docker run --cap-add=SYS_PTRACE --rm -it <image> ./adrscan
  ```

- [ ] **Volume Mount Testing**
  ```bash
  # Test with host filesystem access
  docker run --rm -v $(pwd):/test -it <image> /test/adrscan
  ```

## 🛠️ Fix Implementation Strategy

### Immediate Fixes (High Confidence)

#### 1. Signal Handling Fix
**File**: `src/main.rs` or `src/lib.rs`
```rust
// Add early in main() function
fn setup_signal_handling() {
    #[cfg(unix)]
    {
        unsafe {
            // Ignore SIGPIPE to prevent sudden termination
            libc::signal(libc::SIGPIPE, libc::SIG_IGN);
        }
    }
}
```

#### 2. Panic Hook Configuration
**File**: `src/main.rs`
```rust
use std::panic;

fn main() {
    // Set up panic hook for container environments
    panic::set_hook(Box::new(|panic_info| {
        eprintln!("PANIC: {}", panic_info);
        if let Some(location) = panic_info.location() {
            eprintln!("  at {}:{}:{}", location.file(), location.line(), location.column());
        }
        std::process::exit(1);
    }));
    
    // Rest of main function
}
```

#### 3. Error Output Forcing
**File**: `src/main.rs`
```rust
// Force stderr/stdout to be unbuffered
use std::io::{self, Write};

fn main() {
    // Ensure stdout/stderr are available
    let _ = io::stderr().flush();
    let _ = io::stdout().flush();
    
    // Add explicit error reporting
    if let Err(e) = run_main() {
        eprintln!("ERROR: {}", e);
        std::process::exit(1);
    }
}
```

### Build Configuration Fixes

#### 1. Static Linking Configuration
**File**: `Cargo.toml`
```toml
[profile.release]
lto = "fat"
codegen-units = 1
panic = "unwind"  # Change from "abort" to "unwind"
strip = false     # Keep symbols for debugging
```

#### 2. Container-Specific Build Target
**File**: `.cargo/config.toml`
```toml
[build]
target = "x86_64-unknown-linux-musl"  # Static linking for containers

[target.x86_64-unknown-linux-musl]
rustflags = ["-C", "target-feature=+crt-static"]
```

#### 3. Dockerfile Modifications
**File**: `Dockerfile`
```dockerfile
# Add debugging capabilities
RUN apk add --no-cache gdb strace

# Set runtime environment
ENV RUST_BACKTRACE=1
ENV RUST_LOG=error

# Test binary during build
RUN /usr/local/bin/adrscan --version || echo "Binary test failed"
```

## 🧪 Testing & Validation Strategy

### 1. Local Testing
```bash
# Build with debugging
cargo build --target x86_64-unknown-linux-musl --release

# Test in local container
docker build -t adrscan-debug .
docker run --rm adrscan-debug ./adrscan --help
```

### 2. Progressive Testing
```bash
# Test basic functionality
echo "Testing basic execution..."
docker run --rm adrscan-debug ./adrscan --version

# Test with sample ADR directory
echo "Testing with sample data..."
docker run --rm -v $(pwd)/test-data:/data adrscan-debug ./adrscan inventory --adr-dir /data
```

### 3. Regression Testing
```bash
# Ensure fix doesn't break host execution
./target/release/adrscan --version
./target/release/adrscan --help
```

## 📊 Success Criteria

### Must Have
- [ ] **Binary produces output** in container environment
- [ ] **Error messages visible** when failures occur
- [ ] **Proper exit codes** returned for success/failure cases
- [] **Basic commands functional** (--help, --version, inventory)

### Should Have
- [ ] **Panic messages captured** and displayed properly
- [ ] **Debugging information** available when RUST_BACKTRACE=1
- [ ] **Performance maintained** (no significant slowdown)
- [ ] **Container size impact** minimized

### Could Have
- [ ] **Enhanced error reporting** with context information
- [ ] **Container health checks** integration
- [ ] **Logging integration** with container orchestration
- [ ] **Metrics collection** for container monitoring

## 🚨 Risk Assessment

### High Risk
- **ABI Compatibility**: Changes to panic handling might affect ABI
- **Performance Impact**: Unwind vs abort panic has performance implications
- **Security Implications**: Debug symbols in production containers

### Medium Risk
- **Build Complexity**: Adding musl target increases build complexity
- **Testing Coverage**: Need comprehensive testing across environments
- **Container Size**: Debug symbols and static linking increase size

### Low Risk
- **Signal Handling**: Standard Unix signal handling practices
- **Error Output**: Standard stderr/stdout practices

### Mitigation Strategies
- **Phased Rollout**: Test fixes incrementally
- **Fallback Plan**: Keep current build configuration as backup
- **Comprehensive Testing**: Test across multiple container environments

## 💼 Business Impact

### Current Impact (Critical)
- **Container Deployments Blocked**: Cannot deploy PhotonDrift in production containers
- **User Experience Degraded**: Silent failures provide no debugging information
- **Adoption Hindered**: Container-first deployment strategies blocked
- **Support Burden**: Difficult to troubleshoot user-reported issues

### Post-Fix Benefits
- **Production Ready**: Reliable container deployments
- **Better Debugging**: Clear error messages and failure modes
- **Enhanced Adoption**: Container-friendly deployment options
- **Reduced Support**: Self-diagnosing error messages

## 📅 Implementation Timeline

### Week 1: Investigation & Diagnosis
- **Days 1-2**: Complete diagnostic investigation (Phases 1-4)
- **Days 3-4**: Implement immediate fixes (signal handling, panic hooks)
- **Day 5**: Testing and validation of fixes

### Week 2: Comprehensive Solution
- **Days 1-2**: Build configuration updates and container optimization
- **Days 3-4**: Comprehensive testing across environments
- **Day 5**: Documentation and deployment guide updates

## 🔗 Related Issues

### Dependencies
- **Container Build System**: May need Dockerfile updates
- **CI/CD Pipeline**: Container testing integration required
- **Documentation**: User guides need container troubleshooting section

### Potentially Related
- **Issue #87**: Docker binary verification tests
- **Build System**: General container build improvements
- **Error Handling**: Overall error reporting enhancements

## ✅ Acceptance Criteria

### Functional Requirements
- [ ] `./adrscan --version` produces output in container
- [ ] `./adrscan --help` displays help information in container
- [ ] Error conditions produce visible error messages
- [ ] Proper exit codes returned for all execution paths

### Quality Requirements
- [ ] No performance regression in host execution
- [ ] Container image size increase <20%
- [ ] All existing functionality preserved
- [ ] Cross-platform compatibility maintained

### Documentation Requirements
- [ ] Container troubleshooting guide updated
- [ ] Deployment documentation includes debugging steps
- [ ] Error message meanings documented
- [ ] Known limitations clearly stated

---

## 🎯 Next Steps

1. **Start Diagnostic Investigation** using Phase 1-4 procedures
2. **Implement Signal Handling Fix** as immediate mitigation
3. **Test Progressive Solutions** with container environments
4. **Validate Comprehensive Fix** across multiple platforms
5. **Update Documentation** with troubleshooting guidance

**Expected Resolution Time**: 1-2 weeks  
**Priority Level**: Critical (blocking production deployments)  
**Assigned To**: TBD  
**Labels**: `critical`, `container`, `runtime`, `bug`

**This issue blocks production container deployments and must be resolved before v0.3.0-beta can be considered production-ready.**