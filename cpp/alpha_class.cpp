#include <iostream>
using namespace std;

struct alpha {
    int a;

    alpha(int aa = 0){
        a = aa;
    };

    alpha greater(alpha x)
    {
        if (a > x.a) {
            return *this;
        }
        else {
            return x;
        }
    }
    // friend int main();
};

int main()
{
    alpha b(20), c;
    c.a = 10;
    alpha d;
    d = b.greater(c);
    cout << "The greater value is " << d.a;
    return 0;
}