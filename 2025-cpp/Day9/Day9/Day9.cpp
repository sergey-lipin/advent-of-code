// Day9.cpp : This file contains the 'main' function. Program execution begins and ends there.
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
#include <mdspan>

typedef struct point_t
{
	int64_t x;
	int64_t y;
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

		if (coords.size() != 2)
		{
			std::cerr << "Invalid input line: " << line << std::endl;
			return 1;
		}

		point_t point;
		point.x = coords[0];
		point.y = coords[1];
		points.push_back(point);
	}

	std::map<int64_t, std::vector<size_t>> area_map;

	for (size_t i = 0; i < points.size(); i++)
	{
		for (size_t j = i + 1; j < points.size(); j++)
		{
			int64_t area = (std::abs(points[i].x - points[j].x) + 1) * (std::abs(points[i].y - points[j].y) + 1);
			area_map[area].push_back(i);
			area_map[area].push_back(j);
		}
	}

	std::cout << "Max area: " << area_map.rbegin()->first << std::endl;

	std::vector<point_t> perimeter;

	point_t last_point = points[points.size() - 1];
	for (const auto& point : points)
	{
		if (point.y == last_point.y)
		{
			int64_t x_start = std::min(point.x, last_point.x);
			int64_t x_end = std::max(point.x, last_point.x);
			int64_t y = point.y;
			for (int64_t x = x_start; x < x_end; x++)
			{
				perimeter.push_back({ x, y });
			}
		}
		else if (point.x == last_point.x)
		{
			int64_t y_start = std::min(point.y, last_point.y);
			int64_t y_end = std::max(point.y, last_point.y);
			int64_t x = point.x;
			for (int64_t y = y_start; y < y_end; y++)
			{
				perimeter.push_back({ x, y });
			}
		}
		last_point = point;
	}

	for (auto it = area_map.crbegin(); it != area_map.crend(); it++)
	{
		if (it->second.size() < 2)
		{
			continue;
		}

		size_t index1 = it->second[0];
		size_t index2 = it->second[1];

		// determine bounding box
		int64_t min_x = std::min(points[index1].x, points[index2].x);
		int64_t max_x = std::max(points[index1].x, points[index2].x);
		int64_t min_y = std::min(points[index1].y, points[index2].y);
		int64_t max_y = std::max(points[index1].y, points[index2].y);

		bool is_within_area = true;
		for (const auto& point : perimeter)
		{
			if (point.x > min_x && point.x < max_x && point.y > min_y && point.y < max_y)
			{
				is_within_area = false;
				break;
			}
		}
		if (is_within_area)
		{
			std::cout << "Largest within area: " << it->first << std::endl;
			break;
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
