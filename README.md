# Lorsum

## Lorsum file format

A lorusm exists out of sections started with a `_` followed by the section name.
All section names are lowercase
and must respect the order described in this document.

### Types

A list of all the word types in the file seperated by a newline.

### Templates

A list of lines with `{ ident }` in place of the types.
All idents must be previously specified in the types section.

### Lists

A list of lists per type each type starts with `:type` followed by a list
of words sperated by whitespace until the next `:type`.
All types specified in the types section must be provided at least one item.
