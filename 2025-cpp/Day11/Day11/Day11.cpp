// Day11.cpp : This file contains the 'main' function. Program execution begins and ends there.
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
#include <unordered_map>
#include <functional>

size_t do_dfs(std::unordered_map<std::string, std::vector<std::string>>& adj, std::string start, std::string end)
{
	std::unordered_map<std::string, bool> visited;
	std::unordered_map<std::string, size_t> cache;
	std::function<size_t(const std::string&, std::vector<std::string>&)> dfs =
		[&](const std::string& node, std::vector<std::string>& path) -> size_t
		{
			size_t result = 0;
			if (cache.find(node) != cache.end())
			{
				result = cache[node];
			}
			else
			{
				if (node == end)
				{
					result = 1;
				}
				else
				{
					path.push_back(node);
					visited[node] = true;
					for (const auto& neighbor : adj[node])
					{
						if (neighbor == start || visited[neighbor])
						{
							continue;
						}
						result += dfs(neighbor, path);
					}
					visited[node] = false;
					path.pop_back();
				}
				cache[node] = result;
			}
			return result;
		};

	std::vector<std::string> path;
	return dfs(start, path);;
}

int main()
{
	// read all text from standard input
	std::string line;
	std::unordered_map<std::string, std::vector<std::string>> adj;

	while (std::getline(std::cin, line))
	{
		if (line.empty())
		{
			break;
		}

		std::vector<std::string> words;
		for (const auto& b : std::views::split(std::string_view(line), ' '))
		{
			words.push_back(std::string(std::string_view(b)));
		}

		std::string key = words[0].substr(0, words[0].length() - 1);
		std::erase(words, words[0]);
		adj[key] = words;
	}

	// find paths from "svr" to "out" that contain "fft" and "dac"
	size_t svr_fft = do_dfs(adj, "svr", "fft");
	size_t fft_dac = do_dfs(adj, "fft", "dac");
	size_t dac_out = do_dfs(adj, "dac", "out");

	size_t svr_dac = do_dfs(adj, "svr", "dac");
	size_t dac_fft = do_dfs(adj, "dac", "fft");
	size_t fft_out = do_dfs(adj, "dac", "out");

	size_t total_paths = svr_fft * fft_dac * dac_out + svr_dac * dac_fft * fft_out;
	std::cout << "Total paths from 'svr' to 'out' that contain 'fft' and 'dac': " << total_paths << std::endl;

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
