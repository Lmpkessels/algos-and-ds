#include <stddef.h>
#ifndef METRICS_H
#define METRICS_H

// Structure to track metrics of an algorithm.
typedef struct {
    size_t comparisons;
    size_t swaps;
    size_t allocations;
    size_t recursion_depth;
    size_t max_recursion_depth;
} metrics_t;

extern metrics_t METRICS;

// Functions that are used to track matrics.
void metrics_reset(void);
void metrics_enter_recursion(void);
void metrics_exit_recursion(void);

#endif