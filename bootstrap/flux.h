#ifndef _FLUX_H
#define _FLUX_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

struct rust_struct;

void hello_from_core();
void hello_from_flux();

struct rust_struct *alloc_struct_core();

void free_struct_core( struct rust_struct * );

#ifdef __cplusplus
}
#endif

#endif
