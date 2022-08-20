/*
 * Dump full or printable ascii set as hex values
 * HanishKVC, 2022
 */

#include <stdio.h>
#include <ctype.h>
#include <stdbool.h>
#include <string.h>
#include <libgen.h>

int main(int argc, char **argv) {
    char arg0[1024];
    strncpy(arg0, argv[0], 1023);
    arg0[1023] = '\x0';
    char prgName[1024];
    strncpy(prgName, basename(arg0), 1023);
    prgName[1023] = '\x0';
    for (int i = 0; i < 256; i++) {
        bool bDoPrint = false;
        if (strncmp("dump_ascii_printable", prgName, 19) == 0) {
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
