// Day3.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include <iostream>
#include <string>

int main()
{
	// read all text from standard input
	int64_t joltage = 0;
	std::string line;
	while (std::getline(std::cin, line))
	{
		if (line.length() < 2)
		{
			break;
		}

		std::cout << "line: " << line << "\n";
		std::string max_joltage_str;
		size_t to_fill = 12;
		size_t start = 0;
		size_t end = line.length() - to_fill;

		while (to_fill > 0)
		{
			int64_t max_digit = line[start] - '0';
			size_t max_index = start;
			for (size_t i = start + 1; i <= end; i++)
			{
				int64_t cur = line[i] - '0';
				if (cur > max_digit)
				{
					max_digit = cur;
					max_index = i;
				}
			}
			max_joltage_str += static_cast<char>(max_digit + '0');
			to_fill--;
			start = max_index + 1;
			end++;
		}

		int64_t max_joltage = std::stoll(max_joltage_str);
		std::cout << "max_joltage: " << max_joltage << "\n";
		joltage += max_joltage;
	}

	std::cout << joltage << "\n";
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
