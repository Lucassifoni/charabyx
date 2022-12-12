defmodule CharabyxTest do
  use ExUnit.Case
  doctest Charabyx

  test "Tokenizes a simple string" do
    tokenization = Charabyx.tokenize("bonjour chers amis")
    assert(is_list(tokenization))
    assert(List.first(tokenization).lemma == "bonjour")
  end
end
