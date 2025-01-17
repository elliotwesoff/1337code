#include <stdio.h>
#include <stdlib.h>

struct TreeNode {
    int val;
    struct TreeNode *left;
    struct TreeNode *right;
};

int* traverse(struct TreeNode* n, int* a) {
    if (n == NULL)
        return a;

    a = traverse(n->left, a);
    printf("%p : %d\n", a, n->val);
    *a = n->val;
    a++;
    return traverse(n->right, a);
}

int* inorderTraversal(struct TreeNode* root, int* returnSize) {
    int* a = malloc(*returnSize * sizeof(int));
    traverse(root, a);
    return a;
}

int main(int argc, char *argv[])
{
  struct TreeNode n1, n2, n3;
  n1.val = 1;
  n2.val = 2;
  n3.val = 3;
  n1.left = NULL;
  n1.right = &n2;
  n2.left = &n3;
  n2.right = NULL;
  n3.left = NULL;
  n3.right = NULL;
  int* returnSize = malloc(sizeof(int));
  *returnSize = 3;
  int* result = inorderTraversal(&n1, returnSize);
  for (int i = 0; i < *returnSize; i++)
    printf("(%p : %d) ", result + (i * sizeof(int)), result[i]);
  printf("\n");
  return EXIT_SUCCESS;
}
