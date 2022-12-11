
// gcc -std=gnu99 tata.c && ./a.out
// Mais bon, en vrai, c'est plutôt
// gcc -std=gnu99 tata.c && valgrind ./a.out

#include <stdio.h>
#include <assert.h>
#include <stdlib.h>
#include <string.h>

typedef struct Rucksack {
    char* data;
    struct Rucksack* next;
} Rucksack;

Rucksack* parse_inputs(const char* filepath)
{
    FILE *f = fopen(filepath,"r");
    assert(f!=NULL);
    
    Rucksack* r = malloc(sizeof(Rucksack));
    assert(r!=NULL);
    r->data = NULL;
    r->next = NULL;
    
    Rucksack* curr = r;
    Rucksack* prev = NULL;
    int read = 0;
    size_t len = 0;
    while ((read = getline(&(curr->data), &len, f)) != -1) {
        curr->next = malloc(sizeof(Rucksack));
        assert(curr->next!=NULL);
        prev = curr;
        curr = curr->next;
        curr->data = NULL;
        curr->next = NULL;
    }
    
    // Faudrait faire une boucle plus maline, pour éviter d'avoir à désallouer ce dernier truc
    // On dit qu'on s'en fout?
    free(prev->next->data);
    free(prev->next);
    prev->next = NULL;

    fclose(f);
    return r;
}

void free_inputs(Rucksack* inputs)
{
    Rucksack* curr = inputs;
    while(curr != NULL) {
        free(curr->data);
        Rucksack* prev = curr;
        curr = curr->next;
        free(prev);
    }
}


int get_priority(char c) {
    if(c >= 'a' && c <= 'z') {
        return 1+c-'a';
    } else if( c >= 'A' && c <= 'Z') {
        return 27+c-'A';
    } else {
        return 0; // HOHO
    }
}

int exo1(const Rucksack* inputs)
{
    int priority_1 = 0;
    const Rucksack* ptr = inputs;
    while(ptr != NULL) {
        int found[256] = {};
        int length = strlen(ptr->data)-1; // Trailing \n at the end of the string
        int i = 0;
        for(i = 0; i < length/2; ++i) {
            for(int j = length/2; j < length; ++j) {
                char c = ptr->data[i];
                if(c == ptr->data[j] && found[c] == 0) {
                    priority_1 += get_priority(c);            
                    found[c] = 1;
                }
            }
        }

        ptr = ptr->next;
    }
    return priority_1;
}

int is_in_string(const char* str, char c)
{
    int length = strlen(str);
    for(int i = 0; i < length; ++i) {
        if(str[i] == c) {
            return 1;
        }
    }
    return 0;
}

int exo2(const Rucksack* inputs)
{
    int priority = 0;
    const Rucksack* ptr = inputs;
    while(ptr != NULL) {
        int length = strlen(ptr->data)-1; // Trailing \n at the end of the string
        int i = 0;
        Rucksack* n = ptr->next;
        assert(n!=NULL);
        Rucksack* nn = n->next;
        assert(n!=NULL);
        for(i = 0; i < length; ++i) {
            char c = ptr->data[i];
            if(is_in_string(n->data, c) && is_in_string(nn->data, c)) {
                priority += get_priority(c);
                break;
            }
        }
        ptr = nn->next;
    }
    return priority;
}



int main(int argc, char* argv[])
{
    Rucksack* inputs = parse_inputs("input.txt");
    
    // Part 1
    printf("The first answer is %d\n", exo1(inputs));

    // Part 2
    printf("The second answer is %d\n", exo2(inputs));
    
    free_inputs(inputs);
    return 0;
}