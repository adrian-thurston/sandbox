#include "wrapper.h"
#include <stdio.h>

extern void flux();

void flux_wrapper()
{
	flux();

	printf( "-DUNIQ=%s\n", UNIQ );
}



