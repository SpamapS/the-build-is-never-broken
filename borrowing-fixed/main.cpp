#include <iostream>
#include <map>
#include <string>
#include <functional>

using namespace std;

class user {
  public:
    string name;
    uint64_t rank;
    user(string name, uint64_t rank) :
      name(name), rank(rank)
  { }
};

int main() {
    user pyth (string("Pythagorus"), 100);
    user kant(string("EmanuelKant"), 200);
    map<string, user> users;
    users.insert(pair<string, user>(pyth.name, pyth));
    users.insert(pair<string, user>(kant.name, kant));
    pyth.name = string("Imposter");
    cout << " Check out these users: ";
    for (const auto &p : users) {
      cout << "{\"" << p.first << "\": {\"name\": \"" << p.second.name << "\", " << p.second.rank << "}}" << endl;
    }
}
