
// gcc -std=gnu99 j_ai_plus_d_idee_pour_le_nom.c && ./a.out
// Mais bon, en vrai, c'est plutôt
// gcc -std=gnu99 j_ai_plus_d_idee_pour_le_nom.c && valgrind ./a.out

#include <stdio.h>
#include <assert.h>
#include <stdlib.h>
#include <string.h>

typedef enum OpCode
{
    OPCODE_UNDEFINED,
    OPCODE_NOOP,
    OPCODE_ADDX,
} OpCode;

typedef struct Instruction {
    OpCode opcode;
    int arg;
} Instruction;

typedef struct InstructionList {
    Instruction instr;
    struct InstructionList* next;
} InstructionList;

InstructionList* parse_inputs(const char* filepath)
{
    FILE *f = fopen(filepath,"r");
    assert(f!=NULL);
    
    InstructionList* r = malloc(sizeof(InstructionList));
    assert(r!=NULL);
    r->instr.opcode = OPCODE_UNDEFINED;
    r->instr.arg = 0;
    r->next = NULL;
    
    InstructionList* curr = r;
    InstructionList* prev = NULL;
    int read = 0;
    size_t len = 0;
    char* line = NULL;
    char delim[] = {' ', '\n'};
    while ((read = getline(&line, &len, f)) != -1) {
        
        char* ptr = strtok(line, delim);
        if(strcmp(ptr, "noop") == 0) {
            curr->instr.opcode = OPCODE_NOOP;
        } else if(strcmp(ptr, "addx") == 0) {
            curr->instr.opcode = OPCODE_ADDX;
        } else {
            // Bouhou
            assert(0);
        }
        
        ptr = strtok(NULL, delim);
        switch(curr->instr.opcode) {
        case OPCODE_NOOP:
            assert(ptr == NULL);
            break;
        case OPCODE_ADDX:
            assert(strlen(ptr) > 0);
            curr->instr.arg = atoi(ptr);
            break;
        }

        curr->next = malloc(sizeof(InstructionList));
        assert(curr->next!=NULL);
        prev = curr;
        curr = curr->next;
        curr->instr.opcode = OPCODE_UNDEFINED;
        curr->instr.arg = 0;
        curr->next = NULL;   
    }
    free(line);

    // Faudrait faire une boucle plus maline, pour éviter d'avoir à désallouer ce dernier truc
    // On dit qu'on s'en fout?
    free(prev->next);
    prev->next = NULL;

    fclose(f);
    return r;
}

void free_inputs(InstructionList* inputs)
{
    InstructionList* curr = inputs;
    while(curr != NULL) {
        InstructionList* prev = curr;
        curr = curr->next;
        free(prev);
    }
}

typedef struct Cpu
{
    int reg_x;
    const InstructionList* pc;
    Instruction current_instruction;
    int remaining_cycles; // for current instruction
} Cpu;

typedef struct Crt
{
    char data[6][40];
} Crt;

void start_cycle(Cpu* cpu)
{
    if(cpu->current_instruction.opcode == OPCODE_UNDEFINED) {
        // Fetch next instruction
        cpu->current_instruction = cpu->pc->instr;
        switch(cpu->current_instruction.opcode) {
        case OPCODE_NOOP:
            cpu->remaining_cycles = 1;
            break;
        case OPCODE_ADDX:
            cpu->remaining_cycles = 2;
            break;
        default:
            assert(0);            
        }
    }
}

void finish_cycle(Cpu* cpu)
{
    // Execute current instruction
    cpu->remaining_cycles--;
    if(cpu->remaining_cycles <= 0) {
        switch(cpu->current_instruction.opcode) {
        case OPCODE_NOOP:
            break;
        case OPCODE_ADDX:
            cpu->reg_x += cpu->current_instruction.arg;
            break;
        default:
            assert(0);
        }
        cpu->current_instruction.opcode = OPCODE_UNDEFINED;
        
        cpu->pc = cpu->pc->next;
        assert(cpu->pc != NULL);
    }
}

void print_crt(Crt* crt)
{
    for(int y = 0; y < 6; y++) {
    for(int x = 0; x < 40; x++) {
            printf("%c", crt->data[y][x]);
        }
        printf("\n");
    }
}

void render_crt(Crt* crt, int cycle, int x)
{
    int crt_x = ((cycle-1) % 40);
    int crt_y = cycle/40;
    char c = '.';
    if(abs(crt_x-x) <= 1) {
        c = '#';
    }
    crt->data[crt_y][crt_x] = c;
}

int exo1(const InstructionList* inputs)
{
    int answ = 0;
    Cpu cpu = {
        reg_x:1,
        pc:inputs,
        current_instruction: {opcode: OPCODE_UNDEFINED, arg:0},
        remaining_cycles:0
    };
    
    for(int cycle = 1; cycle < 230; cycle++) {
        start_cycle(&cpu);
        if((cycle % 40) == 20) {
            answ += cycle * cpu.reg_x;
        }
        finish_cycle(&cpu);
    }
    return answ;
}

void exo2(const InstructionList* inputs, Crt* crt)
{
    memset(crt->data, '.', sizeof(crt->data));
    Cpu cpu = {
        reg_x:1,
        pc:inputs,
        current_instruction: {opcode: OPCODE_UNDEFINED, arg:0},
        remaining_cycles:0
    };
    
    for(int cycle = 1; cycle < 240; cycle++) {
        start_cycle(&cpu);
        render_crt(crt, cycle, cpu.reg_x);
        finish_cycle(&cpu);
    }
}

int main(int argc, char* argv[])
{
    InstructionList* inputs = parse_inputs("input.txt");
    
    // Part 1
    printf("The first answer is %d\n", exo1(inputs));

    // Part 2
    Crt crt = {};
    exo2(inputs, &crt);
    printf("The second answer is\n");
    print_crt(&crt);
    
    free_inputs(inputs);
    return 0;
}