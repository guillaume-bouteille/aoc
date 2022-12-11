{
fpc titi.pas && ./titi
}

//Allow processing of exception...
{$MODE OBJFPC}{$H+}

program titi;

uses
    StrUtils,
    Classes,
    SysUtils,
    Types;

type 
  InputItem = array [0..3] of Integer;
  InputType = array of InputItem;
  
function ParseLine(line : string) : InputItem;
var
    idx1 : Integer;
    idx2 : Integer;
    idx3 : Integer;
begin
    // Maybe there is a nicer way, but I’m tired of getting lost in google searchs
    Initialize(ParseLine, 4);
    idx1 := Pos('-', line);
    idx2 := Pos(',', line);
    idx3 := RPos('-', line);
    ParseLine[0] := StrToInt(line.Substring(0, idx1-1));
    ParseLine[1] := StrToInt(line.Substring(idx1, idx2-idx1-1));
    ParseLine[2] := StrToInt(line.Substring(idx2, idx3-idx2-1));
    ParseLine[3] := StrToInt(line.Substring(idx3, Length(line)-idx3));
    //WriteLn(ParseLine[0], '|', ParseLine[1], '|', ParseLine[2],'|',ParseLine[3]);
end;

function ParseInput (filePath : string) : InputType; 
var
    s: String;
    theFile: TextFile;
    lineNumber : Integer;
    output : InputType;
begin 

    // First iteration on file to count the number of lines
    AssignFile(theFile, filePath);
    reset(theFile);
    lineNumber := 0;
    while not eof(theFile) do
    begin
        readln(theFile, s);
        lineNumber := lineNumber + 1;
    end;
    CloseFile(theFile);

    // Set the right size for the output matrix
    SetLength(output, lineNumber);
    
    // Second iteration on file to fill the matrix
    AssignFile(theFile, filePath);
    reset(theFile);
    lineNumber := 0;
    while not eof(theFile) do
    begin
        readln(theFile, s);        
        output[lineNumber] := ParseLine(s);
        lineNumber := lineNumber + 1;
    end;

    CloseFile(theFile);
    
    ParseInput := output;
end;

procedure PrintInput(inputs : InputType);
var
    i : Integer;
begin
    for i := 0 to Length(inputs)-1 do
        WriteLn(inputs[i,0], '|', inputs[i,1], '|', inputs[i,2],'|', inputs[i,3]);
end;

function Exo1(inputs : InputType) : Integer;
var
    i : Integer;
begin
    Exo1 := 0;
    for i := 0 to Length(inputs)-1 do
    begin
        if (((inputs[i,0] >= inputs[i,2]) and (inputs[i,1] <= inputs[i,3])) or ((inputs[i,0] <= inputs[i,2]) and (inputs[i,1] >= inputs[i,3]))) then
            Exo1 := Exo1 + 1;
    end;
end;

function Exo2(inputs : InputType) : Integer;
var
    i : Integer;
begin
    Exo2 := 0;
    for i := 0 to Length(inputs)-1 do
    begin
        if (((inputs[i,0] >= inputs[i,2]) and (inputs[i,0] <= inputs[i,3])) or ((inputs[i,0] <= inputs[i,2]) and (inputs[i,2] <= inputs[i,1]))) then
            Exo2 := Exo2 + 1;
    end;
end;

var
    inputs : InputType;
    answ1, answ2 : Integer;
begin
    inputs := ParseInput('input_day4.txt');
    //PrintInput(inputs);
    
    answ1 := Exo1(inputs);
    WriteLn('The first answer is ', answ1);
    
    answ2 := Exo2(inputs);
    WriteLn('The second answer is ', answ2);
end.