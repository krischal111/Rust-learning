#include <iostream>
using namespace std;

class alpha {
    public:
    class beta {
        public:
        int b;
    };
    int a;
    public:
    
    alpha(){
        a = 0;
        cout << "Default Constructor called " << endl;
    }

    alpha(int aa)
    {
        a = aa;
    }

    static alpha greater(alpha x, alpha y)
    {
        if (x.a > y.a) {
            return x;
        }
        else
        {
            return y;
        }
    }

    int get_a()
    {
        return a;
    }
};

int main()
{
    // alpha x(5), y(6);
    // alpha z;
    // z = z.greater(x,y);
    cout << "The greater class is " << alpha::greater(alpha(5),alpha(6)).get_a() << endl;
    alpha::beta b;
    b.b = 10;
    cout << "The value of inner class is " << b.b << endl;
    return 0;
}