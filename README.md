# mdbook-newday

## Motivation

With a lot of projects I like to keep a log or journal.
Mdbook is a nice way to do that, but I find that actually
entering in the day and date and creating a new file can be
error prone and tedious.

## What mdbook-newday does

mdbook-newday is a very special purpose command that will take
a `SUMMARY.md`file and add a line to it for the current day.

The format of that line is

`- [%A, %b %d, %Y](./%Y/%Y-%m/%Y-%m-%d.md)`, ie

`- [Thursday, Jan 01, 1970](./1970/1970-01/1970-01-01.md)`,

If you then run mdbook serve, it will create a file at `./1970/1970-01/1970-01-01.md` .

The file will be automatically given a title in the form `# Thursday, Jan 01, 1970` .
