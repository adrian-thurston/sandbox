#include "stacks.h"
#include <stdio.h>
#include <string.h>
#include <unistd.h>

void fn( void *base, int i, int depth )
{
	char arr[512 * 1024];
	memset(arr, 0, sizeof(arr));
	int j = 0;
	if ( i < depth ) {
		fn( base, i + 1, depth );
	}
	else {
		void *here = &j;
		char buf[32];
		sprintf( buf, "%ld\n", base - here );
		write( 1, buf, strlen( buf ) );
	}
}

void stacks()
{
	int i = 6;
	fn( &i, 0, 2 );
}
