#include "test.h"
void _start() {
  char *p = malloc(8);
  memcpy(p, "12334",6);
  isize x = atoi(p);
  exit(69);
}