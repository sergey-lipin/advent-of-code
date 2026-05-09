// Day10.cpp : This file contains the 'main' function. Program execution begins and ends there.
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
#include <bitset>
#include <unordered_map>
#include <deque>
#include <functional>

int64_t bfs1(int64_t start, int64_t target, const std::vector<int64_t>& wiring_bits)
{
	std::unordered_map<int64_t, int64_t> distances;
	distances[start] = 0;
	std::vector<int64_t> to_visit;
	to_visit.push_back(start);
	while (!to_visit.empty())
	{
		int64_t current = to_visit.front();
		to_visit.erase(to_visit.begin());
		if (current == target)
		{
			return distances[current];
		}
		for (const auto& wiring : wiring_bits)
		{
			int64_t next = current ^ wiring;
			if (distances.find(next) == distances.end())
			{
				distances[next] = distances[current] + 1;
				to_visit.push_back(next);
			}
		}
	}
	return -1;
}

void print_counters(const std::vector<int16_t>& counters)
{
	for (size_t i = 0; i < counters.size(); i++)
	{
		if (i > 0)
		{
			std::cout << ",";
		}
		std::cout << counters[i];
	}
	std::cout << "\n";
}

int64_t min_num_presses(const std::vector<int16_t>& start, const std::vector<int16_t>& target, const std::vector<std::vector<int16_t>>& wirings, std::unordered_map<std::string, int64_t>& memo)
{
	// Each wiring is a vector of increments, wirings are sorted by total increments descending
	// We need to find the shortest sequence of wirings that transforms start to target

	std::string memo_key;
	for (auto v : target)
	{
		if (memo_key.length() > 0)
		{
			memo_key += ",";
		}
		memo_key += std::to_string(v);
	}
	if (memo.find(memo_key) != memo.end())
	{
		return memo[memo_key];
	}

	std::vector<size_t> target_indices;
	for (size_t i = 0; i < target.size(); i++)
	{
		if (target[i] > 0)
		{
			target_indices.push_back(i);
		}
	}
	if (target_indices.empty())
	{
		memo[memo_key] = 0;
		return 0;
	}
	std::sort(target_indices.begin(), target_indices.end(), [&target](size_t a, size_t b) {
		return target[a] < target[b];
		});

	std::vector<size_t> remaining_wirings;
	for (size_t w = 0; w < wirings.size(); w++)
	{
		remaining_wirings.push_back(w);
	}

	// For the first target index, find all wirings that increment it
	auto idx = target_indices[0];
	std::vector<size_t> wirings_for_index;
	for (auto w : remaining_wirings)
	{
		if (wirings[w][idx] == 1)
		{
			wirings_for_index.push_back(w);
		}
	}
	for (auto w : wirings_for_index)
	{
		std::erase(remaining_wirings, w);
	}

	int64_t total = target[idx];
	std::vector<std::vector<size_t>> all_combinations;

	// Find all combinations of these wirings that sum to the target counter
	std::function<void(size_t, int16_t, std::vector<size_t>&)> find_combinations;
	find_combinations = [&](size_t start_idx, int16_t current_sum, std::vector<size_t>& current_combination) {
		if (current_sum == target[idx])
		{
			all_combinations.push_back(current_combination);
			return;
		}
		if (current_sum > target[idx] || start_idx >= wirings_for_index.size())
		{
			return;
		}
		// Include the wiring (and allow using it again)
		current_combination.push_back(wirings_for_index[start_idx]);
		find_combinations(start_idx, current_sum + 1, current_combination);
		current_combination.pop_back();
		// Move to next wiring
		find_combinations(start_idx + 1, current_sum, current_combination);
		};

	std::vector<size_t> current_combination;
	find_combinations(0, 0, current_combination);
	if (all_combinations.empty())
	{
		memo[memo_key] = -1;
		return -1;
	}

	int64_t min_total = INT64_MAX;
	for (const auto& combination : all_combinations)
	{
		// Apply the combination to the target counters
		auto new_target = target;
		bool is_valid = true;
		for (auto w : combination)
		{
			for (size_t i = 0; i < target.size(); i++)
			{
				new_target[i] -= wirings[w][i];
				if (new_target[i] < 0)
				{
					is_valid = false;
					break;
				}
			}
		}
		if (!is_valid)
		{
			continue;
		}

		std::vector<std::vector<int16_t>> new_wirings;
		for (auto w : remaining_wirings)
		{
			new_wirings.push_back(wirings[w]);
		}
		auto x = min_num_presses(start, new_target, new_wirings, memo);
		if (x != -1 && x < min_total)
		{
			min_total = x;
		}
	}
	
	if (min_total == INT64_MAX)
	{
		memo[memo_key] = -1;
		return -1;
	}

	memo[memo_key] = total + min_total;
	return total + min_total;
}

int main()
{
	// read all text from standard input
	std::string line;

	int64_t result1 = 0, result2 = 0;

	int16_t max_counter = 0;
	size_t max_counter_num = 0;

	while (std::getline(std::cin, line))
	{
		if (line.empty())
		{
			break;
		}

		std::vector<std::string_view> words;
		for (const auto& b : std::views::split(std::string_view(line), ' '))
		{
			words.push_back(std::string_view(b));
		}

		int64_t target = 0;
		auto bits = words[0].substr(1, words[0].size() - 2);
		size_t num_devices = bits.size();
		for (size_t i = 0; i < num_devices; i++)
		{
			if (bits[i] == '#')
			{
				target |= 1i64 << i;
			}
		}

		std::vector<int64_t> wiring_bits;
		std::vector<std::vector<int16_t>> wirings;
		for (size_t i = 1; i < words.size() - 1; i++)
		{
			int64_t target = 0;
			std::vector<int16_t> wiring(num_devices, 0);
			auto bits = words[i].substr(1, words[i].size() - 2);

			for (const auto& b : std::views::split(bits, ','))
			{
				auto pow = std::stoll(std::string(std::string_view(b)));
				target |= 1i64 << pow;
				wiring[pow] = 1;
			}

			wiring_bits.push_back(target);
			wirings.push_back(wiring);
		}

		auto num1 = bfs1(0i64, target, wiring_bits);
		if (num1 == -1)
		{
			std::cout << "No solution found for: " << line << "\n";
			return 1;
		}
		result1 += num1;

		std::vector<int16_t> target_counters;
		auto counters = words[words.size() - 1].substr(1, words[words.size() - 1].size() - 2);

		for (const auto& c : std::views::split(counters, ','))
		{
			auto counter = std::stoi(std::string(std::string_view(c)));
			if (counter > max_counter)
			{
				max_counter = counter;
			}
			target_counters.push_back(counter);
		}

		if (target_counters.size() > max_counter_num)
		{
			max_counter_num = target_counters.size();
		}

		std::sort(wirings.begin(), wirings.end(), [](const std::vector<int16_t>& a, const std::vector<int16_t>& b) {
			int64_t sum_a = 0;
			int64_t sum_b = 0;
			for (auto v : a) sum_a += v;
			for (auto v : b) sum_b += v;
			return sum_a > sum_b;
			});

		std::vector<int16_t> start_counters(target_counters.size(), 0i16);
		std::unordered_map<std::string, int64_t> memo;
		auto num2 = min_num_presses(start_counters, target_counters, wirings, memo);
		if (num2 == -1)
		{
			std::cout << "No solution found for: " << line << "\n";
			return 1;
		}
		result2 += num2;

		std::cout << "Machine: " << num2 << "\n";
	}

	std::cout << "Max counter:        " << max_counter << "\n";
	std::cout << "Max counter number: " << max_counter_num << "\n";

	std::cout << "Result 1: " << result1 << "\n";
	std::cout << "Result 2: " << result2 << "\n";

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
