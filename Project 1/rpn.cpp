#include "rpn.h"

int main() {
	string test[] = {"2", "12", "6", "-", "/", "5", "3", "+", "*"}; 
	int n = 9;

	double result = rpn(test, n);
	cout << result << endl;
	
}


//2 12 6 	