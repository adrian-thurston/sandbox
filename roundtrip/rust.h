#ifndef _RUST_H
#define _RUST_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

struct rust_struct;

void hello_from_rust();

struct rust_struct *alloc_struct();
void free_struct( struct rust_struct * );

#ifdef __cplusplus
}
#endif

#endif
