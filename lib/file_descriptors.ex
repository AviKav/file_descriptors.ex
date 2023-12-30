defmodule FileDescriptors do
  use Rustler, otp_app: :file_descriptors, crate: "filedescriptors"

  defstruct [:path, :fd, :_file_ref]

  @type t :: %__MODULE__{
          path: String.t(),
          fd: integer(),
          _file_ref: reference()
        }

  @doc false
  @spec _open(String.t()) :: %{fd: integer(), file: reference()}
  def _open(_path), do: :erlang.nif_error(:nif_not_loaded)

  @spec open(String.t()) :: t()
  def open(path) do
    %{fd: fd, file: file} = _open(path)
    %__MODULE__{path: "/dev/fd/#{fd}", fd: fd, _file_ref: file}
  end
end
