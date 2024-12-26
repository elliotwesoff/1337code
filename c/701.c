#include <stdio.h>
#include <stdlib.h>

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
 struct TreeNode {
     int val;
     struct TreeNode *left;
     struct TreeNode *right;
 };

struct TreeNode* create(int val) {
  struct TreeNode* node;
  node = malloc(sizeof(struct TreeNode));
  node->val = val;
  node->left = NULL;
  node->right = NULL;
  return node;
}

void insert(struct TreeNode* node, int val) {
    if (val <= node->val) {
      if (node->left == NULL) {
        node->left = create(val);
      } else {
        insert(node->left, val);
      }
    } else {
      if (node->right == NULL) {
        node->right = create(val);
      } else {
        insert(node->right, val);
      }
    }
}

struct TreeNode* insertIntoBST(struct TreeNode* root, int val) {
  insert(root, val);
  return root;
}

int main(int argc, char* argv[]) {
  // struct TreeNode* root = insertIntoBST(n1, 5)
  return 0;
}
