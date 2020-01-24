#include <stdio.h>

void flux()
{
	/* The challenge: increment this number and look for the update when running
	 * $ ./cgo
	 *
	 * Is there any way to do this without the 'uniq' hack in the Makefile? /
	 */
	printf( "flux() inside archive 12\n");
}
