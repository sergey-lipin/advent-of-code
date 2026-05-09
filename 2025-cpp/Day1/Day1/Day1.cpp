// Day1.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include <iostream>
#include <string>

int main()
{
	// read all text from standard input
	int64_t pos = 50;
	int64_t pwd1 = 0, pwd2 = 0;
	std::string line;
	while (std::getline(std::cin, line))
	{
		if (line.length() < 2)
		{
			break;
		}
		int64_t n = atoll(line.c_str() + 1);
		switch (line[0])
		{
		case 'L':
		{
			auto r = n % 100;
			if (r > pos && pos != 0)
			{
				pwd2 += 1;
			}
			pos = (pos + 100 - r) % 100;
			pwd2 += n / 100;
			break;
		}
		case 'R':
		{
			auto r = n % 100;
			if ((r + pos) > 100)
			{
				pwd2 += 1;
			}
			pos = (pos + r)  % 100;
			pwd2 += n / 100;
			break;
		}
		}
		if (pos == 0)
		{
			pwd1 += 1;
			pwd2 += 1;
		}
		std::cout << "pos: " << pos << ", pwd1: " << pwd1 << ", pwd2: " << pwd2 << "\n";
	}

	std::cout << pwd1 << "\n";
	std::cout << pwd2 << "\n";
}

// Run program: Ctrl + F5 or Debug > Start Without Debugging menu
// Debug program: F5 or Debug > Start Debugging menu

// Tips for Getting Started: 
//   1. Use the Solution Explorer window to add/manage files
//   2. Use the Team Explorer window to connect to source control
//   3. Use the Output window to see build output and other messages
//   4. Use the Error List window to view errors
//   5. Go to Project > Add New Item to create new code files, or Project > Add Existing Item to add existing code files to the project
//   6. In the future, to open this project again, go to File > Open > Project and select the .sln file
