#include "test.h" // this defines my own version of memcpy

/*int test_strcmp_strlen_memcpy(){
    char* c = "bleh";
    usize len = strlen(c) + 1;
    if (len != 5){
        return -1;
    }
    char* f = malloc(5);
    memcpy(f, c, len);
    printf("test_strcmp_strlen_memcpy:\nc: %s, f:%s \nstring length: %lu\n\n",c,f,len);
    if (len != 5 || strcmp("xz","utils") == ('x' - 'b') || strcmp(c, f) != 0){
        return -1;
    }
    return 0;
}*/

void _start(){
    pstr("henlo!\n", 7);
    exit(69);
}