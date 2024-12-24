#include <stdio.h>
#include <assert.h>

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

struct TreeNode* searchBST(struct TreeNode* root, int val) {
  if (root == NULL)
    return NULL;

  if (root->val == val)
    return root;

  if (val < root->val)
    return searchBST(root->left, val);

  return searchBST(root->right, val);
};

int main(int argc, char* argv[]) {
  // Input: root = [4,2,7,1,3], val = 2
  // Output: [2,1,3]
  struct TreeNode n1, n2, n3, n4, n5;
  n1.val = 4;
  n2.val = 2;
  n3.val = 7;
  n4.val = 1;
  n5.val = 3;
  n1.left = &n2;
  n1.right = &n3;
  n2.left = &n4;
  n3.right = &n5;
  struct TreeNode* result = searchBST(&n1, 2);
  printf("result = %i\n", result->val);
  assert(result->val == 2);
  result = searchBST(&n1, 5);
  assert(result == NULL);
  return 0;
}
