// g++ -o my_app main.cpp -lpthread
#include "network_request.hpp"

network_request::network_request() = default;
network_request::~network_request() = default;

int network_request::get(std::string url) {
    httplib::Client cli(url);
    
    if (auto res = cli.Get("/enzo")) {
        if (res->status == 200) {
            _response = res->body;
            //std::cout << "Response: " << _response << std::endl;
            return 0;
        }
        return 1;
    } else {
        auto err = res.error();
        std::cerr << "HTTP Error: " << httplib::to_string(err) << std::endl;
        return 1;
    }
}

std::string network_request::get_web_answer() {
    return _response;
}