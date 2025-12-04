defmodule PaperMap do
  defstruct buf: [], height: 0, width: 0

  def create(buf, height, width) do
    %PaperMap{buf: buf, height: height, width: width}
  end

  def parse(input) do
    lines =
      input
      |> String.split("\n")

    height = lines |> length
    width = lines |> List.first() |> String.length()

    buf =
      lines
      |> Enum.flat_map(fn line ->
        line
        |> String.codepoints()
        |> Enum.map(fn c -> c == "@" end)
      end)

    %PaperMap{buf: buf, height: height, width: width}
  end

  def get(%PaperMap{buf: buf, height: height, width: width}, x, y) do
    buf
    |> Enum.at(offset(%PaperMap{height: height, width: width}, x, y))
  end

  def offset(%PaperMap{height: height, width: width}, x, y)
      when x < width and y < height do
    y * width + x
  end

  def offset() do
    nil
  end

  def iter(%PaperMap{buf: buf, width: width}) do
    buf
    |> Enum.zip(Stream.iterate(0, &(&1 + 1)))
    |> Enum.map(fn {b, i} -> {{rem(i, width), div(i, width)}, b} end)
  end

  def accessible(%PaperMap{buf: buf, width: width, height: height}, threshold) do
    iter(%PaperMap{buf: buf, width: width})
    |> Enum.map(fn {{x, y}, paper} ->
      if paper do
        Range.new(-1, 1)
        |> Enum.flat_map(fn dx ->
          Range.new(-1, 1)
          |> Enum.map(fn dy -> {dx, dy} end)
        end)
        |> Enum.filter(fn {dx, dy} -> !(dx == 0 and dy == 0) end)
        |> Enum.map(fn {dx, dy} -> {x + dx, y + dy} end)
        |> Enum.filter(fn {x, y} -> x >= 0 and y >= 0 and x < width and y < height end)
        |> Enum.filter(fn {x, y} ->
          get(%PaperMap{buf: buf, width: width, height: height}, x, y)
        end)
        |> Enum.count() < threshold
      else
        false
      end
    end)
  end
end

defmodule Day04 do
  def load_file() do
    File.read!("../data/inputs/04.txt")
    |> PaperMap.parse()
  end
end

IO.puts("Part 1:")

Day04.load_file()
|> PaperMap.accessible(4)
|> Enum.filter(& &1)
|> Enum.count()
|> IO.inspect()

IO.puts("Part 2:")

Day04.load_file()
|> Stream.iterate(fn map ->
  PaperMap.create(
    map
    |> PaperMap.accessible(4)
    |> Enum.zip(map |> PaperMap.iter())
    |> Enum.map(fn {accessible, {_pos, paper}} ->
      paper and !accessible
    end),
    map.height,
    map.width
  )
end)
|> Stream.map(fn map -> map |> PaperMap.accessible(4) |> Enum.filter(& &1) |> Enum.count() end)
|> Stream.take_while(&(&1 > 0))
|> Enum.to_list()
|> Enum.sum()
|> IO.inspect()
