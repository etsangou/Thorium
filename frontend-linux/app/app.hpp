// app.hpp
#pragma once

#include <iostream>
#include <string>

#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include "../src/imgui/imgui.h"
#include "imgui_impl_glfw.h"
#include "imgui_impl_opengl3.h"
#include "../src/http/network_request.hpp"


class app {
    friend void window_refresh_callback(GLFWwindow* window);

public:
    struct application_specification {
        std::string name;
        struct {
            int width;
            int height;
        } windows_spec;
    };

    struct message_arrage {
        std::string username;
        std::string msg;
        std::string date;
        std::string hour;
    };

    app(application_specification app_spec);
    ~app();
    void start();

private:
    GLFWwindow* _window = nullptr;
    application_specification _app_spec;
    network_request web_r;

    void switch_bg_color(ImVec4 color);
    void setup_imgui();

    void run_frame();
    void new_frame();
    void update_window();

    void message_struct(message_arrage message);
};