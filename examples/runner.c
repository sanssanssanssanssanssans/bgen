#include <stdio.h>
#include <stdlib.h>
#define MEMORY_SIZE 30000

void run(const char* code) {
    unsigned char tape[MEMORY_SIZE] = {0};
    unsigned char* ptr = tape;
    const char* pc = code;

    while (*pc) {
        switch (*pc) {
            case '>':
                ptr++;
                break;
            case '<':
                ptr--;
                break;
            case '+':
                (*ptr)++;
                break;
            case '-':
                (*ptr)--;
                break;
            case '.':
                putchar(*ptr);
                break;
            case ',':
                *ptr = getchar();
                break;
            case '[':
                if (*ptr == 0) {
                    int loop = 1;
                    while (loop > 0) {
                        pc++;
                        if (*pc == '[') loop++;
                        if (*pc == ']') loop--;
                    }
                }
                break;
            case ']':
                if (*ptr != 0) {
                    int loop = 1;
                    while (loop > 0) {
                        pc--;
                        if (*pc == '[') loop--;
                        if (*pc == ']') loop++;
                    }
                }
                break;
        }
        pc++;
    }
}

int main() {
    FILE* file = fopen("examples/nice.bf", "r");
    fseek(file, 0, SEEK_END);
    long size = ftell(file);
    fseek(file, 0, SEEK_SET);
    char* buffer = malloc(size + 1);
    fread(buffer, 1, size, file);
    buffer[size] = '\0';
    run(buffer);
    free(buffer);
    fclose(file);
}