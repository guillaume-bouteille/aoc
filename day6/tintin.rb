
# ruby tintin.rb

require 'set'

def parse_inputs(file)
    File.read(file)
end

def all_different(inputs, i, nb)
    s = Set[]
    for j in 0..nb-1
        s.add(inputs[i+j])
    end
    s.length == nb
end

inputs = parse_inputs("input.txt")

# Part 1
idx = 0
for i in 0..inputs.length-5
    if all_different(inputs, i, 4)
        idx = i+4
        break
    end
end
puts "The first answer is #{idx}"

# Part 2
idx = 0
for i in 0..inputs.length-5
    if all_different(inputs, i, 14)
        idx = i+14
        break
    end
end
puts "The second answer is #{idx}"
