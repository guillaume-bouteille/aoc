
// java tata.java

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;
import java.util.ArrayList;

class Tata {
    
    public static List<String> parseInputs(String filepath) throws IOException {
        return Files.readAllLines(Paths.get(filepath));
    }
    
    public static int getPriority(char c) {
        if(c >= 'a' && c <= 'z') {
            return 1+c-'a';
        } else if( c>='A' && c<='Z') {
            return 27+c-'A';
        } else {
            return 0; // HOHO            
        }
    }
    
    public static int exo1(List<String> inputs) {
        int priority = 0;
        for(String rucksack : inputs) {
            String compartment_1 = rucksack.substring(0, rucksack.length()/2);
            String compartment_2 = rucksack.substring(rucksack.length()/2, rucksack.length());
            List<Character> found_char = new ArrayList<>();
            for(char c : compartment_1.toCharArray()) {
                if( (found_char.contains(c) == false) && (compartment_2.indexOf(c) > -1)) {
                    priority += getPriority(c);
                    found_char.add(c);
                }
            }
        }
        return priority;
    }

    public static int exo2(List<String> inputs) {
        int priority = 0;
        for(int i = 0; i < inputs.size()/3; ++i) {
            List<Character> found_char = new ArrayList<>();
            for(char c : inputs.get(3*i).toCharArray()) {
                if( (found_char.contains(c) == false) && (inputs.get(3*i+1).indexOf(c) > -1) && (inputs.get(3*i+2).indexOf(c) > -1))
                {
                    priority += getPriority(c);
                    found_char.add(c);
                }
            }
        }
        return priority;
    }
    
    public static void main(String[] args) {
        try {
            List<String> inputs = parseInputs("input.txt");
            int answ1 = exo1(inputs);
            System.out.println("The first answer is "+answ1); 
            int answ2 = exo2(inputs);
            System.out.println("The second answer is "+answ2); 
            
        } catch (IOException e) {
			e.printStackTrace();
		}
    }
}