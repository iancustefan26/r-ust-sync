#include <iostream>

using namespace std;


int lengthOfLastWord(string s) {
    int end = s.length() - 1;

    while (end >= 0 && s[end] == ' ') {
        end--;
    }

    int start = end;
    while (start >= 0 && s[start] != ' ') {
        start--;
    }

    return end - start;        
}
int main(){
    string s = "luffy is";
    cout << lengthOfLastWord(s);
    return 0;
}