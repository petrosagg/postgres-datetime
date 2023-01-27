cmake_minimum_required (VERSION 3.9.0)
project (PGDATETIME)
include_directories(include)
add_library (Pgdatetime src/datetime.c)
