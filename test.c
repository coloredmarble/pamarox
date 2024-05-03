#include "test.h"
void _start(){
    char* p = malloc(8);
    *p =  'p';
    pstr_til_nil(p);
    // check same loc
    char* n = malloc(8);
    *n =  'n';
    if (p == n){
       test_panic();
    };
    pstr_til_nil(n);
    free(p);
    // check free 
    char *y = malloc(4);
    if (y != p){
        test_panic();
    };
    usize* x = malloc(8);
    if (*x != 0){
        test_panic();
    } 
    exit(69);
 }