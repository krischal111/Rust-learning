#include <iostream>
#include <math.h>
#include <vector>
#include <cctype>

typedef uint64_t u64;


using namespace std;
int main(){
    vector <u64> prime_list = {};
    u64 n = 600851475143;
    auto a = n;
    vector <u64> prime_factors = {};

    for(u64 i = 2; i <= (u64)sqrt((double)a); i++)
    {
        bool is_prime = true;

        for(auto prev_prime = prime_list.begin(); prev_prime != prime_list.end(); prev_prime++)
        {
            auto pp = *prev_prime;
            if( i % pp == 0)
            {
                is_prime = false;
                break;
            }
            if(pp*pp >= i)
            {
                break;
            }
        }

        if(is_prime)
        {
            prime_list.push_back(i);
            if (a % i == 0)
            {
                auto b = a;
                do
                {
                    prime_factors.push_back(i);
                    b /= i;
                } while (b % i == 0);
                a = b;
            }
        }
    }
    std::cout << endl;
    for(int i=0; i < prime_factors. size(); i++) std::cout << prime_factors.at(i);
    
    return 0;
}