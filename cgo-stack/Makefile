
GO=go

all:
	#apt-cache policy musl-tools

	# All together, static first. Works
	CC=musl-gcc go build --ldflags '-extldflags "-static -Wl,-z,stack-size=6291456"' -x

	# All together, static second. Works.
	# CC=musl-gcc go build --ldflags '-extldflags "-Wl,-z,stack-size=6291456 -static"' -x

	# Split at second level, static first. Only stack size is affected.
	# CC=musl-gcc go build --ldflags '-extldflags "-static" -extldflags "-Wl,-z,stack-size=6291456"' -x

	# Split at second level, static second. Only static is enabled.
	# CC=musl-gcc go build --ldflags '-extldflags "-Wl,-z,stack-size=6291456" -extldflags "-static"' -x

	# Split at first level, static first. Only stack size is affected.
	# CC=musl-gcc go build --ldflags '-extldflags "-static"' --ldflags '-extldflags "-Wl,-z,stack-size=6291456"' -x

	# Split at first level, static second. Only static is enabled.
	# CC=musl-gcc go build --ldflags '-extldflags "-Wl,-z,stack-size=6291456"' --ldflags '-extldflags "-static"' -x

	#CC=musl-gcc go build --ldflags '-extldflags "-static" -extldflags "-Wl,-z,stack-size=6291456" ' -x

.PHONY: all
