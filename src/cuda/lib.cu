#include <stdio.h>

void hello_cuda() {
    printf("Hello CUDA!\n");
}

extern "C" {
    void hello_cuda_unsafe() {
        hello_cuda();
    }
}