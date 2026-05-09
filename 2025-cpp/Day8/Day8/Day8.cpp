// Day8.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include <iostream>
#include <string>
#include <string_view>
#include <iomanip>
#include <ranges>
#include <vector>
#include <algorithm>
#include <map>
#include <cmath>

typedef struct point_t
{
	int64_t x;
	int64_t y;
	int64_t z;
} point_t;

int main()
{
	// read all text from standard input
	std::string line;
	std::vector<point_t> points;

	while (std::getline(std::cin, line))
	{
		if (line.empty())
		{
			break;
		}

		std::vector<int64_t> coords;
		for (const auto& b : std::views::split(std::string_view(line), ','))
		{
			coords.push_back(std::stoll(std::string(std::string_view(b))));
		}

		if (coords.size() != 3)
		{
			std::cerr << "Invalid input line: " << line << std::endl;
			return 1;
		}

		point_t point;
		point.x = coords[0];
		point.y = coords[1];
		point.z = coords[2];

		points.push_back(point);
	}

	std::vector<std::vector<size_t>> circuits;
	std::map<size_t, size_t> circuit_index;
	std::map<double, std::vector<size_t>> distance_map;

	for (size_t i = 0; i < points.size(); i++)
	{
		circuits.push_back({ i });
		circuit_index[i] = i;
		for (size_t j = i + 1; j < points.size(); j++)
		{
			int64_t distance_squared = 
				(points[i].x - points[j].x) * (points[i].x - points[j].x) +
				(points[i].y - points[j].y) * (points[i].y - points[j].y) +
				(points[i].z - points[j].z) * (points[i].z - points[j].z);
			double distance = std::sqrt(distance_squared);
			distance_map[distance].push_back(i);
			distance_map[distance].push_back(j);
		}
	}

	int circuit_count = points.size() >= 1000 ? 1000 : 10;

	for (const auto& [distance, indices] : distance_map)
	{
		size_t i = indices[0];
		size_t j = indices[1];
		size_t circuit_i = circuit_index[i];
		size_t circuit_j = circuit_index[j];
		if (circuit_i != circuit_j)
		{
			// merge circuits
			circuits[circuit_i].insert(
				circuits[circuit_i].end(),
				circuits[circuit_j].begin(),
				circuits[circuit_j].end());
			// update circuit index
			for (const auto& index : circuits[circuit_j])
			{
				circuit_index[index] = circuit_i;
			}
			// clear merged circuit
			circuits[circuit_j].clear();

			if (circuits[circuit_i].size() == points.size())
			{
				std::cout << "Part 2:" << points[i].x * points[j].x << std::endl;
			}
		}
		--circuit_count;
		if (circuit_count == 0)
		{
			std::vector<size_t> circuit_sizes;
			for (const auto& circuit : circuits)
			{
				if (!circuit.empty())
				{
					circuit_sizes.push_back(circuit.size());
				}
			}

			std::sort(circuit_sizes.begin(), circuit_sizes.end(), std::greater<size_t>());
			int64_t result = 1;
			for (size_t i = 0; i < 3; i++)
			{
				result *= circuit_sizes[i];
			}
			std::cout << "Part 1:" << result << std::endl;
		}
	}

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
