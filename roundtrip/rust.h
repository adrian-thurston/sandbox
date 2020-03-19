#ifndef _RUST_H
#define _RUST_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

struct rust_struct;

void hello_from_rust_1();
struct rust_struct *alloc_struct_1();
void free_struct_1( struct rust_struct * );

void hello_from_rust_2();
struct rust_struct *alloc_struct_2();
void free_struct_2( struct rust_struct * );

#ifdef __cplusplus
}
#endif

#endif
