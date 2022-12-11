
# ruby tata.rb

def parse_inputs(file)
    file = File.read(file).split
end

def get_priority(c)
    if c >= 'a'.ord and c <= 'z'.ord
        1+c-'a'.ord
    elsif c >= 'A'.ord and c <= 'Z'.ord
        27+c-'A'.ord
    else
        0 # Boooouh
    end    
end

inputs = parse_inputs("input.txt")

# Part 1
priority_1 = 0
for rucksack in inputs
    l2 = rucksack.length/2
    compartment_1 = rucksack[0..l2-1]
    compartment_2 = rucksack[l2..-1]
    compartment_1.chars.sort.join.squeeze.each_char { |c|
        if compartment_2.count(c) > 0
            priority_1 += get_priority(c.ord)
        end
    }
end
puts "The first answer is #{priority_1}"

# Part 2
priority_2 = 0
for i in 0..(inputs.length/3)-1
    inputs[i*3].chars.sort.join.squeeze.each_char { |c|
        if inputs[i*3+1].count(c) > 0 and inputs[i*3+2].count(c) > 0
            priority_2 += get_priority(c.ord)
        end
    }
end
puts "The second answer is #{priority_2}"
