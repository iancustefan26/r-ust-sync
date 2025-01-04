#include <iostream>
#include <vector>
using namespace std;

class Solution {
public:
    int findCycle(vector<int>& nums){
        int slow, fast;
        slow = fast = nums[0];
        bool started = false;
        while(slow != fast && started != false)
            started = true,
            slow = nums[slow],
            fast = nums[nums[fast]];
        
        return slow;
    }
    int findDuplicate(vector<int>& nums) {
        int cycle = findCycle(nums);
        int start = nums[0];
        while(cycle != start)
            cycle = nums[cycle],
            start = nums[start];
        
        return cycle;
    }
};
int main(){

    return 0;
}