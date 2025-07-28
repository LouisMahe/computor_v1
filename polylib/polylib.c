
#include "polylib.h"
#include "math.h"

polyn   solve_const(double c)
{
    polyn   s;
    s.c = c;
    if (c == 0.0)
        s.type = CONSTANT_INFINITE_SOL;
    else
        s.type = CONSTANT_NO_SOL;
    return s;
}

polyn   solve_linear(double b, double c) //bx+c=0
{
    polyn   s;
    s.b = b;
    s.c = c;
    s.type = LINEAR_ONE_SOL;
    s.x1 = (-c/b);
    s.x2 = s.x1;
    return s;
}

polyn   solve_quadra(double a, double b, double c)
{
    
    polyn   s;
    s.a = a;
    s.b =b ;
    s.c = c;
    s.delta = b*b - 4.0*a*c;
    double  ainv = 1 / (2*a);
    if (s.delta < 0.0)
    {
        s.type = QUADRATIC_NO_SOL;
        double root = sqrt(-s.delta);
        s.x1 = -b * ainv;
        s.x2 = s.x1;
        s.y1 = root*ainv;
        s.y2 = -s.y1;

    }
    else if (s.delta == 0.0)
    {
        s.type = QUADRATIC_ONE_SOL;
        s.x1 = -b*ainv;
        s.x2 = s.x1;
        
    }
    else{
        s.type = QUADRATIC_TWO_SOL;
        double root = sqrt(s.delta);
        if (ainv > 0.0)
        {
            s.x1 = (-b-root)*ainv;
            s.x2 = (-b+root)*ainv;
        }
        else{
            s.x2 = (-b-root)*ainv;
            s.x1 = (-b+root)*ainv;
        }
    }
    return s;
}

polyn   solve(double a, double b, double c) //ax^2+bx+c=0
{
    if(a == 0.0 && b == 0.0)
        return solve_const(c);
    else if (a == 0.0)
        return solve_linear(b,c);
    else
        return solve_quadra(a,b,c);
}

