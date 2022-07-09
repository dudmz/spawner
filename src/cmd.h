#include <string>
#include <vector>

#ifndef HEADER_H
#define HEADER_H
using namespace std;

class InputParser
{
	private:
		string         mode;
		string         action;
		string         seed;
		vector<string> crawlers;

	public:
		InputParser(string mode, string action, string seed, vector<string>crawlers);
		bool is_valid();
};
#endif
