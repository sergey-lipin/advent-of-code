// Day6.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include <iostream>
#include <string>
#include <string_view>
#include <iomanip>
#include <ranges>
#include <vector>

int64_t column_to_number(const std::vector<std::string>& sheet, size_t col)
{
	int64_t result = 0;
	const size_t arg_count = sheet.size() - 1;
	for (size_t row = 0; row < arg_count; row++)
	{
		int64_t c = sheet[row][col];
		if (c < '0' || c > '9')
		{
			continue;
		}
		int64_t digit = c - '0';
		result = result * 10 + digit;
	}
	return result;
}

int main()
{
	// read all text from standard input
	std::vector<std::vector<std::string>> sheet1;
	std::vector<std::string> sheet2;
	std::string line;
	while (std::getline(std::cin, line))
	{
		if (line.empty())
		{
			break;
		}
		std::vector<std::string> row;
		for (const auto& r : std::views::split(line, ' '))
		{
			std::string s(r.begin(), r.end());
			if (s.empty())
			{
				continue;
			}
			row.push_back(s);
		}
		sheet1.push_back(row);
		sheet2.push_back(line);
	}

	if (sheet1.empty())
	{
		return 1;
	}

	const auto& op_codes = sheet1[sheet1.size() - 1];
	const size_t col_count = op_codes.size();
	const size_t arg_count = sheet1.size() - 1;
	int64_t result1 = 0;

	for (size_t col = 0; col < col_count; col++)
	{
		const auto& op_code = op_codes[col];
		if (op_code == "+")
		{
			int64_t sum = 0;
			for (size_t row = 0; row < arg_count; row++)
			{
				sum += std::stoll(sheet1[row][col]);
			}
			result1 += sum;
		}
		else if (op_code == "*")
		{
			int64_t prd = 1;
			for (size_t row = 0; row < arg_count; row++)
			{
				prd *= std::stoll(sheet1[row][col]);
			}
			result1 += prd;
		}
		else
		{
			std::cerr << "Unknown operation code: " << op_code << std::endl;
			return 1;
		}
	}

	std::cout << result1 << std::endl;

	char operation = ' ';
	const auto& operations = sheet2[sheet2.size() - 1];
	int64_t result2 = 0;
	int64_t accum = 0;

	for (size_t col = 0; col < operations.length(); col++)
	{
		const auto& op_code = operations[col];
		if (op_code == '+')
		{
			result2 += accum;
			operation = '+';
			accum = 0;
		}
		else if (op_code == '*')
		{
			result2 += accum;
			operation = '*';
			accum = 1;
		}
		int64_t col_value = column_to_number(sheet2, col);
		if (col_value == 0)
		{
			continue;
		}
		if (operation == '+')
		{
			accum += col_value;
		}
		else if (operation == '*')
		{
			accum *= col_value;
		}
	}
	result2 += accum;
	std::cout << result2 << std::endl;
	return 0;
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
