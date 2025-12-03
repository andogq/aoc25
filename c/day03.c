#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct Bank {
    uint8_t len;
    uint8_t buf[128];
};


struct BankSlice {
    uint8_t len;
    uint8_t const* start;
    uint8_t const* end;
};

struct Bank parse_bank(char* buf, int max_len) {
    struct Bank bank = { .len = 0, .buf = {} };

    for (int i = 0; i < max_len; i++) {
        bank.buf[i] = buf[i] - '0';
        bank.len += 1;
    }

    return bank;
}

struct BankSlice bank_slice(struct Bank const* bank, uint8_t start, uint8_t end) {
    struct BankSlice slice = { .len = 0, .start = NULL, .end = NULL };

    if (start >= bank->len || end > bank->len) {
        printf("slice must be within bounds\n");
        return slice;
    }

    if (start > end) {
        printf("start must be before end\n");
        return slice;
    }

    slice.len = end - start;
    slice.start = &bank->buf[start];
    slice.end = &bank->buf[end];

    return slice;
}

uint64_t ipow(uint64_t base, uint64_t exp) {
    uint64_t result = 1;

    for (int i = 0; i < exp; i++) {
        result *= base;
    }

    return result;
}

uint64_t largest(struct BankSlice const* bank, uint32_t remaining) {
    if (bank->len < remaining || remaining == 0) {
        return 0;
    }

    uint8_t const* max = bank->start;
    uint8_t max_i = 0;
    for (int i = 0; i < bank->len - remaining + 1; i++) {
        uint8_t const* ptr = bank->start + i;

        if (ptr >= bank->end) {
            break;
        }

        if (*ptr > *max) {
            max = ptr;
            max_i = i;
        }
    }

    struct BankSlice sub = {
        .len = bank->len - max_i - 1,
        .start = bank->start + max_i + 1,
        .end = bank->end,
    };

    uint64_t lhs = ((uint64_t) *max * ipow(10, remaining - 1)) + largest(&sub, remaining - 1);
    uint64_t rhs = largest(&sub, remaining);

    if (lhs > rhs) {
        return lhs;
    } else {
        return rhs;
    }
}

int read_line(FILE* file, char* buf, int max_len) {
    char c;
    int read_bytes = 0;

    while ((c = fgetc(file)) != EOF) {
        if (c == '\n') {
            break;
        }

        if (read_bytes >= max_len) {
            printf("exceeded provided buffer");
            return -1;
        }

        buf[read_bytes] = c;
        read_bytes += 1;
    }

    return read_bytes;
}

int main(int argc, char *argv[]) {
    char* kind = "examples";

    if (argc == 2) {
        kind = argv[1];
    }

    char* path;
    if (strcmp(kind, "examples") == 0) {
        path = "../data/examples/03.txt";
    } else if (strcmp(kind, "input") == 0) {
        path = "../data/inputs/03.txt";
    } else {
        printf("unknown kind: %s\n", kind);
        return 1;
    }

    FILE* handle = fopen(path, "r");
    if (handle == NULL) {
        perror("unable to open file");
        return 1;
    }

    uint64_t power_pt1 = 0;
    uint64_t power_pt2 = 0;

    char line[128];
    int length;
    while ((length = read_line(handle, line, 128)) > 0) {
        struct Bank bank = parse_bank(line, length);
        struct BankSlice slice = bank_slice(&bank, 0, bank.len);

        power_pt1 += largest(&slice, 2);
        power_pt2 += largest(&slice, 12);
    }

    printf("part 1: %llu\n", power_pt1);
    printf("part 2: %llu\n", power_pt2);

    if (fclose(handle)) {
        perror("error closing file");
        return 1;
    }

    return 0;
}
