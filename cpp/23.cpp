#include <iostream>
#include <vector>

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
struct ListNode {
  int val;
  ListNode *next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode *next) : val(x), next(next) {}
};
class Solution {
  public:
    ListNode* mergeKLists(std::vector<ListNode*>& lists) {
      ListNode* head = nullptr;
      ListNode* tail = nullptr;
      ListNode* minNode = nullptr;
      size_t size = lists.size();

      while (true) {
        int listIndex = -1;
        minNode = nullptr;

        // find the next min node
        for (int i = 0; i < size; i++) {
          std::cout << "checking list " << i + 1 << "\n";
          ListNode* node = lists[i]; // current node of the (advancing) list

          if (node == nullptr)
            continue;
          
          if (minNode == nullptr || node->val < minNode->val) {
            minNode = node;
            listIndex = i;
          }
        }

        if (minNode == nullptr) {
          break; // all lists are empty
        } else {
          ListNode* n = lists[listIndex];
          lists[listIndex] = n->next; // advance the list containing the min node
        }

        // add the min node to the result list
        if (head == nullptr) {
          head = new ListNode(minNode->val);
          tail = head;
        } else {
          tail->next = new ListNode(minNode->val);
          tail = tail->next;
        }
      }

      return head;
    }
};

void setup1(ListNode& n1, ListNode& n2, ListNode& n3, ListNode& n4, ListNode& n5, ListNode& n6, ListNode& n7, ListNode& n8) {
  n1.val = 1;
  n1.next = &n2;
  n2.val = 4;
  n2.next = &n3;
  n3.val = 5;
  n3.next = nullptr;
  n4.val = 1;
  n4.next = &n5;
  n5.val = 3;
  n5.next = &n6;
  n6.val = 4;
  n6.next = nullptr;
  n7.val = 2;
  n7.next = &n8;
  n8.val = 6;
  n8.next = nullptr;
}

int main (int argc, char *argv[]) {
  ListNode n1, n2, n3, n4, n5, n6, n7, n8; 
  std::vector<ListNode*> lists;
  setup1(n1, n2, n3, n4, n5, n6, n7, n8);
  lists.push_back(&n1);
  lists.push_back(&n4);
  lists.push_back(&n7);

  Solution s;
  ListNode* node = s.mergeKLists(lists);

  std::cout << "The list is: ";
  while (node != nullptr) {
    std::cout << node->val << " ";
    node = node->next;
  }

  std::cout << "\n";

  return 0;
}
