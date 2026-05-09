// Day7.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include <iostream>
#include <string>
#include <string_view>
#include <iomanip>
#include <ranges>
#include <vector>
#include <unordered_map>

void place_char(std::vector<std::string>& diagram, int64_t row, int64_t col, char c)
{
	if (row < 0 || col < 0)
	{
		return;
	}
	if (row >= (int64_t)diagram.size() || col >= (int64_t)diagram[row].size())
	{
		return;
	}
	diagram[row][col] = c;
}

int64_t do_step(std::vector<std::string>& diagram, size_t row, size_t& start_col)
{
	int64_t split_count = 0;
	for (size_t col = 0; col < diagram[row].size(); col++)
	{
		char cur = diagram[row][col];
		char prev = diagram[row - 1][col];
		if (prev == '|' || prev == 'S')
		{
			if (prev == 'S')
			{
				start_col = col;
			}
			if (cur == '.')
			{
				place_char(diagram, row, col, '|');
			}
			else if (cur == '^')
			{
				place_char(diagram, row, col - 1, '|');
				place_char(diagram, row, col + 1, '|');
				split_count++;
			}
		}
	}
	return split_count;
}

int64_t traverse_diagram(const std::vector<std::string>& diagram, int64_t row, int64_t col, std::unordered_map<int64_t, int64_t>& visited)
{
	int64_t key = (row << 32) | col;
	if (visited.contains(key))
	{
		return visited[key];
	}
	int64_t result = 0;
	if (row >= 0 && col >= 0 && row < (int64_t)diagram.size() && col < (int64_t)diagram[row].size())
	{
		char cur = diagram[row][col];
		if (cur == '|' || cur == 'S')
		{
			if (row == diagram.size() - 1)
			{
				result = 1;
			}
			else
			{
				result = traverse_diagram(diagram, row + 1, col, visited);
			}
		}
		else if (cur == '^')
		{
			result = traverse_diagram(diagram, row, col - 1, visited) + traverse_diagram(diagram, row, col + 1, visited);
		}
	}
	visited[key] = result;
	return result;
}

int main()
{
	// read all text from standard input
	std::vector<std::string> diagram;
	std::string line;
	while (std::getline(std::cin, line))
	{
		if (line.empty())
		{
			break;
		}
		diagram.push_back(line);
	}

	if (diagram.empty())
	{
		return 1;
	}

	size_t start_col = 0;
	int64_t result1 = 0;
	for (size_t row = 1; row < diagram.size(); row++)
	{
		int64_t splits = do_step(diagram, row, start_col);
		result1 += splits;
	}
	std::cout << "Result: " << result1 << std::endl;

	std::unordered_map<int64_t, int64_t> visited;
	int64_t result2 = traverse_diagram(diagram, 0, start_col, visited);
	std::cout << "Result2: " << result2 << std::endl;

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
