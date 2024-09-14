#include <stdio.h>

void bar() {
}

void foo() {
    bar();
}
int main() {
    printf("Hello, world!\n");
    foo();
    return 0;
}
