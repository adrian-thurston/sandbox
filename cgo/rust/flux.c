#include <stdio.h>

void flux()
{
	/* The challenge: increment this number and look for the update when running
	 * $ ./cgo
	 *
	 * Is there any way to do this without the 'uniq' hack in the Makefile? /
	 *
	 * Already tried: make it into a shared library and link with that. Go will
	 * still link it statically and use the same rebuilding rules.
	 */
	printf( "flux() inside archive 15\n");
}
