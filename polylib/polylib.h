#ifndef C_LIB_H
#define C_LIB_H


enum    equationSol{
    CONSTANT_NO_SOL         = 0,
    CONSTANT_INFINITE_SOL   = 1,
    LINEAR_ONE_SOL          = 2,
    QUADRATIC_NO_SOL        = 3,
    QUADRATIC_ONE_SOL       = 4,
    QUADRATIC_TWO_SOL       = 5
};

typedef struct polynom{

    double  a;
    double  b;
    double  c;
    int     type;
    double  delta;
    double  x1;
    double  y1;
    double  x2;
    double  y2;
} polyn;

polyn   solve(double a, double b, double c);

#endif