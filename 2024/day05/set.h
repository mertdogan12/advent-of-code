#include "stdio.h"

typedef struct Node Node;

struct Node 
{
    size_t hash;
    void* value;
    Node* next;
};

typedef struct Set
{
    Node** buckets;
    size_t count;
    size_t (*hash_func)(void* key);
} Set;

Set* create_set(size_t (*hash_func)(void* key));

void add(Set* set, void* value);

int add_node(Node* cur, Node* value);

int exists(Set* set, void* value);