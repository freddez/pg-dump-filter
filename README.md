pg-dump-filter
==============

`pg-dump-filter` is a tool that filters tables from a regular
PostgreSQL dump file. It could be used to reload some tables from a
dump in living instance, or to ignore others while creating a test
environment.

Install
-------

This tool is written in [Rust language](https://www.rust-lang.org/). 
Install it, then run:

  cargo install

Usage
-----

    Usage: pg-dump-filter [options]

    Options:
        -t, --table NAME    output only tables matching NAME
        -T, --exclude-table NAME
                            no not output tables matching NAME
        -c, --copy-only     output only COPY FROM statements
        -r, --truncate      empty tables before COPY FROM statements
        -h, --help          print this help menu

Examples
--------

To reload *pool* and *customer* tables datas to an existing database :

    pg-dump-filter -cr -t "pool|customer" < dump.sql | psql mydatabase
    
To load a complete dump excluding *log* datas :

    pg-dump-filter -T log < dump.sql | psql mydatabase

Important notice
----------------

Tables arguments folows the [regex rust library
syntax](https://docs.rs/regex/0.2.2/regex/#syntax). For example the
argument:

    -t "pool|customer"

matches all `*pool*` and `*customer*` table names. If you want an
exact match, you have to type:

    -t "^pool$|^customer$"`
    
Be warned.

I've written this tool for personal needs. If you find it useful,
don't hesitate to send me some remarks for improvements requests.
