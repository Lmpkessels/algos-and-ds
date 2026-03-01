#include <stdio.h>
#include <assert.h>
#include "../src/binary_search.h"

void test_binary_search_found() {
    int array[5] = {
        1, 2, 3, 4, 5
    };

    size_t size = sizeof(array) / sizeof(array[0]);

    int index = binary_search(array, size, 4);

    assert(index == 4);
}

void test_binary_search_not_found() {
    int array[5] = {
        0, 1, 2, 3, 4
    };

    size_t size = sizeof(array) / sizeof(array[0]);

    int index = binary_search(array, size, 5);

    assert(index == -1);
}

int main(void) {
    test_binary_search_found();
    test_binary_search_not_found();
}