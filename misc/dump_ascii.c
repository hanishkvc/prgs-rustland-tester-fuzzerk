/*
 * Generate the printable ascii set as hex values
 * HanishKVC, 2022
 */

#include <stdio.h>
#include <ctype.h>
#include <stdbool.h>
#include <string.h>

int main(int argc, char **argv) {
    for (int i = 0; i < 256; i++) {
        bool bDoPrint = false;
        if (strncmp("gen_printable_ascii", argv[0], 19) == 0) {
            if isprint(i) {
                bDoPrint=true;
            }
        } else {
            bDoPrint=true;
        }
        if (bDoPrint) {
            printf("%c, %d, 0x%x\n", i, i, i);
        }
    }
}

// vim: set tabstop=4 softtabstop=4 shiftwidth=4 expandtab:
