#include <iostream>
#include <vector>

using namespace std;

int candy(vector<int>& ratings) {
    int length = ratings.size();
    int sum = 0;
    vector<int> candies;
    for(int i = 0; i < length; ++i)
        candies.push_back(1);
    for(int i = 1; i < length; ++i)
        if(ratings[i] > ratings[i - 1])
            candies[i] = candies[i - 1] + 1;
    for(int i = length - 2; i >= 0; --i)
        if(ratings[i] > ratings[i + 1])
            candies[i] = max(candies[i], candies[i + 1] + 1);

    for(int i = 0; i < length; ++i)
        sum += candies[i];
    
    return sum;
}


int main(){
    vector<int> ratings;
    ratings.push_back(1);
    ratings.push_back(0);
    ratings.push_back(2);

    cout << candy(ratings);
    return 0;
}