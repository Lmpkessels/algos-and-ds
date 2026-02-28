#include <stdio.h>

/*
Binary search starts in middle and reduces search by updating the middle
to a increment in beginning or a decrement in end, then adding up to get
a whole and deviding it by two till the target it's index is found.

TODO: Add a cleaner documentation.
*/
int binary_search(int array[], size_t size, int target) {
    // size_t is used for array counting and indexing.
    size_t beginning = 0;
    size_t end = size - 1;

    while (beginning <= end) {
        // Prevent overflow through (end - beginning).
        size_t middle = beginning + (end - beginning) / 2;

        if (array[middle] == target) {
            return array[middle];
        };

        if (array[middle] < target) {
            beginning = middle + 1;
        } else {
            if (middle == 0) // Prevent underflow.
                break;
            end = middle - 1;
        }
    }

    return -1;
} 