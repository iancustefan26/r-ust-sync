#include <iostream>
#include <map>
using namespace std;

class WordDictionary {
    unordered_map<string, bool> hashMap;
private:
    int find(string word, const char s) const{
        int n = word.length();
        for(int i = 0; i < n; ++i)
            if(word[i] == '.')
                {cout << word << " (find) " << i << "\n"; return i;}
        return -1;
    }
    int find(string word, const char* s) const{
        int n = word.length();
        for(int i = 0; i < n - 1; ++i)
            if(word[i] == s[0] && word[i] == s[1])
                return i;
        return -1;
    }
public:
    WordDictionary() {
        this -> hashMap.reserve(10000);
    }
    void addWord(string word) {
        this -> hashMap.insert(pair<string, bool>(word, true));
    }
    
    bool search(string word) const{ 
        int pos = word.find("..");
        if(pos != string::npos){
            for(char c = 'a'; c <= 'z'; ++c)
                for(char w = 'a';c <= 'z'; ++w){
                    word[pos] = c, word[pos + 1] = w;
                    cout << pos << " (.) " << word << "\n";
                    if(hashMap.find(word) != hashMap.end())
                        return true;
                }
        }
        pos = word.find(".");
        if(pos != string::npos){
            for(char c = 'a'; c <= 'z'; ++c){
                    word[pos] = c;
                    cout << pos << " (..) " << word << "\n";
                    if(hashMap.find(word) != hashMap.end())
                        return true;
                }
        }
        if(hashMap.find(word) != hashMap.end())
            return true;
        return false;
    }
};

int main(){


    return 0;
}