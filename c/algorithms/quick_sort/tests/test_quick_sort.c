#include <stdio.h>
#include <assert.h>
#include "../src/quick_sort.h"

void test_quick_sort_ordering_list_with_repeating_numbers() {
    int array[10] = {
        0, 3, 1, 8, 1, 9, 9, 1, 1, 0
    };

    int expected[10] = {
        0, 0, 1, 1, 1, 1, 3, 8, 9, 9
    };

    int size = sizeof(array) / sizeof(array[0]);

    int ordered_list = quick_sort(array, 0, 10);

    for (int i = 0; i < size; i++) {
        assert(array[i] == expected[i]);
    }
}