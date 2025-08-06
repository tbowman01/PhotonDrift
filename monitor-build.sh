#!/bin/bash

# Build Validation Monitor Script
# Continuously tracks compilation progress

echo "🔄 Starting Continuous Build Validation Monitor"
echo "🎯 Target: 0 errors | Current baseline: 2 errors"
echo "📋 Primary Issue: Missing rand dependency"
echo ""

ITERATION=0
PREVIOUS_ERRORS=2

while true; do
    ITERATION=$((ITERATION + 1))
    CURRENT_TIME=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    
    # Run cargo check and count errors
    ERROR_COUNT=$(cargo check --message-format=short 2>&1 | grep -c "error" || echo "0")
    WARNING_COUNT=$(cargo check --message-format=short 2>&1 | grep -c "warning" || echo "0")
    
    echo "🔍 Iteration $ITERATION - $CURRENT_TIME"
    echo "   Errors: $ERROR_COUNT | Warnings: $WARNING_COUNT"
    
    # Check if errors decreased
    if [ "$ERROR_COUNT" -lt "$PREVIOUS_ERRORS" ]; then
        echo "   ✅ PROGRESS: Errors reduced from $PREVIOUS_ERRORS to $ERROR_COUNT"
        npx claude-flow@alpha hooks notify --message "BUILD PROGRESS: Errors reduced from $PREVIOUS_ERRORS to $ERROR_COUNT" --level "success" 2>/dev/null || true
        PREVIOUS_ERRORS=$ERROR_COUNT
    elif [ "$ERROR_COUNT" -eq 0 ]; then
        echo "   🎉 SUCCESS: All compilation errors resolved!"
        echo "   🧪 Running comprehensive tests..."
        
        # Run full test suite
        if cargo test --all-features; then
            echo "   ✅ ALL TESTS PASSED!"
            npx claude-flow@alpha hooks notify --message "VALIDATION COMPLETE: All errors resolved, tests passing" --level "success" 2>/dev/null || true
        else
            echo "   ⚠️  Tests failed, but compilation successful"
            npx claude-flow@alpha hooks notify --message "Compilation fixed but tests failing" --level "warning" 2>/dev/null || true
        fi
        break
    elif [ "$ERROR_COUNT" -gt "$PREVIOUS_ERRORS" ]; then
        echo "   ⚠️  WARNING: Errors increased from $PREVIOUS_ERRORS to $ERROR_COUNT"
        npx claude-flow@alpha hooks notify --message "REGRESSION: Errors increased to $ERROR_COUNT" --level "warning" 2>/dev/null || true
        PREVIOUS_ERRORS=$ERROR_COUNT
    else
        echo "   ⏸️  Waiting for fixes..."
    fi
    
    # Brief pause between checks
    sleep 5
done

echo ""
echo "🏁 Build validation monitoring completed!"