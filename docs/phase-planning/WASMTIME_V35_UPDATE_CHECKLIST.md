# wasmtime v23 → v35 Update Checklist

**Priority**: 🔴 **CRITICAL**  
**Risk Level**: HIGH - 12 major version jump  
**Estimated Effort**: 8-12 hours  
**Blocking**: Phase 3 WASM functionality  

## 📋 **Pre-Update Analysis** ✅

### **Version Analysis**
- [x] **Current version**: wasmtime 23.0.3
- [x] **Target version**: wasmtime 35.0.0  
- [x] **Version jump**: 12 major releases (23→24→...→35)
- [x] **Release timeline**: ~12 months of changes
- [x] **Breaking changes**: Extensive API modifications

### **Rust MSRV Impact**
- [x] **wasmtime v35 requirement**: Rust 1.86+
- [ ] **Current project MSRV**: Unknown - needs verification
- [ ] **Action required**: Verify/update project MSRV

### **Dependency Tree Analysis**
- [x] **Direct dependency**: `wasmtime = "23.0"`
- [x] **Feature usage**: `features = ["plugins"]`
- [x] **Code locations**: `src/plugins/manager.rs`, tests
- [x] **Optional dependency**: Only enabled with `plugins` feature

## 🔍 **Breaking Changes Assessment**

### **Major API Changes (v23→v35)**

#### **1. Component Model Integration** 🆕
- **Change**: Enhanced WebAssembly component model support
- **Impact**: New APIs available, existing APIs still supported
- **Action**: Optional - can adopt new features later

#### **2. Engine Configuration** ⚠️
- **Change**: Enhanced `Config` API with new options
- **Impact**: Current `Engine::default()` still works
- **Action**: Review for new security/performance options

#### **3. WASI Changes** ⚠️
- **Change**: WASIp2 support, preview1 improvements
- **Impact**: Better WASI compatibility
- **Action**: Test existing WASI functionality

#### **4. Memory Management** 🔧
- **Change**: Improved GC support, better bounds checking
- **Impact**: Performance improvements, API compatible
- **Action**: Validate existing WASM modules work

#### **5. Security Enhancements** 🔒
- **Change**: CVE-2025-53901 fix, better sandboxing
- **Impact**: More secure WASM execution
- **Action**: Verify security improvements work

## 🧪 **Testing Strategy**

### **Phase 1: Basic Compatibility** 
- [ ] **1.1**: Update Cargo.toml version
- [ ] **1.2**: Attempt basic compilation
- [ ] **1.3**: Fix any compilation errors
- [ ] **1.4**: Run existing unit tests

### **Phase 2: Functionality Validation**
- [ ] **2.1**: Test WASM module loading
- [ ] **2.2**: Verify plugin execution
- [ ] **2.3**: Check memory usage patterns
- [ ] **2.4**: Validate security boundaries

### **Phase 3: Performance Validation**
- [ ] **3.1**: Benchmark WASM execution speed
- [ ] **3.2**: Measure memory consumption
- [ ] **3.3**: Test startup time impact
- [ ] **3.4**: Compare with baseline metrics

### **Phase 4: Integration Testing**
- [ ] **4.1**: End-to-end plugin functionality
- [ ] **4.2**: Browser WASM compatibility
- [ ] **4.3**: Cross-platform testing
- [ ] **4.4**: Container build validation

## 🔧 **Implementation Steps**

### **Step 1: Backup and Preparation**
```bash
# Create feature branch
git checkout -b feature/wasmtime-v35-update

# Backup current state
cp Cargo.toml Cargo.toml.backup
cp Cargo.lock Cargo.lock.backup

# Document current functionality
cargo test --features=plugins --verbose > current-test-results.txt
```

### **Step 2: Update Dependency**
```toml
# In Cargo.toml, change:
# FROM:
wasmtime = { version = "23.0", optional = true }

# TO:
wasmtime = { version = "35.0", optional = true }
```

### **Step 3: Update Lock File**
```bash
# Update specific dependency
cargo update -p wasmtime

# Verify new version
cargo tree | grep wasmtime
```

### **Step 4: Code Compatibility Check**
```bash
# Attempt compilation
cargo check --features=plugins

# Fix any compilation errors
# (Document specific fixes needed)
```

### **Step 5: Test Validation**
```bash
# Run all tests
cargo test --features=plugins --verbose

# Run specific dependency validation tests
cargo test dependency_validation::test_wasmtime_api_compatibility

# Run performance benchmarks
cargo bench --features=plugins
```

## 🐛 **Expected Issues & Solutions**

### **Issue 1: Compilation Errors**
**Symptoms**: Import/API errors in `src/plugins/manager.rs`
**Solution**: Update import statements and API calls
```rust
// Potential changes needed:
// OLD (v23):
use wasmtime::{Engine, Module, Store};

// NEW (v35): 
use wasmtime::{Engine, Module, Store, Config};
// May need component model imports if using new features
```

### **Issue 2: Test Failures**
**Symptoms**: Unit tests fail with API changes
**Solution**: Update test code to use new APIs
**Rollback Plan**: Revert to v23 if unfixable

### **Issue 3: Performance Regression**
**Symptoms**: Slower WASM execution than baseline
**Solution**: Review new configuration options
**Acceptance**: Up to 5% regression acceptable

### **Issue 4: Memory Usage Increase**
**Symptoms**: Higher memory consumption
**Solution**: Tune new memory management options
**Monitoring**: Track memory usage in CI

## 📊 **Validation Criteria**

### **Must Pass (Blocking)**
- [ ] ✅ **Compilation**: Clean build with no errors
- [ ] ✅ **Unit Tests**: All existing tests pass
- [ ] ✅ **Security**: No new vulnerabilities
- [ ] ✅ **Basic WASM**: Module loading/execution works

### **Should Pass (Important)**
- [ ] 📈 **Performance**: <5% regression on benchmarks
- [ ] 🔧 **Integration**: End-to-end functionality works
- [ ] 🌐 **Cross-platform**: Works on Linux/Windows/macOS
- [ ] 📦 **Container**: Docker builds successfully

### **Nice to Have (Optional)**
- [ ] 🚀 **Performance**: Improvements from new optimizations
- [ ] 🆕 **Features**: Adoption of new component model features
- [ ] 🔒 **Security**: Enhanced security benefits
- [ ] 📚 **Documentation**: Update documentation for new features

## 🚨 **Rollback Plan**

### **Immediate Rollback (if critical issues)**
```bash
# Restore backups
mv Cargo.toml.backup Cargo.toml
mv Cargo.lock.backup Cargo.lock

# Update dependencies
cargo update

# Verify restoration
cargo test --features=plugins
```

### **Partial Rollback (if performance issues)**
```bash
# Pin to specific v34 or earlier if v35 has issues
wasmtime = { version = "34.0", optional = true }
```

### **Rollback Triggers**
- **Security vulnerabilities** introduced
- **>10% performance regression** 
- **Breaking functionality** that can't be fixed in 4 hours
- **Cross-platform compatibility** issues

## 📈 **Success Metrics**

### **Technical Metrics**
- ✅ **Zero compilation errors**
- ✅ **100% test pass rate**
- ✅ **<5% performance impact**
- ✅ **No new security issues**

### **Integration Metrics**  
- ✅ **WASM modules load successfully**
- ✅ **Plugin functionality intact**
- ✅ **Container builds pass**
- ✅ **Cross-platform compatibility maintained**

### **Quality Metrics**
- ✅ **Documentation updated**
- ✅ **Migration notes created** 
- ✅ **Performance baseline updated**
- ✅ **Security audit clean**

## 📝 **Post-Update Tasks**

### **Documentation**
- [ ] Update CHANGELOG.md with wasmtime upgrade
- [ ] Document any API changes made
- [ ] Update performance baselines
- [ ] Note any new features available

### **Monitoring**
- [ ] Monitor CI for 1 week for any issues
- [ ] Track performance metrics
- [ ] Watch for community feedback
- [ ] Monitor for security advisories

### **Future Opportunities**
- [ ] Explore new component model features
- [ ] Optimize configuration for performance
- [ ] Adopt new security enhancements
- [ ] Plan next major update process

## ⏱️ **Timeline**

**Estimated Duration**: 8-12 hours over 2-3 days

### **Day 1: Analysis & Update (4-6 hours)**
- Hours 1-2: Backup, analysis, dependency update
- Hours 3-4: Compilation fixes and basic testing
- Hours 5-6: Initial validation and documentation

### **Day 2: Testing & Validation (3-4 hours)**
- Hours 1-2: Comprehensive testing and performance validation
- Hours 3-4: Integration testing and final validation

### **Day 3: Integration & Monitoring (1-2 hours)**
- Hour 1: PR creation and review
- Hour 2: Monitor initial CI results

---

**✅ Update Complete When**: All checkboxes checked, PR merged, monitoring stable for 24h