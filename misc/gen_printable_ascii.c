/*
 * Generate the printable ascii set as hex values
 * HanishKVC, 2022
 */

#include <stdio.h>
#include <ctype.h>

int main(int argc, char **argv) {
    for (int i = 0; i < 256; i++) {
        if isprint(i) {
            printf("%c, %d, 0x%x\n", i, i, i);
        }
    }
}
