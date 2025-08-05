# Upgrade Rust to 1.79+ and Update wasmtime to v35+

## 🎯 Issue Summary

**Priority:** High  
**Type:** enhancement  
**Component:** core, build  
**Phase:** Phase 3 - Developer Experience  

PhotonDrift currently uses Rust 1.75.0, which is blocking the upgrade to wasmtime v35.0+ due to edition2024 feature requirements. This issue tracks the systematic upgrade to a modern Rust toolchain.

## 🔍 Current State

### Current Versions
- **Rust:** 1.75.0 (82e1608df 2023-12-21)
- **Cargo:** 1.75.0 (1d8b05cdd 2023-11-20)
- **wasmtime:** 25.0 (downgraded from 35.0 due to compatibility)

### Current Blockers
```
error: feature `edition2024` is required

The package requires the Cargo feature called `edition2024`, but that feature is not stabilized in this version of Cargo (1.75.0).
Consider trying a newer version of Cargo (this may require the nightly release).
```

## 🎯 Target State

### Target Versions
- **Rust:** 1.79+ (stable edition2024 support)
- **Cargo:** 1.79+ (matching Rust version)
- **wasmtime:** 35.0+ (latest features and security updates)

### Benefits of Upgrade
- ✅ **Latest wasmtime features:** Performance improvements and security patches
- ✅ **Edition 2024 support:** Modern Rust language features
- ✅ **Better WASM performance:** Optimized compilation and runtime
- ✅ **Security updates:** Latest vulnerability patches
- ✅ **Future compatibility:** Alignment with ecosystem standards

## 📋 Implementation Plan

### Phase 1: Environment Preparation
- [ ] **Update Makefile Rust version**
  - Change `RUST_VERSION ?= 1.75` to `RUST_VERSION ?= 1.79`
  - Update setup targets to use new version
- [ ] **Update CI/CD workflows**
  - Update `.github/workflows/ci.yml` Rust version
  - Update `.github/workflows/container-build.yml` 
  - Update Docker base images to use Rust 1.79+
- [ ] **Update documentation**
  - Update README.md minimum Rust version
  - Update development setup instructions

### Phase 2: Dependency Updates
- [ ] **Upgrade wasmtime dependency**
  - Update `Cargo.toml`: `wasmtime = { version = "35.0", optional = true }`
  - Test WASM plugin functionality
  - Validate performance benchmarks
- [ ] **Test compatibility**
  - Run full test suite with new Rust version
  - Test all cargo features combinations
  - Validate WASM builds and functionality

### Phase 3: Validation & Documentation
- [ ] **Comprehensive testing**
  - All 178+ tests pass with new versions
  - ML models (26 tests) validate correctly
  - WASM builds work across platforms
  - Container builds succeed
- [ ] **Update documentation**
  - Development setup instructions
  - Contributor guidelines
  - Docker build guides
  - Minimum system requirements

## 🔧 Technical Considerations

### Compatibility Matrix
| Component | Current | Target | Impact |
|-----------|---------|--------|---------|
| Rust | 1.75.0 | 1.79+ | Breaking changes possible |
| wasmtime | 25.0 | 35.0+ | Major version jump |
| Edition | 2021 | 2024 | Language feature updates |

### Risk Assessment
- **Low Risk:** Rust 1.79 is stable and well-tested
- **Medium Risk:** wasmtime 35.0 may have API changes
- **Mitigation:** Gradual rollout with comprehensive testing

### Performance Impact
- **Expected improvement:** 5-15% WASM performance boost
- **Memory usage:** Potential reduction in WASM memory footprint
- **Compilation time:** May increase slightly due to more optimizations

## 🧪 Testing Strategy

### Pre-upgrade Testing
```bash
# Current state validation
make test-all
cargo test --all-features
make container-build
make wasm-build
```

### Post-upgrade Testing
```bash
# Full validation suite
make test-all
cargo test --all-features --release
make benchmark
make container-test
make security-scan
```

### Rollback Plan
If issues arise:
1. Revert Cargo.toml changes
2. Restore Rust version in Makefile/CI
3. Rebuild with previous versions
4. Document specific incompatibilities

## 💼 Business Impact

### Benefits
- **Security:** Latest Rust security patches
- **Performance:** Improved WASM runtime performance
- **Developer Experience:** Access to latest language features
- **Maintenance:** Reduced technical debt
- **Future-proofing:** Alignment with ecosystem direction

### Costs
- **Development time:** 4-8 hours for upgrade and testing
- **Risk:** Temporary instability during transition
- **Learning curve:** New language features (minimal)

## 📅 Timeline

### Week 1: Preparation
- [ ] Update CI/CD configurations
- [ ] Prepare testing environments
- [ ] Review wasmtime v35 changelog

### Week 2: Implementation
- [ ] Upgrade Rust toolchain
- [ ] Update wasmtime dependency
- [ ] Run comprehensive test suite

### Week 3: Validation
- [ ] Performance benchmarking
- [ ] Cross-platform testing
- [ ] Documentation updates

## 🔗 Related Issues

- **Dependencies:** None (can be done independently)
- **Blocks:** Future wasmtime security updates
- **Relates to:** WASM build pipeline improvements
- **Follows:** Current wasmtime downgrade (temporary fix)

## ✅ Acceptance Criteria

### Must Have
- [ ] Rust 1.79+ successfully installed and working
- [ ] wasmtime 35.0+ integrated and functional
- [ ] All existing tests pass (178+ Rust tests, 26 ML tests)
- [ ] CI/CD pipelines use new Rust version
- [ ] Documentation updated with new requirements

### Should Have
- [ ] Performance benchmarks show improvement or no regression
- [ ] WASM builds work on all target platforms
- [ ] Container builds succeed with new versions
- [ ] Development setup instructions updated

### Could Have
- [ ] Migration guide for other projects
- [ ] Performance comparison report
- [ ] New Rust 2024 edition features utilized

## 📊 Success Metrics

### Quantitative
- **Build time:** Should not increase >10%
- **Test pass rate:** 100% (no regressions)
- **WASM size:** Should not increase >5%
- **Performance:** WASM runtime should improve 5-15%

### Qualitative
- **Developer feedback:** Positive experience with new features
- **Stability:** No new crashes or instability
- **Security:** No new vulnerabilities introduced

---

**Estimated Effort:** 1-2 days  
**Priority:** High (blocks future wasmtime updates)  
**Risk Level:** Low-Medium  
**Assigned to:** TBD  

**Next Steps:**
1. Review and approve this upgrade plan
2. Schedule implementation window
3. Begin with CI/CD configuration updates
4. Execute systematic upgrade and testing

**Dependencies:**
- None (can proceed immediately)

**Impact on other work:**
- Blocks: Advanced WASM features requiring wasmtime 35+
- Enables: Latest Rust security updates and performance improvements