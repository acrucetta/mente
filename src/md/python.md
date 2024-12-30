
# Python Tips

## Running Programs

- To run debug a program use `python3 -i foo.py`
- To run a program with a module open use `python3 -m module`

## Collections

- All collections are references to their memory location

## Modules

- Modules in python are imported from the toplevel
- If we want to import a file within the same module, we can't use `import foo` we need to import the module itself `import spam` or use `from . import foo`
- Every module needs a __init__.py
- __main__.py is used when the file gets called directly `python3 foo.py`
- If you want to merge modules, you can use the `__init__.py` file. Calling `from . import Foo` and `from . import Bar`, then you can import spam along with all its modules
  - You can use these later by just calling `spam.Foo()` or `spam.Bar()`
  - The split across submodules is hidden
- To add a path to be searched by python you can add it to the `syspath` using the sys or as an `env` variable via: `env PYTHONPATH=/file/path`
- Use `help()` to see module documentations

## Functions

- Functions are first class in python, they can be passed as objects to other functions and later used via `func(a)`
- You can use * for values e.g., *vals, or **dict for dictionaries
- You can store functions in dictionaries e.g., `d = {"add":add}`

## Exceptions

- Only catch exceptions you can handle, else let them propagate or log them and raise them 

## References:
- Python Mastery by David Beazley
- Fluent Python by Luciano Ramalho
