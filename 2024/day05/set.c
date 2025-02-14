#include "set.h"

#include <stdio.h>
#include <stdlib.h>

const size_t SET_SIZE = 71;

Set* create_set(size_t (*hash_func)(void* key))
{
    Set* set = malloc(sizeof(Set));
    set->count = SET_SIZE;
    set->buckets = calloc(set->count, sizeof(Node*));
    set->hash_func = hash_func;
    
    return set;
}

void add(struct Set* set, void* value) {
    struct Node* node = malloc(sizeof(struct Node));
    
    node->hash = set->hash_func(value);
    node->value = value;

    size_t pos = node->hash % SET_SIZE;
    if (!add_node(set->buckets[pos], node))
    {
        set->buckets[pos] = node;
    }
}

int add_node(struct Node* cur, Node* value) 
{
    if (cur == NULL) 
    {
        return 0;
    }

    if (cur->next == NULL) {
        cur->next = value;
        return 1;
    }

    return add_node(cur->next, value);
}

int exists(struct Set* set, void* value) 
{
    size_t hash = set->hash_func(value);
    size_t pos = hash % SET_SIZE;

    struct Node* cur = set->buckets[pos];

    while (cur != NULL)
    {
        if (cur->hash == hash)
        {
            return 1;
        }

        cur = cur->next;
    }
    
    return 0;
}