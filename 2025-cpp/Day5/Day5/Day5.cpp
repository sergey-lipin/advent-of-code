// Day5.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include <iostream>
#include <string>
#include <string_view>
#include <iomanip>
#include <ranges>
#include <vector>
#include <algorithm>

bool is_fresh(const std::vector<std::vector<int64_t>>& ranges, int64_t id)
{
	for (const auto& bounds : ranges)
	{
		if (id >= bounds[0] && id <= bounds[1])
		{
			return true;
		}
	}
	return false;
}

int main()
{
	// read all text from standard input
	std::string line;
	bool reading_ranges = true;
	std::vector<std::vector<int64_t>> ranges;
	int64_t total = 0;

	while (std::getline(std::cin, line))
	{
		if (line.empty())
		{
			if (reading_ranges)
			{
				reading_ranges = false;
				continue;
			}
			break;
		}

		if (reading_ranges)
		{
			std::vector<int64_t> bounds;
			for (const auto& b : std::views::split(std::string_view(line), '-'))
			{
				bounds.push_back(std::stoll(std::string(std::string_view(b))));
			}

			ranges.push_back(bounds);
		}
		else
		{
			if (is_fresh(ranges, std::stoll(line)))
			{
				++total;
			}
		}
	}

	std::cout << total << std::endl;

	// merge overlapping ranges
	std::sort(ranges.begin(), ranges.end(), [](const auto& a, const auto& b) {
		return a[0] < b[0];
	});

	std::vector<std::vector<int64_t>> merged;
	for (const auto& range : ranges)
	{
		if (merged.empty() || merged.back()[1] < range[0] - 1)
		{
			merged.push_back(range);
		}
		else
		{
			merged.back()[1] = std::max(merged.back()[1], range[1]);
		}
	}
	ranges = merged;

	int64_t covered = 0;
	for (const auto& range : ranges)
	{
		covered += range[1] - range[0] + 1;
	}
	std::cout << covered << std::endl;
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
