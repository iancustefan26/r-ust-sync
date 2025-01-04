#include <iostream>
#include <string>
#include <map>
using namespace std;

void clear_table(bool table[]){
    for(int i = 0; i < 256; ++i)
        table[i] = 0;
}

int partitionString(string s) {
    bool table[256] = {0};
    int i = 0, length = s.size();
    int partitions = 1;
    while(i < length){
        if(table[int(s[i])] == 0)
            table[int(s[i])] = 1;
        else partitions++, clear_table(table), i--;
        i++;
    }

    return partitions;
}

int main(){
    string s = "ssssss";
    cout << partitionString(s);
    return 0;
}