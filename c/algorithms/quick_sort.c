#include <stdio.h>

void swap_indices(int* a, int* b) {
    int temporary = *a;
    *a = *b;
    *b = temporary;
}

int partition(int arr[], int low, int high) {
    int pivot = arr[low];
    int i = low;
    int j = high;

    while (i < j) {
        while (i <= high - 1 && arr[i] <= pivot) {
            i++;
        }

        while (j >= low + 1 && arr[j] > pivot) {
            j--;
        }

        if (i < j) {
            swap_indices(&arr[i], &arr[j]);
        }
    }

    swap_indices(&arr[low], &arr[j]);
    return j;
}

void quick_sort(int arr[], int low, int high) {
    if (low < high) {
        int pi = partition(arr, low, high);

        quick_sort(arr, low, pi - 1);
        quick_sort(arr, pi + 1, high);
    }
}

int main(void) {
    int arr[10] = {36, 34, 43, 11, 15, 20, 28, 45, 27, 32};

    int low = 0;
    int high = (sizeof(arr) / sizeof(arr[0])) - 1;

    quick_sort(arr, low, high);

    for (int i = 0; i < 10; i++) {
        printf("Sorted: %d\n", arr[i]);
    }

    return 0;
 }