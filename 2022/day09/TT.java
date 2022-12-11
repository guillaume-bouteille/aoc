
// java TT.java

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;
import java.util.ArrayList;
import java.awt.Point;
import java.util.Set;
import java.util.HashSet;
import java.lang.Math;

class TT {
    
    public static class Instruction
    {
        public Point direction; 
        public int length;
        public Instruction(Point i_dir, int i_length) {
            direction = i_dir;
            length = i_length;
        }
        public String toString() {
            return "Instruction{" + direction + "," + length + "}";
        }
    };
    
    public static class World
    {
        public List<Point> rope;

        public World(int nb_knot) {
            rope = new ArrayList<>();
            for(int i=0; i < nb_knot; i++) {
                rope.add(new Point());
            }
        }
        
        public String toString() {
            String s = "";
            
            // Pffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
            int x_min = +10000000;
            int y_min = +10000000;
            int x_max = -10000000;
            int y_max = -10000000;
            for(Point vp : rope) {
                x_min = Math.min(vp.x, x_min);
                y_min = Math.min(vp.y, y_min);
                x_max = Math.max(vp.x, x_max);
                y_max = Math.max(vp.y, y_max);
            }
        
            for(int y = y_max; y >= y_min; y--) {
                for(int x = x_min; x <= x_max; x++) {
                    s += (rope.contains(new Point(x, y)) ? "#" : "."); 
                }
                s += "\n";
            }
            
            return s;
        }
    };
        
    public static List<Instruction> parseInputs(String filepath) throws IOException {
        List<String> strs = Files.readAllLines(Paths.get(filepath));
        List<Instruction> res = new ArrayList<>();
        for(String s : strs) {
            char direction_c = s.toCharArray()[0];
            int length = Integer.parseInt(s.substring(2));
            Point direction;
            switch (direction_c) {
            case 'R':
                direction = new Point(+1, 0);
                break;
            case 'L':
                direction = new Point(-1, 0);
                break;
            case 'U':
                direction = new Point(0, +1);
                break;
            case 'D':
                direction = new Point(0, -1);
                break;
            default:
                throw new IOException("Unexpected direction " + direction_c);
            }
            res.add(new Instruction(direction, length));
        }
        return res;
    }
    
    public static void printVisited(Set<Point> visited_points) {
        // Pffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
        int x_min = +10000000;
        int y_min = +10000000;
        int x_max = -10000000;
        int y_max = -10000000;
        for(Point vp : visited_points) {
            x_min = Math.min(vp.x, x_min);
            y_min = Math.min(vp.y, y_min);
            x_max = Math.max(vp.x, x_max);
            y_max = Math.max(vp.y, y_max);
        }
        
        for(int y = y_max; y >= y_min; y--) {
            for(int x = x_min; x <= x_max; x++) {
                System.out.print((visited_points.contains(new Point(x, y)) ? "#" : ".")); 
            }
            System.out.println(""); 
        }
    }
    
    public static void executeInstruction(World w, Instruction instr, Set<Point> visited_points) {
        for(int i = 0; i < instr.length; i++) {
            w.rope.get(0).translate(instr.direction.x, instr.direction.y);
            
            for(int j = 0; j < w.rope.size()-1; j++) {
                int dx = w.rope.get(j).x - w.rope.get(j+1).x;
                int dy = w.rope.get(j).y - w.rope.get(j+1).y;
                if( Math.max( Math.abs(dx), Math.abs(dy)) > 1) { // Too far, need to move
                    if(dx == 0) { // same line
                        w.rope.get(j+1).translate(0, (dy > 0 ? +1 : -1));
                    } else if(dy == 0) { // same column
                        w.rope.get(j+1).translate((dx > 0 ? +1 : -1), 0);
                    } else { // we have to move in diagonal
                        w.rope.get(j+1).translate(
                            (dx > 0 ? +1 : -1),
                            (dy > 0 ? +1 : -1));
                    }
                }
            }
            //System.out.println(w); 
            //System.out.println("head="+w.rope.get(0)+" tail="+w.rope.get(w.rope.size()-1));
            
            Point tail = w.rope.get(w.rope.size()-1);
            visited_points.add(new Point(tail.x, tail.y));
            //System.out.println(visited_points);
        }
    }

    public static int exo1(List<Instruction> inputs) {
        Set<Point> visited_points = new HashSet<>();
        World w = new World(2);
        for(Instruction i : inputs) {
            executeInstruction(w, i, visited_points);
        }
        return visited_points.size();
    }
    

    public static int exo2(List<Instruction> inputs) {
        Set<Point> visited_points = new HashSet<>();
        World w = new World(10);
        for(Instruction i : inputs) {
            executeInstruction(w, i, visited_points);
        }
        //printVisited(visited_points);
        return visited_points.size();
    }
    
    public static void main(String[] args) {
        try {
            List<Instruction> inputs = parseInputs("input.txt");
            // System.out.println(inputs); 

            int answ1 = exo1(inputs);
            System.out.println("The first answer is "+answ1); 
            int answ2 = exo2(inputs);
            System.out.println("The second answer is "+answ2); 
            
        } catch (IOException e) {
			e.printStackTrace();
		}
    }
}