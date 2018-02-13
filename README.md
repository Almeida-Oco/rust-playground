First major project using [Rust](https://www.rust-lang.org/en-US/index.html).

Trying to create an easier way to rename files in a system using regex.
(It is highly likely that such features are already provided by some other program)

# Status

* Pattern matching to names should now be possible.
* Currently possible wild characters are '*' and '.'
* Restriction characters are '$' and '^'
* It is also possible escape characters using '\\'. Example: '\*'
* Character sets '[<set>]' are available, though escaping characters inside them might not fully work
