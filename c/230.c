#include <stdio.h>
#include <stdlib.h>
#include "TreeNode.h"

void traverse(struct TreeNode* root, int k, int* ctr, int* result) {
  asdfsadf
  if (root == NULL) {
    return;
  }

  traverse(root->left, k, ctr, result);

  if (++*ctr == k) {
    *result = root->val;
    return;
  }

  traverse(root->right, k, ctr, result);
}

int kthSmallest(struct TreeNode* root, int k) {
  int ctr = 0;
  int result = -1;
  traverse(root, k, &ctr, &result);
  return result;
}

void setup1(struct TreeNode* n1, struct TreeNode* n2, struct TreeNode* n3, struct TreeNode* n4) {
  n1->val = 1;
  n2->val = 2;
  n3->val = 3;
  n4->val = 4;

  n1->left = NULL;
  n1->right = n2;
  n2->left = NULL;
  n2->right = NULL;
  n3->left = n1;
  n3->right = n4;
  n4->left = NULL;
  n4->right = NULL;
}

void setup2(struct TreeNode* n1, struct TreeNode* n2, struct TreeNode* n3, struct TreeNode* n4, struct TreeNode* n5, struct TreeNode* n6) {
  n1->val = 1;
  n2->val = 2;
  n3->val = 3;
  n4->val = 4;
  n5->val = 5;
  n6->val = 6;

  n1->left = NULL;
  n1->right = NULL;
  n2->left = n1;
  n2->right = NULL;
  n3->left = n2;
  n3->right = n4;
  n4->left = NULL;
  n4->right = NULL;
  n5->left = n3;
  n5->right = n6;
  n6->left = NULL;
  n6->right = NULL;
}

int main(int argc, char *argv[])
{
  struct TreeNode n1, n2, n3, n4, n5, n6;

  // int k = 1;
  // setup1(&n1, &n2, &n3, &n4);
  // int result = kthSmallest(&n3, k);

  int k = 3;
  setup2(&n1, &n2, &n3, &n4, &n5, &n6);
  int result = kthSmallest(&n5, k);

  printf("the %ith smallest element is %i\n", k, result);

  return EXIT_SUCCESS;
}
