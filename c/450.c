#include <stdio.h>
#include <stdlib.h>
#include "assert.h"

 struct TreeNode {
     int val;
     struct TreeNode *left;
     struct TreeNode *right;
 };

void printCurrentLevel(struct TreeNode* root, int level);
int height(struct TreeNode* node);

void printLevelOrder(struct TreeNode* root) {
    int h = height(root);
    int i;
    for (i = 1; i <= h; i++)
        printCurrentLevel(root, i);
}

void printCurrentLevel(struct TreeNode* root, int level) {
    if (root == NULL) {
      printf("null ");
      return;
    }
    if (level == 1)
        printf("%d ", root->val);
    else if (level > 1) {
        printCurrentLevel(root->left, level - 1);
        printCurrentLevel(root->right, level - 1);
    }
}

int height(struct TreeNode* node) {
    if (node == NULL)
        return 0;
    else {
        int lheight = height(node->left);
        int rheight = height(node->right);
        if (lheight > rheight)
            return (lheight + 1);
        else
            return (rheight + 1);
    }
}

void setup_case1(struct TreeNode* n1,struct TreeNode* n2,struct TreeNode* n3,struct TreeNode* n4,struct TreeNode* n5,struct TreeNode* n6) {
  // deletion of node with two children (case 3)
  n1->val = 5;
  n2->val = 3;
  n3->val = 6;
  n4->val = 2;
  n5->val = 4;
  n6->val = 7;
  n1->left = n2;
  n1->right = n3;
  n2->left = n4;
  n2->right = n5;
  n3->left = NULL;
  n3->right = n6;
  n4->left = NULL;
  n4->right = NULL;
  n5->left = NULL;
  n5->right = NULL;
  n6->left = NULL;
  n6->right = NULL;
}

void expect_case1(struct TreeNode* n1,struct TreeNode* n2,struct TreeNode* n3,struct TreeNode* n4,struct TreeNode* n5,struct TreeNode* n6) {
  assert(n1->left->val == n5->val);
  assert(n1->left->left->val == n4->val);
}

/*
 * Try to delete node that doesn't exist - case 0?
 */
void setup_case2(struct TreeNode* n1,struct TreeNode* n2,struct TreeNode* n3,struct TreeNode* n4,struct TreeNode* n5) {
  n1->val = 5;
  n2->val = 2;
  n3->val = 6;
  n4->val = 4;
  n5->val = 7;
  n1->left = n2;
  n1->right = n3;
  n2->left = NULL;
  n2->right = n4;
  n3->left = NULL;
  n3->right = n5;
  n4->left = NULL;
  n4->right = NULL;
  n5->left = NULL;
  n5->right = NULL;
}

void expect_case2(struct TreeNode* n1,struct TreeNode* n2,struct TreeNode* n3,struct TreeNode* n4,struct TreeNode* n5) {
  assert(n1->left->val == n2->val);
  assert(n1->right->val == n3->val);
  assert(n1->left->right->val == n4->val);
  assert(n1->right->right->val == n5->val);
}

/**
 * Delete node with no children - case 1
 * => try to delete node 4 (val = 4)
 */
void setup_case3(struct TreeNode* n1,struct TreeNode* n2,struct TreeNode* n3,struct TreeNode* n4,struct TreeNode* n5) {
  n1->val = 5;
  n2->val = 2;
  n3->val = 6;
  n4->val = 4;
  n5->val = 7;
  n1->left = n2;
  n1->right = n3;
  n2->left = NULL;
  n2->right = n4;
  n3->left = NULL;
  n3->right = n5;
  n4->left = NULL;
  n4->right = NULL;
  n5->left = NULL;
  n5->right = NULL;
}

void expect_case3(struct TreeNode* n1,struct TreeNode* n2,struct TreeNode* n3,struct TreeNode* n4,struct TreeNode* n5) {
  assert(n1->left->left == NULL);
  assert(n1->left->right == NULL);
}

void delete(struct TreeNode* node, int key) {
  if (node->left == NULL && node->right == NULL) { // case 1
    return;
  }

  if (node->left != NULL && node->right == NULL) { // case 2.1
    return;
  }

  if (node->left == NULL && node->right != NULL) { // case 2.2
    return;
  }
  
  if (node->left != NULL && node->right != NULL) { // case 3
    return;
  }
}

struct TreeNode* deleteNode(struct TreeNode* root, int key) {
  if (root == NULL) { // base case - node not found
    return NULL;
  }

  if (key < root->val) {
    deleteNode(root->left, key); // recurse
  } else if (key > root->val) {
    deleteNode(root->right, key); // recurse
  } else { // node found
    delete(root, key); // no recurse, let control go to end of fn
  }

  return root;
}

int main(int argc, char *argv[])
{
  struct TreeNode n1, n2, n3, n4, n5, n6;

  // try to delete node that doesn't exist
  setup_case2(&n1, &n2, &n3, &n4, &n5);
  deleteNode(&n1, 0);
  printLevelOrder(&n1);
  expect_case2(&n1, &n2, &n3, &n4, &n5);

  // delete node with 2 children - case 3
  // setup_case1(&n1, &n2, &n3, &n4, &n5, &n6);
  // deleteNode(&n1, 3);
  // expect_case1(&n1, &n2, &n3, &n4, &n5, &n6);

  return EXIT_SUCCESS;
}
