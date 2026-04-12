// g++ -o my_app main.cpp -lpthread
#include "network_request.hpp"

network_request() = default;
~network_request() = default;

static int get(std::string url) {
    httplib::Client cli(url);
    
    if (auto res = cli.Get("/")) {
        if (res->status == 200) {
            std::cout << "Response: " << res->body << std::endl;
            return 0;
        }
        return 1;
    } else {
        auto err = res.error();
        std::cerr << "HTTP Error: " << httplib::to_string(err) << std::endl;
        return 1;
    }
}
