.PHONY: run clear up

run:
	cargo build
	LUA_CPATH=target/debug/lib?.so tarantool init.lua

clear:
	rm -f *.snap *.xlog

up: clear run

