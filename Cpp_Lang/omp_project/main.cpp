#include <cstdio>
#include <omp.h>
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  int nthread, tid;

#pragma omp parallel private(nthread, tid)
  {
    tid = omp_get_thread_num();
    printf("Welcom to GFG from thread = %d \n", tid);

    if (tid == 0) {
      nthread = omp_get_num_threads();
      printf("Number of threads = %d \n", nthread);
    }
  }
}
