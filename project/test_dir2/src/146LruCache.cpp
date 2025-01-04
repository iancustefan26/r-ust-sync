#include <iostream>
#include <map>
#include <queue>

class LRUCache {
    int capacity;
    std::unordered_map<int, int> cache;
    std::deque<int> LRUkeys;
public:
    LRUCache(int capacity) {
        if(capacity >= 0)
            cache.reserve(capacity),
            this -> capacity = capacity;
        else throw std::invalid_argument("Capacity must be positive.");
    }
    
    int get(int key) {
        auto it = cache.find(key);
        if (it != cache.end()){
            std::cout << "get(" << key << ") : value " << it->second << "\n";
            LRUkeys.erase(std::remove(LRUkeys.begin(), LRUkeys.end(), key), LRUkeys.end());
            LRUkeys.push_back(key);
            return it->second;
        }
        std::cout << "Key " << key << " not found (returned -1)\n";
        return -1;
    }

    void put(int key, int value) {
        auto it = cache.find(key);
        if (it != cache.end()){
            std::cout << "Updated key " << key << "\n";
            cache[key] = value;
            LRUkeys.erase(std::remove(LRUkeys.begin(), LRUkeys.end(), key), LRUkeys.end());
            LRUkeys.push_back(key);
        }
        else if(cache.size() < this -> capacity){
            cache.insert(std::pair<int, int>(key, value));
            std::cout << "Inserted (key, value) : (" << key << " " << value << ")\n";
            LRUkeys.push_back(key); 
        }
        else{
            std::cout << "Limit exceeded -- ";
            int evict_key = LRUkeys.front();
            LRUkeys.pop_front();
            cache.erase(evict_key);
            cache.insert(std::pair<int, int>(key, value));
            std::cout << "Removed key " << evict_key << " Inserted (key, value) : (" << key << " " << value << ")\n";
        }
    }
};


int main(){
    LRUCache* lrucache = new LRUCache(2);
    lrucache->put(1, 1);
    lrucache->put(2, 2);
    lrucache->get(1);
    lrucache->put(3, 3);
    lrucache->get(2);
    lrucache->put(4, 4);
    lrucache->get(1);
    lrucache->get(3);
    lrucache->get(4);
    return 0;
}