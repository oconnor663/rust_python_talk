#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

uint64_t fib(uint64_t x) {
  if (x <= 1) {
    return 1;
  }
  return fib(x - 1) + fib(x - 2);
}

void main(int argc, char **argv) {
  uint64_t x = strtoull(argv[1], NULL, 10);
  printf("%" PRIu64 "\n", fib(x));
}
