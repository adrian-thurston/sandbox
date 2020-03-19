
#include "rust.h"

int main()
{
	hello_from_rust_1();
	hello_from_rust_2();

	struct rust_struct *rs = alloc_struct_1();
	free_struct_2( rs );
}
