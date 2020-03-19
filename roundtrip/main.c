
#include "rust.h"

int main()
{
	hello_from_rust();

	struct rust_struct *rs = alloc_struct();
	free_struct( rs );
}
