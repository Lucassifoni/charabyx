# Charabyx

WIP : Elixir NIFs for [Meilisearch's Charabia](https://github.com/meilisearch/charabia) crate.

Functions will be added as I need them.

```
iex> Charabyx.tokenize("bonjour chers amis !")
[
  %Charabyx.NifToken{
    kind: :word,
    lemma: "bonjour",
    script: :Latin,
    char_start: 0,
    char_end: 7,
    language: nil
  },
  %Charabyx.NifToken{
    kind: :separator_soft,
    lemma: " ",
    script: :Latin,
    char_start: 7,
    char_end: 8,
    language: nil
  },
  %Charabyx.NifToken{
    kind: :word,
    lemma: "chers",
    script: :Latin,
    char_start: 8,
    char_end: 13,
    language: nil
  },
  %Charabyx.NifToken{
    kind: :separator_soft,
    lemma: " ",
    script: :Latin,
    char_start: 13,
    char_end: 14,
    language: nil
  },
  ...
```