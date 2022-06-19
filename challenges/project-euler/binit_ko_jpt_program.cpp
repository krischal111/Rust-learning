#include<iostream>
#include<math.h>
#include<vector>
using namespace std;
int main()
{
    long long int n=600851475143;
    int j=2;
    vector<int> prime_factor;
    
    while(sqrt(n)>=2)
    {
        if(n%j==0)
        {
            prime_factor.push_back(j);
            n=n/j;
            j=2;
        }
        j++;
        
    }
    for(int i=0;i<prime_factor.size();i++)
    {
        cout << " " << prime_factor.at(i);
    }
    return 0;
}