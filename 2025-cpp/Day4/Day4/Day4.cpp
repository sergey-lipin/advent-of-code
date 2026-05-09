// Day4.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include <iostream>
#include <string>
#include <vector>

bool is_roll(const std::vector<std::string>& plan, int64_t row, int64_t col)
{
	if (row < 0 || col < 0)
	{
		return false;
	}
	if (row >= (int64_t)plan.size() || col >= (int64_t)plan[row].size())
	{
		return false;
	}
	if (plan[row][col] == '@')
	{
		return true;
	}
	return false;
}

typedef struct
{
	int64_t dx;
	int64_t dy;
} delta_t;

static const std::vector<delta_t> deltas = {
	{ -1, -1 }, { 0, -1 }, { 1, -1 },
	{ -1,  0 },            { 1,  0 },
	{ -1,  1 }, { 0,  1 }, { 1,  1 },
};

bool is_accessible(const std::vector<std::string>& plan, int64_t row, int64_t col)
{
	if (!is_roll(plan, row, col))
	{
		return false;
	}
	int num_rolls = 0;
	for (const auto& delta : deltas)
	{
		if (is_roll(plan, row + delta.dy, col + delta.dx))
		{
			num_rolls++;
			if (num_rolls >= 4)
			{
				return false;
			}
		}
	}
	return true;
}

std::vector<delta_t> get_accessible(const std::vector<std::string>& plan)
{
	std::vector<delta_t> accessible;

	for (int64_t row = 0; row < (int64_t)plan.size(); row++)
	{
		for (int64_t col = 0; col < (int64_t)plan[row].size(); col++)
		{
			if (is_accessible(plan, row, col))
			{
				accessible.push_back({col, row});
			}
		}
	}

	return accessible;
}

void print_plan(const std::vector<std::string>& plan)
{
	for (const auto& line : plan)
	{
		std::cout << line << std::endl;
	}
}

int main()
{
	// read all text from standard input
	std::vector<std::string> plan;
	std::string line;
	while (std::getline(std::cin, line))
	{
		if (line.empty())
		{
			break;
		}
		plan.push_back(line);
	}

	auto accessible = get_accessible(plan);

	std::cout << "Number of accessible locations: " << accessible.size() << std::endl;

	// print_plan(plan);

	size_t removed = 0;
	while (accessible.size() > 0)
	{
		for (const auto& loc : accessible)
		{
			plan[loc.dy][loc.dx] = '.';
		}
		// std::cout << "After removing " << accessible.size() << " rolls:" << std::endl;
		// print_plan(plan);
		removed += accessible.size();
		accessible = get_accessible(plan);
	}

	std::cout << "Number of removed rolls: " << removed << std::endl;
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
