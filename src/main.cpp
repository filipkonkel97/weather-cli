#include <iostream>
#include <string>

enum class Command
{
    Help,
    City,
    Invalid
};

std::string get_city(char *argv[]);
Command parse_arguments(int argc, char *argv[]);

int main(int argc, char *argv[])
{
    std::string city;
    Command command = parse_arguments(argc, argv);

    if (command == Command::City)
    {
        city = get_city(argv);
        std::cout << "Fetching weather for: " << city << '\n';
    }
    else if (command == Command::Help)
    {
        std::cout << "Using weather --help\n";
    }
    else
    {
        std::cout << "Usage: weather --city <city_name>\n";
    }
}

std::string get_city(char *argv[])
{
    return argv[2];
}

Command parse_arguments(int argc, char *argv[])
{
    if (argc >= 3 && std::string(argv[1]) == "--city" && !std::string(argv[2]).empty())
    {
        return Command::City;
    }
    else if (argc >= 2 && std::string(argv[1]) == "--help")
    {
        return Command::Help;
    }
    else
    {
        return Command::Invalid;
    }
}
