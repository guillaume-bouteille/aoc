
-- gnatmake tata.adb && ./tata

with Text_IO; use Text_IO;
with Ada.Containers.Indefinite_Vectors;
with Ada.Strings.Maps; use Ada.Strings.Maps;

procedure tata is

    package VectString is new Ada.Containers.Indefinite_Vectors(Index_Type => Natural, Element_Type => String);
    package VectChar is new Ada.Containers.Indefinite_Vectors(Index_Type => Natural, Element_Type => Character);

    use type VectString.Vector;
    use type VectChar.Vector;
    
    function Parse_Inputs(File_Path : String) return VectString.Vector is
        File : File_Type;
        Inputs : VectString.Vector;
    begin
        Open (File => File,
            Mode => In_File,
            Name => File_Path);
            
        While not  End_Of_File (File) Loop
            Inputs.Append ( Get_Line (File) );
        end loop;

        Close (File);
        return Inputs;
    end Parse_Inputs;
    
    function Get_Priority(C : Character) return Integer is
    begin
        if C >= 'a' and C <= 'z' then
            return 1+Character'Pos(C)-Character'Pos('a');
        elsif C >= 'A' and C <= 'Z' then
            return 27+Character'Pos(C)-Character'Pos('A');
        else
            return 0;
        end if;
    end Get_Priority;
    
    function Is_In(Str : String ; C : Character) return Boolean is
    begin
        for I in 1..Str'Length loop
            if Str(I) = C then
                return True;
            end if;
        end loop;
        return False;
    end Is_In;
    
    function Exo_1(Inputs : VectString.Vector) return Integer is
        Priority : Integer := 0;
        C : Character := '0';
        Compartment_Length : Integer := 0;
        Found_Char : VectChar.Vector;
    begin
        for E of Inputs loop
            Found_Char.Clear;
            Compartment_Length := E'Length/2;
            for I in 1..Compartment_Length loop
                C := E(I);
                for J in Compartment_Length+1..E'Length loop
                    if Found_Char.Find_Index(C) < 0 and E(J) = C then
                        --Put_Line(C'Img & " - " & Get_Priority(C)'Img);
                        Found_Char.Append(C);
                        Priority := Priority + Get_Priority(C);
                    end if;
                end loop;
            end loop;
        end loop;
        return Priority;
    end Exo_1;
    
    function Exo_2(Inputs : VectString.Vector) return Integer is
        Priority : Integer := 0;
        Nb_Inputs : Integer := Integer(Inputs.Length);
        Found_Char : VectChar.Vector;
    begin
        for I in 1..Nb_Inputs/3 loop
            Found_Char.Clear;
            for C of Inputs((I-1)*3) loop
                if Found_Char.Find_Index(C) < 0 and Is_In(Inputs((I-1)*3+1), C) and Is_In(Inputs((I-1)*3+2), C) then
                    Priority := Priority + Get_Priority(C);
                    Found_Char.Append(C);
                end if;
            end loop;
        end loop;
        return Priority;
    end Exo_2;
    
    Inputs : VectString.Vector;
    Answer_1 : Integer;
    Answer_2 : Integer;
begin
    Inputs := Parse_Inputs("input.txt");
    
    Answer_1 := Exo_1(Inputs);
    Put_Line("The first answer is " & Answer_1'Img);
    Answer_2 := Exo_2(Inputs);
    Put_Line("The second answer is " & Answer_2'Img);
    
end tata;