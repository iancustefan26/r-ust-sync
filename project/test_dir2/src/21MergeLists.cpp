#include <iostream>

struct ListNode
{
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};
// list

class Solution
{
public:
    ListNode *mergeTwoLists(ListNode *list1, ListNode *list2)
    {
        if (!list1 && !list2)
            return nullptr;

        ListNode *root;

        if (list1 && !list2)
            return list1;
        else if (!list1 && list2)
            return list2;
        else
        {
            if (list1->val > list2->val)
                root = list1, list1 = list1->next;
            else
                root = list2, list2 = list2->next;
        }

        ListNode *temp = root;
        while (list1 && list2)
            if (list1->val > list2->val)
                temp->next = list1, list1 = list1->next, temp = temp->next;
            else
                temp->next = list2, list2 = list2->next, temp = temp->next;
        while (list1)
            temp->next = list1, list1 = list1->next, temp = temp->next;
        while (list2)
            temp->next = list2, list2 = list2->next, temp = temp->next;

        return root;
    }
};