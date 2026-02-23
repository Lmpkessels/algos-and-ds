#include "metrics.h"

metrics_t METRICS = {0};

// Set all metrics back to default.
void metrics_reset(void) {
    METRICS.comparisons = 0;
    METRICS.swaps = 0;
    METRICS.allocations = 0;
    METRICS.recursion_depth = 0;
    METRICS.max_recursion_depth = 0;
}

// Track recursion depth till max.
void metrics_recursion_depth(void) {
    METRICS.recursion_depth++;
    if (METRICS.recursion_depth > METRICS.max_recursion_depth) {
        METRICS.max_recursion_depth = METRICS.recursion_depth;
    }
}

// End tracking recursion depth.
void metrics_exit_recursion(void) {
    if (METRICS.recursion_depth > 0) {
        METRICS.recursion_depth--;
    }
}