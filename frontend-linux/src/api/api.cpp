api::api(std::string server_ip) : _server_ip(server_ip) {
    std::cout << "API ON" << std::endl;
    std::flush;
}

api::~api() = default;