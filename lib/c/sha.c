#include <stdio.h>
#include <stdint.h>
#include <Windows.h>

uint32_t rol(const uint32_t value, uint32_t shift);
uint32_t f(int i, uint32_t b, uint32_t c, uint32_t d);
__declspec(dllexport) void __stdcall compute_buffer_values_c(const uint32_t* words, uint32_t* h);

void compute_buffer_values_c(
        const uint32_t* words,
        uint32_t* h
        ) {

    uint32_t k[4] = {
        0x5A827999,
        0x6ED9EBA1,
        0x8F1BBCDC,
        0xCA62C1D6
    };

    uint32_t tmp = 0;
    for (int i = 0; i < 80; i++) {
        tmp = rol(h[0], 5) + f(i, h[1], h[2], h[3]) + h[4] + words[i] + k[i/20];
        h[4] = h[3];
        h[3] = h[2];
        h[2] = rol(h[1], 30);
        h[1] = h[0];
        h[0] = tmp;
    }
}

uint32_t f(int i, uint32_t b, uint32_t c, uint32_t d) {
    if (i <= 19) {
        return ((b&c) | ((~b)&d));
    }
    if (i <= 39) {
        return (b^c^d);
    }
    if (i <= 59) {
        return ((b&c) | (b&d) | (c&d));
    }
    if (i <= 79) {
        return (b^c^d);
    };
    return 0;
}

uint32_t rol(const uint32_t value, uint32_t shift) {
    if ((shift &= sizeof(value)*8 - 1) == 0)
        return value;
    return (value << shift) | (value >> (sizeof(value)*8 - shift));
}
