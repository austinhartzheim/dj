# Dj

A small set of shortcuts coded in Rust.

This respository is largely experimental code for trying out the amazing [Rust](http://www.rust-lang.org/) programming language.

## Usage

Simply run the following:
```bash
dj [SHORTCUT NAME]
```
where the shortcut names are defined as follows:

* test: run `python3 manage.py test` to execute Django's unit test engine.
* coverage: run `coverage run manage.py test` followed by `coverage report` to display the coverage report.
