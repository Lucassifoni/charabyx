defmodule Charabyx do
  @moduledoc """
  Wrapper around Meilisearch's Charabia Rust library.
  """

  @doc """
  Tokenizes input text.
  """
  def tokenize(input) do
    CharabyxNif.tokenize(input)
  end
end
