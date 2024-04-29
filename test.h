typedef unsigned long long usize; // this is my size
typedef long long isize;
typedef unsigned char u8;
typedef char i8 ;
void *memcpy(void* dst, const void* src, usize len);
void *memset(void *dst, u8 c, unsigned long);
char *strchr(const i8* str, i8 c);
char *strpbrk(const char* haystack, const char* needle);
int strcmp(const char* s1, const char* s2);
int strncmp(const char* s1, const char* s2, usize len);
isize atoi(const char* str);
int toascii(int);
int abs(int);
void exit(int);
char* uitoa(char* str, unsigned int x);
isize pstr(const char* s, usize size);
usize strlen(const char *str);