#pragma once

#include <iostream>
#include <string>
#include "../http/httplib.h"

class api {
private:
    std::string _server_ip;
public:
    api(std::string server_ip);
    ~api();
};