# rustypographe

Typography-checker for the French language. It enforces the following typographic rules:
* use of the character U+2026 (`…`) instead of `...` or `....` for ellipses;
* use of the U+00A0 non-breaking space character (` `) instead of a simple space ` ` or nothing before punctuations
marks that require it (`?`, `!`, `:`, `;`)

It performs these enforcements on UTF-8 strings and HTML code.
