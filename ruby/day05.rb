class Input
  attr_accessor :ranges
  attr_accessor :available

  def initialize(input)
    ranges, available = input.split("\n\n")

    @ranges = ranges.split("\n").map { |line|
      from, to = line.split("-")
      from.to_i()..to.to_i()
    }

    @available = available.split("\n").map { |available| available.to_i() }
  end
end

def part1(input)
  input.available
    .filter { |available| input.ranges.any? { |range| range.cover?(available) } }
    .count()
end

def part2(input)
  ranges = input.ranges()
  solutions = []

  i = 0
  while i < ranges.length do
    range = ranges[i]
    valid = true

    for prev in solutions do
      case [prev.cover?(range.begin), prev.cover?(range.end)]
      when [true, true]
        # This range is already fulfilled by a solution, so skip it.
        i += 1
        valid = false
        break
      when [true, false]
        # Update this range to not overlap an existing solution, then retry.
        ranges[i] = (prev.end + 1)..range.end
        valid = false
        break;
      when [false, true]
        # Update this range to not overlap an existing solution, then retry.
        ranges[i] = range.begin..(prev.begin - 1)
        valid = false
        break;
      else
        if range.begin < prev.begin && range.end > prev.end then
          ranges[i] = range.begin..(prev.begin - 1)
          ranges.push((prev.end + 1)..range.end)
          valid = false
          break;
        end
      end
    end

    if valid then
      solutions.push(range)
      i += 1
    end
  end

  solutions
    .map { |solution| solution.count() }
    .sum()
end

example = Input.new(File.read("../data/examples/05.txt"))
puts("example part 1: #{part1(example)}")
puts("example part 2: #{part2(example)}")

puts("----")

input = Input.new(File.read("../data/inputs/05.txt"))
puts("part 1: #{part1(input)}")
puts("part 2: #{part2(input)}")
