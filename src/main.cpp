#include <iostream>
#include <string>

enum class Command
{
    Help,
    City,
    Invalid
};

struct Arguments
{
    Command command = Command::Invalid;
    std::string city;
    std::string units = "metric";
};

Arguments parse_arguments(int argc, char *argv[]);

int main(int argc, char *argv[])
{
    std::string city;
    Arguments args = parse_arguments(argc, argv);

    if (args.command == Command::City)
    {
        city = args.city;
        std::cout << "Fetching weather for: " << city << '\n';
        std::cout << "Units: " << args.units << '\n';
    }
    else if (args.command == Command::Help)
    {
        std::cout << "Using weather --help\n";
    }
    else
    {
        std::cout << "Usage: weather --city <city_name>\n";
    }
}

Arguments parse_arguments(int argc, char *argv[])
{
    Arguments args;

    for (int i = 1; i < argc; i++)
    {
        std::string arg = argv[i];

        if (arg == "--help")
        {
            args.command = Command::Help;
        }
        else if (arg == "--city" && i + 1 < argc)
        {
            args.city = argv[i + 1];
            args.command = Command::City;
            i++;
        }
        else if (arg == "--units" && i + 1 < argc)
        {
            args.units = argv[i + 1];
            i++;
        }
    }

    return args;
}
