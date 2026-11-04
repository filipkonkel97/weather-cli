#include <iostream>
#include <string>

std::string get_city(int argc, char *argv[]);
bool has_valid_arguments(int argc, char *argv[]);

int main(int argc, char *argv[])
{
    std::string city;
    bool check_validity;
    check_validity = has_valid_arguments(argc, argv);
    if (check_validity == true)
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
    std::string city;
    if (argc >= 3 && std::string(argv[1]) == "--city")
    {
        city = argv[2];
        return city;
    }
    else
    {
        return "";
    }
}

bool has_valid_arguments(int argc, char *argv[])
{
    if (argc >= 3 && std::string(argv[1]) == "--city" && !std::string(argv[2]).empty())
    {
        return true;
    }
    else
    {
        return false;
    }
}