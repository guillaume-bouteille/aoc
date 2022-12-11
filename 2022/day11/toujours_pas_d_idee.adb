
-- gnatmake toujours_pas_d_idee.adb && ./toujours_pas_d_idee
pragma Ada_2020;

with Text_IO; use Text_IO;
with Ada.Containers.Indefinite_Vectors;
with Ada.Strings.Maps; use Ada.Strings.Maps;
with Ada.Strings.Fixed; use Ada.Strings.Fixed;
with Ada.Numerics.Big_Numbers.Big_Integers;
use Ada.Numerics.Big_Numbers.Big_Integers;
use Ada.Containers;
use Ada.Strings;
--use Ada.Text_IO, Ada.Containers, Ada.Strings, Ada.Strings.Fixed, Ada.Strings.Maps;

procedure toujours_pas_d_idee is
    package VectInt is new Ada.Containers.Indefinite_Vectors(Index_Type => Natural, Element_Type => Integer);
    use type VectInt.Vector;
    package VectLInt is new Ada.Containers.Indefinite_Vectors(Index_Type => Natural, Element_Type => Big_Integer);
    use type VectLInt.Vector;
    package VectInt_Sorting is new VectInt.Generic_Sorting;
    use VectInt_Sorting;

    
    package VectString is new Ada.Containers.Indefinite_Vectors(Index_Type => Natural, Element_Type => String);
    use type VectString.Vector;

    type Monkey is record
       Items   : VectLInt.Vector;
       Operation_Arg_1 : Big_Integer := 0; -- if set to 0, means "old"
       Operation_Arg_2 : Big_Integer := 0; -- if set to 0, means "old"
       Operation_Operand : Character := '+';
       Test_Divider : Big_Integer := 1;
       True_Monkey : Integer := 0;
       False_Monkey : Integer := 0;
       Nb_Inspections : Integer := 0;
    end record;
    
    package VectMonkey is new Ada.Containers.Indefinite_Vectors(Index_Type => Natural, Element_Type => Monkey);
    use type VectMonkey.Vector;
    
    type ItemPass is record
        Item : Big_Integer;
        Monkey_Id : Integer;
    end record;
    
    package VectIP is new Ada.Containers.Indefinite_Vectors(Index_Type => Natural, Element_Type => ItemPass);
    use type VectIP.Vector;
    
    function Parse_Start_Items(Input : String) return VectLInt.Vector is
       Start  : Positive := Input'First;
       Finish : Natural  := 0;
       Output : VectLInt.Vector := VectLInt.Empty_Vector;
    begin
        while Start <= Input'Last loop
            Find_Token (Input, To_Set (','), Start, Outside, Start, Finish);
            exit when Start > Finish;
            Output.Append ( To_Big_Integer(Integer'Value(Input (Start .. Finish))));
            Start := Finish + 1;
        end loop;
        return Output;
    end Parse_Start_Items;
    
    procedure Parse_Operation(Input : String; Arg1 : out Big_Integer; Arg2 : out Big_Integer; Operand : out Character ) is
       Start  : Positive := Input'First;
       Finish : Natural  := 0;
       Tmp : VectString.Vector := VectString.Empty_Vector;
    begin
        while Start <= Input'Last loop
            Find_Token (Input, To_Set (' '), Start, Outside, Start, Finish);
            exit when Start > Finish;
            Tmp.Append ( Input (Start .. Finish));
            Start := Finish + 1;
        end loop;
        
        -- Arg1
        if Tmp(0) = "old" then
            Arg1 := 0;
        else
            Arg1 := To_Big_Integer(Integer'Value(Tmp(0)));
        end if;
        
        -- OK... So it seems that strings may have indexes that are not 0..length-1 or 1..length,
        -- but instead depend on if they are a slice of another string.
        -- In the following, the correct start index is 24, but since the string is meant to contain
        -- a single char, it seems safer just to access to chars through iteration
        -- Pfiou!
        for c of Tmp(1) loop
            Operand := c;
        end loop;

        -- Arg2
        if Tmp(2) = "old" then
            Arg2 := 0;
        else
            Arg2 := To_Big_Integer(Integer'Value(Tmp(2)));
        end if;
        
    end Parse_Operation;
    
    function Parse_Inputs(File_Path : String) return VectMonkey.Vector is
        File : File_Type;
        Inputs : VectMonkey.Vector;
        Tmp : String(1..100);
        Tmp2 : Natural := 100;
        Tmp_Monkey : Monkey;
    begin
        Open (File => File,
            Mode => In_File,
            Name => File_Path);
            
        While not  End_Of_File (File) Loop
        
            -- Line: |Monkey <idx>: - OSEF - assume monkey are sorted in inputs
            Get_Line(File, Tmp, Tmp2);

            -- Line: |  Starting items: <item1>, <item2>, ...
            Get_Line(File, Tmp, Tmp2);
            Tmp_Monkey.Items := Parse_Start_Items(Tmp(18 .. Tmp2));
            
            -- Line: |  Operation: new = old * 19
            Get_Line(File, Tmp, Tmp2);
            Parse_Operation(
                Input => Tmp(19 .. Tmp2),
                Arg1 => Tmp_Monkey.Operation_Arg_1,
                Arg2 => Tmp_Monkey.Operation_Arg_2,
                Operand => Tmp_Monkey.Operation_Operand);
            
            -- Line: |  Test: divisible by 7
            Get_Line(File, Tmp, Tmp2);
            Tmp_Monkey.Test_Divider := To_Big_Integer(Integer'Value(Tmp(22 .. Tmp2)));

            -- Line: |    If true: throw to monkey 2
            Get_Line(File, Tmp, Tmp2);

            Tmp_Monkey.True_Monkey := Integer'Value(Tmp(29 .. Tmp2));

            -- Line: |    If false: throw to monkey 3
            Get_Line(File, Tmp, Tmp2);
            Tmp_Monkey.False_Monkey := Integer'Value(Tmp(30 .. Tmp2));

            Inputs.Append ( Tmp_Monkey );

            -- Line: whitespace - OSEF
            exit when End_Of_File (File);
            Get_Line(File, Tmp, Tmp2);
        end loop;

        Close (File);
        return Inputs;
    end Parse_Inputs;
    
    procedure Execute_Round(Monkeys : in out VectMonkey.Vector; Managed_Worry : Boolean) is
        Worry_Level : Big_Integer := 0;
        Arg1 : Big_Integer := 0;
        Arg2 : Big_Integer := 0;
        Test_Result : Boolean := False;
        Passes : VectIP.Vector := VectIP.Empty_Vector;
        Pass : ItemPass;
    begin
        for Monkey of Monkeys loop
            for Item of Monkey.Items loop
                Monkey.Nb_Inspections := Monkey.Nb_Inspections + 1;
                -- Compute worry level
                if Monkey.Operation_Arg_1 = 0 then
                    Arg1 := Item;
                else 
                    Arg1 := Monkey.Operation_Arg_1;
                end if;
                if Monkey.Operation_Arg_2 = 0 then
                    Arg2 := Item;
                else 
                    Arg2 := Monkey.Operation_Arg_2;
                end if;
                
                if Monkey.Operation_Operand = '+' then
                    Worry_Level := Arg1 + Arg2;
                elsif Monkey.Operation_Operand = '*' then 
                    Worry_Level := Arg1 * Arg2;
                else
                    Put_Line("WTFBBQ!");
                end if;
                
                if Managed_Worry then
                    Worry_Level := Worry_Level / 3;
                end if;
                
                -- tant pis pour le truc propre de lire les inputs et tout
                Worry_Level := Worry_Level mod (7 * 19 * 5 * 11 * 17 * 13 * 2 * 3); 
                
                Test_Result:= (Worry_Level mod Monkey.Test_Divider) = 0;
                if Test_Result then
                    Pass := (Worry_Level, Monkey.True_Monkey);
                else
                    Pass := (Worry_Level, Monkey.False_Monkey);
                end if;
                Passes.Append(Pass);  
            end loop;
            
            -- Remove items from monkeys
            Monkey.Items.Clear;
            
            for P of Passes loop
                Monkeys(P.Monkey_Id).Items.Append(P.Item);
            end loop;
            Passes.Clear;
        end loop;
    end Execute_Round;
    
    procedure Print_Monkeys(Monkeys : VectMonkey.Vector) is
    begin
        for Monkey of Monkeys loop
            Put("Monkey:");
            for It of Monkey.Items loop
                Put(It'Img);
            end loop;
            Put_Line("");
        end loop;
    end;
    
    function Exo_1(Inputs : in out VectMonkey.Vector) return Integer is
        Answer : Integer := 0;
        Nb_Inspections : VectInt.Vector;
    begin
        for I in 1..20 loop
            Execute_Round(Inputs, True);
        end loop;
        
        for M of Inputs loop
            Nb_Inspections.Append(M.Nb_Inspections);
        end loop;
        
        Sort(Nb_Inspections);
        Answer := Nb_Inspections.Element(Nb_Inspections.Last_Index) * Nb_Inspections.Element(Nb_Inspections.Last_Index-1);

        return Answer;
    end Exo_1;
    
    function Exo_2(Inputs : in out VectMonkey.Vector) return Big_Integer is
        Answer : Big_Integer := 0;
        T1 : Integer := 0;
        T2 : Integer := 0;
        Nb_Inspections : VectInt.Vector;
    begin
        for I in 1..10000 loop
            Execute_Round(Inputs, False);
        end loop;
        
        for M of Inputs loop
            Nb_Inspections.Append(M.Nb_Inspections);
        end loop;
        
        Sort(Nb_Inspections);
        T1 := Nb_Inspections.Element(Nb_Inspections.Last_Index);
        T2 := Nb_Inspections.Element(Nb_Inspections.Last_Index-1);
        Answer := To_Big_Integer(T1) * To_Big_Integer(T2);

        return Answer;
    end Exo_2;
    
    Inputs : VectMonkey.Vector;
    Answer_1 : Integer;
    Answer_2 : Big_Integer;
begin
    Inputs := Parse_Inputs("input.txt");
    Answer_1 := Exo_1(Inputs);
    Put_Line("The first answer is " & Answer_1'Img);
    
    -- Input was modified by Exo 1
    Inputs := Parse_Inputs("input.txt");
    Answer_2 := Exo_2(Inputs);
    Put_Line("The second answer is " & Answer_2'Img);
    
end toujours_pas_d_idee;