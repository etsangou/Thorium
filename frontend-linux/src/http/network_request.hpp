#pragma once

#include "httplib.h"
#include <iostream>
#include <string>

class network_request {
public:
    int get(std::string url);
    network_request(/* args */);
    ~network_request();
    std::string get_web_answer();

private:
    std::string _response;
};
