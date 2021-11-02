file target/debug/asmgen 
tui enable
layout split
break main
break src/parser.rs:474
run progs/stage6-nested.bas
record
