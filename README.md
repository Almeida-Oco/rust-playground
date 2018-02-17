First major project using [Rust](https://www.rust-lang.org/en-US/index.html).

Trying to create an easier way to rename files in a system using regex.
(It is highly likely that such features are already provided by some other program)

# Status

* **Renaming is now possible**
* Pattern matching to names.
* Currently possible wild characters are '*' and '.'
* Restriction characters are '$' and '^'
* It is also possible escape characters using '\\'. Example: '\\*'
* Character sets '[<set>]' are available, though escaping characters inside them might not fully work
* Regex symbols are now assigned the respective matching text, to easily rename the files.
* Conversion from matching regex to target regex is now done.

# Symbols

### Asterisk (*)
The asterisk matches 0 or more of any characters.
It must be followed by its ID.

##### Examples
Lets assume a directory contains the following files:

    foo[remove]bar  [remove]bar  bar[remove]foo  f[remove]b  M[remove] foo_bar

And say that we wanted to remove the '[remove]' bit of every file that contains it.
We would then run the following command:
    
    rn '*0\[remove]*1' '*0*1'

(We have to enclose the regular expressions in ' so that the shell does not interpret them).

This would rename the files that match the first regular expression, meaning the directory would contain the following files:

    foobar  bar  barfoo  fb  M  foo_bar 
