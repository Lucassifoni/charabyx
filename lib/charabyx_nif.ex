defmodule CharabyxNif do
  use Rustler, otp_app: :charabyx, crate: :charabyx_nif

  def tokenize(_input), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
