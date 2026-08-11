#include <stdio.h>
#include <stdlib.h>

int main() {

    printf("$ ");
    // flush after every printf call to avoid buffering , check the docs for more information 
    setbuf(stdout,NULL);
    
    return 0;
}