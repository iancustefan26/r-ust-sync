#include <iostream>

using namespace std;

void merge(vector<int>& nums1, int m, vector<int>& nums2, int n) {
        int i = m - 1, j = n - 1;
        int p = n + m - 1;
        while (i >= 0 && j >= 0) {
            if(nums1[i] >= nums2[j]){
                nums1[p] = nums1[i];
                --i;
                cout << "Schimbat: " << nums1[i + 1] << "\n";
            } else{
                nums1[p] = nums2[j];
                --j;
                cout << "Schimbat: " << nums2[j + 1] << "\n";
            }
            --p;
        }
        while(i >= 0)
            nums1[p--] = nums1[i], --i;
        while(j >= 0)
            nums1[p--] = nums2[j], --j;
        for(int x : nums1)
            cout << x << " ";
}

int main(){
    vector<int> x, y;
    x.push_back(1);
    x.push_back(2);
    x.push_back(3);
    x.push_back(0);
    x.push_back(0);
    x.push_back(0);
    y.push_back(2);
    y.push_back(5);
    y.push_back(6);
    merge(x, 3, y, 3);
    return 0;
}