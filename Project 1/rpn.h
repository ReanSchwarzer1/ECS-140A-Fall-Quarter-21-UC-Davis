#include<bits/stdc++.h>
using namespace std;

// Relevant sources/links: https://www.geeksforgeeks.org/postfix-prefix-conversion/


bool isOperator(string x)
{
	if(x=="+"||x=="-" || x=="/" || x=="*") return true;
	return false;
}


double postfixEval(string tokens[],int n) {
            reverse(tokens,tokens+n);
            stack<string>st1,st2;
            for(int i=0;i<n;i++){
               st1.push(tokens[i]);
            }
            while(st1.size()>=1){
                
            while(!st1.empty()&&(st1.top()!="+"&&st1.top()!="-"&&st1.top()!="/"&&st1.top()!="*")){
                st2.push(st1.top());
                    st1.pop();
                }
                if(st1.empty())return stod(st2.top());
                double a=0,b=0;
                string sign=st1.top();
                st1.pop();
                b=stod(st2.top());
                st2.pop();
                a=stod(st2.top());
                st2.pop();
                double z=0;
                if(sign=="+")z=a+b;
                else if(sign=="-")z=a-b;
                else if(sign=="/")z=a/b;
                else if(sign=="*")z=a*b;
                string g=to_string(z);
                st2.push(g);
            }
             double ret=stod(st2.top());
            st2.pop();
            return ret;
        }

string posttoprefix(string post_exp[],int length)
{
	stack<string> s;

	for (int i = 0; i < length; i++) {

		if (isOperator(post_exp[i])) {
			string op1 = s.top();
			s.pop();
			string op2 = s.top();
			s.pop();
			string temp = post_exp[i] + op2+"_"+op1 ;
				
			s.push(temp);
		}

		else {

			s.push(post_exp[i]);
		}
	}

	string ans = "";
	while (!s.empty()) {
		ans += s.top();
		s.pop();
	}
	return ans;
}




void printprefix(string ans){

stack<int> space;
int countspace=0;
space.push(countspace);
int consoperand=0;
for(int i=0;i<ans.length();i++){

    if(ans[i]=='+' || ans[i]=='-' || ans[i]=='*' || ans[i] =='/'){
        consoperand=0;
      for(int k=1;k<=countspace;k++){
          cout<<" ";
      }
         cout<<"("<<ans[i]<<endl;
    
      countspace+=2;
      space.push(countspace);
    }
    else
    {
        string temp="";
     while(i<ans.length() && ans[i]!='_'){
       
      temp+=ans[i];
      i++;
     }

        consoperand++;
    for(int k=1;k<=countspace;k++){
          cout<<" ";
      }
     cout<<temp<<endl;
     if(consoperand>=2){
        for(int j=1;j<=2;j++){
        space.pop();
          countspace=space.top();
           for(int k=1;k<=countspace;k++){
            cout<<" ";}
          
          cout<<")"<<endl;
        }
      

      }

     }

    }



}


double rpn(string strs[], int n) 
{
	double result = 0.0;

    cout<<"Ans: of Part2: \n"; 
    string prefix= posttoprefix(strs,n);
    printprefix(prefix);
    cout<<"\nAns: Of Part1: \n";   
	return postfixEval(strs,n);

	return result;
}

