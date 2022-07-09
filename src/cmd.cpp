#include <string>
#include <vector>

#include "cmd.h"

using namespace std;

bool InputParser::is_valid()
{
	return true;
}

// spawner main crawl --seed https://damazio.dev --crawlers bot.damazio.dev:10355,...
InputParser parse(int argc, char **argv)
{
	string mode, action, seed = "";
	vector<string> crawlers;

	mode   = argv[1];
	action = argv[2];
	for (int i = 3; i < argc; i += 2) {
		if (string(argv[i]).compare("--seed") == 0 && string(argv[i+1]).rfind("--", 0) == 0) {
			seed = argv[i+1];
		} else if (string(argv[i]).compare("--crawlers") == 0 && string(argv[i+1]).rfind("--", 0) == 0) {
				
		}
	}

	return InputParser(mode, action, seed, crawlers);
}
