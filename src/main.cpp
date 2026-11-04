#include <iostream>
#include <string>

std::string get_city(int argc, char *argv[]);
bool has_valid_arguments(int argc, char *argv[]);

int main(int argc, char *argv[])
{
    std::string city;
    if (has_valid_arguments(argc, argv))
    {
        city = get_city(argc, argv);
        std::cout << "Fetching weather for: " << city << '\n';
    }
    else
    {
        std::cout << "Usage: weather --city <city_name>\n";
    }
}

std::string get_city(int argc, char *argv[])
{
    return argv[2];
}

bool has_valid_arguments(int argc, char *argv[])
{
    return argc >= 3 && std::string(argv[1]) == "--city" && !std::string(argv[2]).empty();
}
