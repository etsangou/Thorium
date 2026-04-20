// app.hpp
#pragma once

#include <iostream>
#include <string>
#include <vector>

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

    struct contact_arrage {
        std::string username;
        bool status; // Connecté en peer to peer ?
        int user_id;
    };

    app(application_specification app_spec);
    ~app();
    void start();

private:
    GLFWwindow* _window = nullptr;
    application_specification _app_spec;
    network_request web_r;

    int _actual_user_conv_index = 0;

    char _message_buffer[1024] = "";

    void switch_bg_color(ImVec4 color);
    void setup_imgui();

    void run_frame();
    void new_frame();
    void update_window();

    void message_struct(message_arrage message);
    void contact_struct(contact_arrage contact);
    void switch_conv(const int& user_id);

    //* TODO: convertir en float
    int LOGO_ZONE_X_SIZE = 250,
    LOGO_ZONE_Y_SIZE = 50,

    PROFIL_ZONE_X_SIZE = LOGO_ZONE_X_SIZE,
    PROFIL_ZONE_Y_SIZE = 50,

    CONTACT_ZONE_X_SIZE = LOGO_ZONE_X_SIZE,
    CONTACT_ZONE_Y_SIZE = _app_spec.windows_spec.height - LOGO_ZONE_Y_SIZE - PROFIL_ZONE_Y_SIZE,

    INFO_ZONE_X_SIZE = _app_spec.windows_spec.width - LOGO_ZONE_X_SIZE,
    INFO_ZONE_Y_SIZE = LOGO_ZONE_Y_SIZE,

    INPUT_ZONE_Y_SIZE = 60,

    CHAT_ZONE_X_SIZE = INFO_ZONE_X_SIZE,
    CHAT_ZONE_Y_SIZE = _app_spec.windows_spec.height - LOGO_ZONE_Y_SIZE - INPUT_ZONE_Y_SIZE;
};