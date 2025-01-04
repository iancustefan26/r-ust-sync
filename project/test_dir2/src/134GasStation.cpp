#include <iostream>
#include <vector>
using namespace std;

class Solution {
public:
    int canCompleteCircuit(vector<int>& gas, vector<int>& cost) {
        vector<int> difs;
        int sum = 0;
        int length = gas.size();
        difs.reserve(length);
        for(int i = 0; i < length; ++i){
            sum += gas[i] - cost[i];
            difs.push_back(gas[i] - cost[i]);
        }
        if(sum < 0)
            return -1;
        
        vector<int> sums;
        sums.reserve(length);
        int maxSum = -1;
        int maxStart = -1;
        for(int i = 0; i < length; ++i){
            int sum = 0;
            int start = i;
            while(sum >= 0 && i < length){
                sum += difs[i];
                if(sum < 0){
                    sum -= difs[i];
                    if(sum > maxSum){
                        maxSum = sum;
                        maxStart = start;
                        if(i < length - 1)
                            ++i;
                        break;
                    }
                }
                else ++i;
            }
            if (sum > maxSum)
                maxSum = sum, maxStart = start;
        }

        return maxStart;
    }
};

int canCompleteCircuit(vector<int>& gas, vector<int>& cost) {
        vector<int> difs;
        int sum = 0;
        int length = gas.size();
        if(length == 0)
            return -1;
        difs.reserve(length);
        for(int i = 0; i < length; ++i){
            sum += gas[i] - cost[i];
            difs.push_back(gas[i] - cost[i]);
        }
        if(sum < 0)
            return -1;
        
        vector<int> sums;
        sums.reserve(length);
        sums[0] = difs[0];
        for(int i = 1; i < length; ++i){
            if(sums[i-1] < 0)
                sums[i] = difs[i];
            else sums[i] = sums[i-1] + difs[i];
        }
        for(int i = 0; i < length; ++i)
            cout << sums[i] << " ";
        cout << "\n";
        int max = sums[0];
        int pos = 0;
        for(int i = 1; i < length; ++i)
            if(sums[i] > max)
                max = sums[i], pos = i;
        
        int start = 0;
        while(sums[start] < 0 && start < length)
            start++;
        for(int i = 1; i < length; ++i)
            if(sums[i] < 0)
                start = i + 1;
        
        return start;
    }

int main(){
    vector<int> gas1;
    vector<int> cost1;
    for(int i = 1; i <= 5; ++i)
        gas1.push_back(i);
    cost1.push_back(3);
    cost1.push_back(4);
    cost1.push_back(5);
    cost1.push_back(1);
    cost1.push_back(2);
    std::cout << canCompleteCircuit(gas1, cost1);
    return 0;
}