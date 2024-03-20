# rustypographe

Typography checker for the French language. It enforces the following typographic rules:
* use of the character U+2026 (`…`) instead of `....` or `...` for ellipses;
* use of the U+00A0 non-breaking space character (` `) instead of a simple space ` ` or nothing before punctuations
marks that require it (`?`, `!`, `:`, `;`);
* use of the characters U+00AB and U+00BB (`«` and `»`), along with non-breaking space characters, instead of `"` or
`''` for quotes;
* use of the U+2019 “apostrophe” (`’`) instead of `'`.

It performs these enforcements on UTF-8 strings and HTML code.
