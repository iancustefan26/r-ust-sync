#include <iostream>

using namespace std;

struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
public:
    ListNode* intersection(ListNode* head){
        ListNode* slow, *fast;
        slow = fast = head;
        while(fast -> next && fast -> next -> next){
            fast = fast -> next -> next;
            slow = slow -> next;
            if(slow == fast)
                return slow;
        }
        return nullptr;
    }
    ListNode *detectCycle(ListNode *head) {
        if(head == nullptr)
            return nullptr;
        ListNode* junction = intersection(head);
        if(junction == nullptr)
            return nullptr;
        
        while(head != junction)
            head = head -> next,
            junction = junction -> next;

        return junction;
    }
};

int main(){

    ListNode* root = new ListNode(1);
    root->next = new ListNode(2);
    //root -> next -> next = root;
    //cout << detectCycle(root);
    return 0;
}