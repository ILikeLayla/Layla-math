#include <stdio.h>


void print_vector(float *v, unsigned int lenth) {
    for (int i = 0; i < lenth; i++) {
        printf("%f ", v[i]);
    }
    printf("\n");
}

extern "C" {
    void print_vector_f32_unsafe(float *v, unsigned int lenth) {
        print_vector(v, lenth);
    }
}