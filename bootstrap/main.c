#include "flux.h"

int main()
{
	struct rust_struct *core;
	hello_from_core();
	hello_from_flux();

	core = alloc_struct_core();
	free_struct_core( core );
}
