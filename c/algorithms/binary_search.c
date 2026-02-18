#include <stdio.h>

int binary_search(int array[], int size, int target) {
    int beginning = 0;
    int end = size - 1;

    while (beginning <= end) {
        int middle = beginning + (end - beginning) / 2;

        if (array[middle] == target) {
            return array[middle];
        };

        if (array[middle] < target) {
            beginning = middle + 1;
        } else {
            end = middle - 1;
        }
    }

    return -1;
}