#include <iostream>
#include <map>
using namespace std;

struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

bool hasCycle(ListNode *head) {
    if(head == nullptr)
        return false;
    ListNode* slow, *fast;
    slow = fast = head;
    while(fast -> next != nullptr && fast -> next -> next != nullptr){
        fast = fast -> next -> next;
        slow = slow -> next;
        if(slow == fast)
            return true;
    }

    return false;

}

int main(){
    ListNode* root = new ListNode(1);
    root->next = new ListNode(2);
    root -> next -> next = root;
    cout << hasCycle(root);
    return 0;
}