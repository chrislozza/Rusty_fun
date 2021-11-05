#include <iostream>
#include <cstdlib>
#include <unistd.h>

using namespace std;

int main()
{
  cout << "Hello World" << endl;

  for (int i = 0; i < 120; ++i) {
    cout << "hello " << i << endl;
    sleep(1);
  }

  return 0;
}
