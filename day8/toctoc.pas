{
fpc toctoc.pas && ./toctoc
}

//Allow processing of exception...
{$MODE OBJFPC}{$H+}

program toctoc;

uses
    StrUtils,
    Classes,
    SysUtils,
    Types;
type 
  InputItem = array of Integer;
  InputType = array of InputItem;
  
function ParseLine(line : string) : InputItem;
var
    i : Integer;
    res : Integer;
begin
    SetLength(ParseLine, Length(line));
    for i := 0 to Length(line)-1 do
    begin
        res := StrToInt(line.Substring(i, 1));
        ParseLine[i] := res;
    end;
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
    j : Integer;
begin
    for i := 0 to Length(inputs)-1 do
    begin
        for j := 0 to Length(inputs[i])-1 do
            Write(inputs[i,j], '|');
        WriteLn();
    end;
end;

function IsVisible(inputs : InputType; i : Integer; j : Integer) : Integer;
var
    k : Integer;
    vis : Boolean;
begin
    IsVisible := 0;
    vis := true;
    for k := 0 to i-1 do
    begin
        if (vis = true) and (inputs[k,j] >= inputs[i,j]) then
            vis := false;
    end;   
    if vis = true then
    begin
        IsVisible := 1;
        exit;
    end;
        
    vis := true;
    for k := i+1 to Length(inputs)-1 do
    begin
        
        if (vis = true) and (inputs[k,j] >= inputs[i,j]) then
            vis := false;
    end;   
    if vis = true then
    begin
        IsVisible := 1;
        exit;
    end;
        
    vis := true;
    for k := 0 to j-1 do
    begin
        if (vis = true) and (inputs[i,k] >= inputs[i,j]) then
            vis := false;
    end;   
    if vis = true then
    begin
        IsVisible := 1;
        exit;
    end;
        
    vis := true;
    for k := j+1 to Length(inputs[i])-1 do
    begin
        if (vis = true) and (inputs[i,k] >= inputs[i,j]) then
            vis := false;
    end;   
    if vis = true then
    begin
        IsVisible := 1;
        exit;
    end;
end;


function ScenicScore(inputs : InputType; i : Integer; j : Integer) : Integer;
var
    k : Integer;
    scenics : array [0..4] of integer;
begin
    ScenicScore := 0;
    scenics[0] := 0;
    scenics[1] := 0;
    scenics[2] := 0;
    scenics[3] := 0;

    for k := i-1 downto 0 do
    begin
        scenics[0] := scenics[0]+1;
        if (inputs[k,j] >= inputs[i,j]) then
            break;
    end;
        
    for k := i+1 to Length(inputs)-1 do
    begin
        scenics[1] := scenics[1]+1;
        if (inputs[k,j] >= inputs[i,j]) then
            break;
    end;   
        
    for k := j-1 downto 0 do
    begin
        scenics[2] := scenics[2]+1;
        if (inputs[i,k] >= inputs[i,j]) then
            break;
    end;   
        
    for k := j+1 to Length(inputs[i])-1 do
    begin
        scenics[3] := scenics[3]+1;
        if (inputs[i,k] >= inputs[i,j]) then
            break;
    end;
    
    ScenicScore := scenics[0]*scenics[1]*scenics[2]*scenics[3];
end;


function Exo1(inputs : InputType) : Integer;
var
    i : Integer;
    j : Integer;
begin
    Exo1 := 0;
    for i := 0 to Length(inputs)-1 do
        for j := 0 to Length(inputs[i])-1 do
        begin
            Exo1 := Exo1 + IsVisible(inputs, i, j);
        end;
end;

function Exo2(inputs : InputType) : Integer;
var
    i : Integer;
    j : Integer;
    curr_score : Integer;
begin
    Exo2 := 0;
    for i := 0 to Length(inputs)-1 do
        for j := 0 to Length(inputs[i])-1 do
        begin
            curr_score := ScenicScore(inputs, i, j);
            if curr_score > Exo2 then
                Exo2 := curr_score;
        end;
end;

var
    inputs : InputType;
    answ1, answ2 : Integer;
begin
    inputs := ParseInput('input.txt');
    //PrintInput(inputs);
    
    answ1 := Exo1(inputs);
    WriteLn('The first answer is ', answ1);
    
    answ2 := Exo2(inputs);
    WriteLn('The second answer is ', answ2);
end.