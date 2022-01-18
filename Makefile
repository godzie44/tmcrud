hello:
	echo "hello world"

run:
	cargo build
	LUA_CPATH=target/debug/lib?.so tarantool init.lua
