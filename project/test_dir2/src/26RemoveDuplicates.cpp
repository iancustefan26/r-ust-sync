#include <iostream>
#include <map>
using namespace std;

class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        map<int, int> hashMap;
        int k = 0;
        for(int i : nums)
            if(hashMap.find(nums[i]) != hashMap.end())
                hashMap[nums[i]]++;
            else hashMap.insert(pair<int, int>(nums[i], 1)), k++;
        int i = 0;
        for(auto e : hashMap)
            nums[i] = e.first, i++;
        for(i = 0; i < nums.size(); ++i)
            cout << nums[i] << " ";
        return k;
    }
};

int removeDuplicates(vector<int>& nums) {
        map<int, int> hashMap;
        int k = 0;
        for(int i = 0; i < nums.size(); ++i)
            if(hashMap.find(nums[i]) != hashMap.end())
                hashMap[nums[i]]++;
            else hashMap.insert(pair<int, int>(nums[i], 1)), k++;
        int i = 0;
        for(auto e : hashMap)
            nums[i] = e.first, i++;
        return k;
}

int main(){
    vector<int> nums;
    nums.push_back(-3);
    nums.push_back(-3);
    nums.push_back(-1);
    nums.push_back(1);
    nums.push_back(1);
    nums.push_back(2);
    nums.push_back(2);
    nums.push_back(3);
    int k = removeDuplicates(nums);

    cout << "\n" << k;
    return 0;
}