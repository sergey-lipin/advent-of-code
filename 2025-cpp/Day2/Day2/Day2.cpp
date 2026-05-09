// Day2.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include <iostream>
#include <string>
#include <string_view>
#include <iomanip>
#include <ranges>
#include <vector>

int64_t count_invalid_ids(const std::vector<int64_t>& bounds, bool find_all)
{
	int64_t total = 0;
	for (int64_t i = bounds[0]; i <= bounds[1]; i++)
	{
		auto s = std::to_string(i);
		bool is_invalid = false;
		size_t start = 2;
		size_t end = find_all ? s.length() : 2;

		for (size_t n = start; n <= end; n++)
		{
			if (s.length() % n != 0)
			{
				continue;
			}
			auto part_length = s.length() / n;
			std::string_view sv = s;
			bool has_repeatitions = true;
			for (size_t part = 1; part < n; part++)
			{
				if (sv.substr(0, part_length) != sv.substr(part * part_length, part_length))
				{
					has_repeatitions = false;
					break;
				}
			}
			if (has_repeatitions)
			{
				is_invalid = true;
				break;
			}
		}

		if (is_invalid)
		{
			total += i;
		}
	}
	return total;
}

int main()
{
	// read all text from standard input
	std::string line;
	if (!std::getline(std::cin, line))
	{
		return 1;
	}
	int64_t total1 = 0, total2 = 0;
	for (const auto& r : std::views::split(line, ','))
	{
		std::vector<int64_t> bounds;
		for (const auto& b : std::views::split(std::string_view(r), '-'))
		{
			bounds.push_back(std::stoll(std::string(std::string_view(b))));
		}

		if (bounds.size() == 2)
		{
			total1 += count_invalid_ids(bounds, false);
			total2 += count_invalid_ids(bounds, true);
		}
	}

	std::cout << total1 << std::endl;
	std::cout << total2 << std::endl;
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
