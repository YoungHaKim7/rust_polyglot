#include "csapp.h"
#include <cstddef>
#include <cstdio>
#include <pthread.h>
#include <stdio.h>
#include <unistd.h>

void *sum_mutex(void *vargp);

long gsum = 0;
long nelems_per_thread;
sem_t mutex;

int main(int argc, char **argv) {
  long i, nelems, log_nelems, nthreads, myid[MAXTHREADS];
  pthread_t tid[MAXTHREADS];

  nthreads = atoi(argv[1]);
  log_nelems = atoi(argv[2]);
  nelems = (1L << log_nelems);
  nelems_per_thread = nelems / nthreads;
  sem_init(&mutex, 0, 1);

  for (i = 0; i < nthreads; i++) {
    myid[i] = i;
    Pthread_create(&tid[i], NULL, sum_mutex, &myid[i]);
  }
  for (i = 0; i < nthreads; i++) {
    Pthread_join(tid[i], NULL);
  }

  if (gsum != (nelems * (nelems - 1)) / 2)
    printf("Error: result=%ld\n", gsum);

  exit(0);
}

void *sum_mutex(void *vargp) {
  long myid = *((long *)vargp);
  long start = myid * nelems_per_thread;
  long end = start + nelems_per_thread;
  long i;

  for (i = start; i < end; i++) {
    P(&mutex);
    gsum += i;
    V(&mutex);
  }
  return NULL;
}
